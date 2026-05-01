use anyhow::{Context, Result, anyhow, bail};
use image::ExtendedColorType;
use image::codecs::bmp::BmpEncoder;
use x11rb::connection::Connection;
use x11rb::properties::WmClass;
use x11rb::protocol::xproto::{
    Atom, AtomEnum, ConnectionExt, GetImageReply, ImageFormat as XImageFormat, ImageOrder,
    VisualClass, Visualtype, Window,
};
use x11rb::rust_connection::RustConnection;

use super::common::{WARFRAME_CLASS_HINTS, WARFRAME_TITLE_HINTS, ensure_bmp_bytes};

struct X11Context {
    conn: RustConnection,
    screen_num: usize,
    atoms: Atoms,
}

struct Atoms {
    net_wm_name: Atom,
    net_wm_pid: Atom,
    utf8_string: Atom,
}

pub(super) fn find_window(pid: u32) -> Result<String> {
    let context = X11Context::connect()?;

    let mut heuristic_match = None;
    for window in context.windows()? {
        if context.window_pid(window).ok().flatten() == Some(pid) {
            return Ok(window.to_string());
        }

        if heuristic_match.is_none() && context.matches_warframe_hints(window) {
            heuristic_match = Some(window);
        }
    }

    if let Some(window) = heuristic_match {
        return Ok(window.to_string());
    }

    bail!("No X11/XWayland window found for Warframe; PID and title/class lookup both failed");
}

pub(super) fn capture_window(window_id: &str) -> Result<Vec<u8>> {
    let window = window_id
        .parse::<Window>()
        .with_context(|| format!("Invalid X11 window id '{}'", window_id))?;
    let context = X11Context::connect()?;
    let geometry = context
        .conn
        .get_geometry(window)
        .with_context(|| format!("Failed to request X11 geometry for window {}", window_id))?
        .reply()
        .with_context(|| format!("Failed to read X11 geometry for window {}", window_id))?;

    if geometry.width == 0 || geometry.height == 0 {
        bail!("X11 window {} has empty geometry", window_id);
    }

    let image = context
        .conn
        .get_image(
            XImageFormat::Z_PIXMAP,
            window,
            0,
            0,
            geometry.width,
            geometry.height,
            u32::MAX,
        )
        .with_context(|| format!("Failed to request X11 image for window {}", window_id))?
        .reply()
        .with_context(|| format!("Failed to read X11 image for window {}", window_id))?;

    let visual = context.visual_for_window(window)?;
    let bytes = encode_x11_image_bmp(
        &context.conn,
        &image,
        &visual,
        geometry.width,
        geometry.height,
    )
    .with_context(|| format!("Failed to encode X11 window {} capture as BMP", window_id))?;
    ensure_bmp_bytes(&bytes, "X11 window capture")?;
    Ok(bytes)
}

impl X11Context {
    fn connect() -> Result<Self> {
        let (conn, screen_num) =
            x11rb::connect(None).context("Failed to connect to the X11/XWayland server")?;
        let atoms = Atoms::intern(&conn)?;
        Ok(Self {
            conn,
            screen_num,
            atoms,
        })
    }

    fn windows(&self) -> Result<Vec<Window>> {
        let root = self.conn.setup().roots[self.screen_num].root;
        let mut windows = Vec::new();
        self.collect_windows(root, &mut windows)?;
        Ok(windows)
    }

    fn collect_windows(&self, window: Window, windows: &mut Vec<Window>) -> Result<()> {
        let tree = match self.conn.query_tree(window)?.reply() {
            Ok(tree) => tree,
            Err(_) => return Ok(()),
        };

        for child in tree.children {
            windows.push(child);
            self.collect_windows(child, windows)?;
        }

        Ok(())
    }

    fn window_pid(&self, window: Window) -> Result<Option<u32>> {
        let reply = self
            .conn
            .get_property(
                false,
                window,
                self.atoms.net_wm_pid,
                AtomEnum::CARDINAL,
                0,
                1,
            )?
            .reply()?;

        Ok(reply.value32().and_then(|mut values| values.next()))
    }

    fn matches_warframe_hints(&self, window: Window) -> bool {
        self.window_title(window)
            .map(|title| WARFRAME_TITLE_HINTS.iter().any(|hint| title.contains(hint)))
            .unwrap_or(false)
            || self
                .window_class(window)
                .map(|classes| {
                    classes.iter().any(|class| {
                        WARFRAME_CLASS_HINTS
                            .iter()
                            .any(|hint| class.eq_ignore_ascii_case(hint))
                    })
                })
                .unwrap_or(false)
    }

    fn window_title(&self, window: Window) -> Result<String> {
        self.property_string(window, self.atoms.net_wm_name, self.atoms.utf8_string)
            .or_else(|_| self.property_string(window, AtomEnum::WM_NAME.into(), AtomEnum::STRING))
    }

    fn property_string<T>(&self, window: Window, property: Atom, type_: T) -> Result<String>
    where
        T: Into<Atom>,
    {
        let reply = self
            .conn
            .get_property(false, window, property, type_, 0, 1024)?
            .reply()?;
        String::from_utf8(reply.value).context("X11 property returned invalid UTF-8")
    }

    fn window_class(&self, window: Window) -> Result<Vec<String>> {
        let Some(wm_class) = WmClass::get(&self.conn, window)?.reply()? else {
            return Ok(Vec::new());
        };

        Ok([wm_class.instance(), wm_class.class()]
            .into_iter()
            .filter_map(|value| std::str::from_utf8(value).ok())
            .map(str::to_string)
            .collect())
    }

    fn visual_for_window(&self, window: Window) -> Result<Visualtype> {
        let attributes = self.conn.get_window_attributes(window)?.reply()?;
        self.conn.setup().roots[self.screen_num]
            .allowed_depths
            .iter()
            .flat_map(|depth| depth.visuals.iter())
            .find(|visual| visual.visual_id == attributes.visual)
            .cloned()
            .ok_or_else(|| anyhow!("Could not find X11 visual {}", attributes.visual))
    }
}

impl Atoms {
    fn intern(conn: &RustConnection) -> Result<Self> {
        Ok(Self {
            net_wm_name: intern_atom(conn, "_NET_WM_NAME")?,
            net_wm_pid: intern_atom(conn, "_NET_WM_PID")?,
            utf8_string: intern_atom(conn, "UTF8_STRING")?,
        })
    }
}

fn intern_atom(conn: &RustConnection, name: &str) -> Result<Atom> {
    Ok(conn.intern_atom(false, name.as_bytes())?.reply()?.atom)
}

fn encode_x11_image_bmp(
    conn: &RustConnection,
    image: &GetImageReply,
    visual: &Visualtype,
    width: u16,
    height: u16,
) -> Result<Vec<u8>> {
    if visual.class != VisualClass::TRUE_COLOR && visual.class != VisualClass::DIRECT_COLOR {
        bail!("Unsupported X11 visual class {:?}", visual.class);
    }

    let format = conn
        .setup()
        .pixmap_formats
        .iter()
        .find(|format| format.depth == image.depth)
        .ok_or_else(|| anyhow!("No X11 pixmap format for depth {}", image.depth))?;
    let bytes_per_pixel = usize::from(format.bits_per_pixel / 8);
    if bytes_per_pixel == 0 {
        bail!("Unsupported X11 bits-per-pixel {}", format.bits_per_pixel);
    }

    let width_usize = usize::from(width);
    let height_usize = usize::from(height);
    let stride = image
        .data
        .len()
        .checked_div(height_usize)
        .ok_or_else(|| anyhow!("Invalid X11 image height"))?;
    let mut rgb = vec![0; width_usize * height_usize * 3];

    for y in 0..height_usize {
        for x in 0..width_usize {
            let offset = y * stride + x * bytes_per_pixel;
            let pixel = read_pixel(
                &image.data,
                offset,
                bytes_per_pixel,
                conn.setup().image_byte_order,
            )?;
            let rgb_offset = (y * width_usize + x) * 3;
            rgb[rgb_offset] = component(pixel, visual.red_mask);
            rgb[rgb_offset + 1] = component(pixel, visual.green_mask);
            rgb[rgb_offset + 2] = component(pixel, visual.blue_mask);
        }
    }

    let mut bytes = Vec::new();
    BmpEncoder::new(&mut bytes).encode(
        &rgb,
        u32::from(width),
        u32::from(height),
        ExtendedColorType::Rgb8,
    )?;
    Ok(bytes)
}

fn read_pixel(
    data: &[u8],
    offset: usize,
    bytes_per_pixel: usize,
    image_byte_order: ImageOrder,
) -> Result<u32> {
    let bytes = data
        .get(offset..offset + bytes_per_pixel)
        .ok_or_else(|| anyhow!("X11 image data ended unexpectedly"))?;
    let mut pixel = 0u32;
    match image_byte_order {
        ImageOrder::LSB_FIRST => {
            for (shift, byte) in bytes.iter().enumerate() {
                pixel |= u32::from(*byte) << (shift * 8);
            }
        }
        ImageOrder::MSB_FIRST => {
            for byte in bytes {
                pixel = (pixel << 8) | u32::from(*byte);
            }
        }
        _ => bail!("Unsupported X11 image byte order {:?}", image_byte_order),
    }
    Ok(pixel)
}

fn component(pixel: u32, mask: u32) -> u8 {
    if mask == 0 {
        return 0;
    }

    let shift = mask.trailing_zeros();
    let max = mask >> shift;
    let value = (pixel & mask) >> shift;
    ((value * 255 + max / 2) / max) as u8
}
