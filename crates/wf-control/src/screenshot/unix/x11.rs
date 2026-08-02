use std::num::NonZeroI32;
use std::time::Instant;

use x11rb::connection::Connection;
use x11rb::properties::WmClass;
use x11rb::protocol::xproto::{
    Atom, AtomEnum, ConnectionExt, GetImageReply, ImageFormat as XImageFormat, ImageOrder,
    VisualClass, Visualtype, Window,
};
use x11rb::rust_connection::RustConnection;

use super::common::{BmpError, BmpRgb24, WARFRAME_CLASS_HINTS, WARFRAME_TITLE_HINTS};

#[derive(Debug, thiserror::Error)]
pub(crate) enum X11Error {
    #[error("No X11/XWayland window found for Warframe; PID and title/class lookup both failed")]
    WindowNotFound,
    #[error("Invalid X11 window id '{0}'")]
    InvalidWindowId(String),
    #[error("Failed to connect to the X11/XWayland server")]
    Connect(#[source] x11rb::errors::ConnectError),
    #[error("X11 window {0} has empty geometry")]
    EmptyGeometry(String),
    #[error("Failed to read X11 geometry for window {window_id}")]
    Geometry {
        window_id: String,
        #[source]
        source: x11rb::errors::ReplyError,
    },
    #[error("Failed to read X11 image for window {window_id}")]
    Image {
        window_id: String,
        #[source]
        source: x11rb::errors::ReplyError,
    },
    #[error("X11 property returned invalid UTF-8")]
    InvalidUtf8(#[source] std::string::FromUtf8Error),
    #[error("Could not find X11 visual {0}")]
    VisualNotFound(u32),
    #[error("Unsupported X11 visual class {0:?}")]
    UnsupportedVisualClass(VisualClass),
    #[error("No X11 pixmap format for depth {0}")]
    NoPixmapFormat(u8),
    #[error("Unsupported X11 bits-per-pixel {0}")]
    UnsupportedBitsPerPixel(u8),
    #[error("Invalid X11 image height")]
    InvalidImageHeight,
    #[error("X11 image has invalid width or height")]
    InvalidImageDimension,
    #[error("X11 image data ended unexpectedly")]
    ImageDataTruncated,
    #[error("Unsupported X11 image byte order {0:?}")]
    UnsupportedByteOrder(ImageOrder),
    #[error(transparent)]
    Connection(#[from] x11rb::errors::ConnectionError),
    #[error(transparent)]
    Reply(#[from] x11rb::errors::ReplyError),
    #[error(transparent)]
    Bmp(#[from] BmpError),
}

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

pub(super) fn find_window(pid: u32) -> Result<String, X11Error> {
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

    Err(X11Error::WindowNotFound)
}

pub(super) fn capture_window(window_id: &str) -> Result<Vec<u8>, X11Error> {
    let total_start = Instant::now();
    let window = window_id
        .parse::<Window>()
        .map_err(|_| X11Error::InvalidWindowId(window_id.to_string()))?;
    let connect_start = Instant::now();
    let context = X11Context::connect()?;
    log::trace!(
        "Screenshot X11 connection initialized in {:?}",
        connect_start.elapsed()
    );
    let geometry_start = Instant::now();
    let geometry = context
        .conn
        .get_geometry(window)?
        .reply()
        .map_err(|source| X11Error::Geometry {
            window_id: window_id.to_string(),
            source,
        })?;
    log::trace!(
        "Screenshot X11 geometry {}x{} fetched in {:?}",
        geometry.width,
        geometry.height,
        geometry_start.elapsed()
    );

    if geometry.width == 0 || geometry.height == 0 {
        return Err(X11Error::EmptyGeometry(window_id.to_string()));
    }

    let get_image_start = Instant::now();
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
        )?
        .reply()
        .map_err(|source| X11Error::Image {
            window_id: window_id.to_string(),
            source,
        })?;
    log::trace!(
        "Screenshot X11 GetImage returned {} bytes in {:?}",
        image.data.len(),
        get_image_start.elapsed()
    );

    let encode_start = Instant::now();
    let visual = context.visual_for_window(window)?;
    let bytes = encode_x11_image_bmp(
        &context.conn,
        &image,
        &visual,
        geometry.width,
        geometry.height,
    )?;
    log::trace!(
        "Screenshot X11 BMP encode produced {} bytes in {:?}",
        bytes.len(),
        encode_start.elapsed()
    );
    log::trace!(
        "Screenshot X11 capture_window completed in {:?}",
        total_start.elapsed()
    );
    Ok(bytes)
}

impl X11Context {
    fn connect() -> Result<Self, X11Error> {
        let (conn, screen_num) = x11rb::connect(None).map_err(X11Error::Connect)?;
        let atoms = Atoms::intern(&conn)?;
        Ok(Self {
            conn,
            screen_num,
            atoms,
        })
    }

    fn windows(&self) -> Result<Vec<Window>, X11Error> {
        let root = self.conn.setup().roots[self.screen_num].root;
        let mut windows = Vec::new();
        self.collect_windows(root, &mut windows)?;
        Ok(windows)
    }

    fn collect_windows(&self, window: Window, windows: &mut Vec<Window>) -> Result<(), X11Error> {
        let Ok(tree) = self.conn.query_tree(window)?.reply() else {
            return Ok(());
        };

        for child in tree.children {
            windows.push(child);
            self.collect_windows(child, windows)?;
        }

        Ok(())
    }

    fn window_pid(&self, window: Window) -> Result<Option<u32>, X11Error> {
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
            .is_ok_and(|title| WARFRAME_TITLE_HINTS.iter().any(|hint| title.contains(hint)))
            || self
                .window_class(window)
                .is_ok_and(|classes| {
                    classes.iter().any(|class| {
                        WARFRAME_CLASS_HINTS
                            .iter()
                            .any(|hint| class.eq_ignore_ascii_case(hint))
                    })
                })
    }

    fn window_title(&self, window: Window) -> Result<String, X11Error> {
        self.property_string(window, self.atoms.net_wm_name, self.atoms.utf8_string)
            .or_else(|_| self.property_string(window, AtomEnum::WM_NAME.into(), AtomEnum::STRING))
    }

    fn property_string<T>(&self, window: Window, property: Atom, type_: T) -> Result<String, X11Error>
    where
        T: Into<Atom>,
    {
        let reply = self
            .conn
            .get_property(false, window, property, type_, 0, 1024)?
            .reply()?;
        String::from_utf8(reply.value).map_err(X11Error::InvalidUtf8)
    }

    fn window_class(&self, window: Window) -> Result<Vec<String>, X11Error> {
        let Some(wm_class) = WmClass::get(&self.conn, window)?.reply()? else {
            return Ok(Vec::new());
        };

        Ok([wm_class.instance(), wm_class.class()]
            .into_iter()
            .filter_map(|value| std::str::from_utf8(value).ok())
            .map(str::to_string)
            .collect())
    }

    fn visual_for_window(&self, window: Window) -> Result<Visualtype, X11Error> {
        let attributes = self.conn.get_window_attributes(window)?.reply()?;
        self.conn.setup().roots[self.screen_num]
            .allowed_depths
            .iter()
            .flat_map(|depth| depth.visuals.iter())
            .find(|visual| visual.visual_id == attributes.visual)
            .cloned()
            .ok_or(X11Error::VisualNotFound(attributes.visual))
    }
}

impl Atoms {
    fn intern(conn: &RustConnection) -> Result<Self, X11Error> {
        Ok(Self {
            net_wm_name: intern_atom(conn, "_NET_WM_NAME")?,
            net_wm_pid: intern_atom(conn, "_NET_WM_PID")?,
            utf8_string: intern_atom(conn, "UTF8_STRING")?,
        })
    }
}

fn intern_atom(conn: &RustConnection, name: &str) -> Result<Atom, X11Error> {
    Ok(conn.intern_atom(false, name.as_bytes())?.reply()?.atom)
}

fn encode_x11_image_bmp(
    conn: &RustConnection,
    image: &GetImageReply,
    visual: &Visualtype,
    width: u16,
    height: u16,
) -> Result<Vec<u8>, X11Error> {
    if visual.class != VisualClass::TRUE_COLOR && visual.class != VisualClass::DIRECT_COLOR {
        return Err(X11Error::UnsupportedVisualClass(visual.class));
    }

    let format = conn
        .setup()
        .pixmap_formats
        .iter()
        .find(|format| format.depth == image.depth)
        .ok_or(X11Error::NoPixmapFormat(image.depth))?;
    let bytes_per_pixel = usize::from(format.bits_per_pixel / 8);
    if bytes_per_pixel == 0 {
        return Err(X11Error::UnsupportedBitsPerPixel(format.bits_per_pixel));
    }

    let width_usize = usize::from(width);
    let height_usize = usize::from(height);
    let stride = image
        .data
        .len()
        .checked_div(height_usize)
        .ok_or(X11Error::InvalidImageHeight)?;
    let mut bmp = BmpRgb24::new(
        NonZeroI32::new(i32::from(width)).ok_or(X11Error::InvalidImageDimension)?,
        NonZeroI32::new(i32::from(height)).ok_or(X11Error::InvalidImageDimension)?,
    )?;

    if bytes_per_pixel == 4
        && conn.setup().image_byte_order == ImageOrder::LSB_FIRST
        && visual.red_mask == 0x00ff_0000
        && visual.green_mask == 0x0000_ff00
        && visual.blue_mask == 0x0000_00ff
    {
        for y in 0..height_usize {
            let source_start = y * stride;
            let source_end = source_start + width_usize * bytes_per_pixel;
            let source_row = image
                .data
                .get(source_start..source_end)
                .ok_or(X11Error::ImageDataTruncated)?;
            bmp.copy_bgrx_row(y, source_row);
        }
        return Ok(bmp.into_bytes());
    }

    for y in 0..height_usize {
        for x in 0..width_usize {
            let offset = y * stride + x * bytes_per_pixel;
            let pixel = read_pixel(
                &image.data,
                offset,
                bytes_per_pixel,
                conn.setup().image_byte_order,
            )?;
            bmp.set_pixel_bgr(
                x,
                y,
                [
                    component(pixel, visual.blue_mask),
                    component(pixel, visual.green_mask),
                    component(pixel, visual.red_mask),
                ],
            );
        }
    }

    Ok(bmp.into_bytes())
}

fn read_pixel(
    data: &[u8],
    offset: usize,
    bytes_per_pixel: usize,
    image_byte_order: ImageOrder,
) -> Result<u32, X11Error> {
    let bytes = data
        .get(offset..offset + bytes_per_pixel)
        .ok_or(X11Error::ImageDataTruncated)?;
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
        _ => return Err(X11Error::UnsupportedByteOrder(image_byte_order)),
    }
    Ok(pixel)
}

fn component(pixel: u32, mask: u32) -> u8 {
    if mask == 0 {
        return 0;
    }

    let shift = mask.trailing_zeros();
    let max = u64::from(mask >> shift);
    let value = u64::from((pixel & mask) >> shift);
    // value <= max, so the rounded scale is always <= 255.
    u8::try_from((value * 255 + max / 2) / max).unwrap_or(u8::MAX)
}
