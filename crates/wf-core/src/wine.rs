//! Discovery of the wine environment a running game process lives in, so a
//! helper Windows binary can be launched into the same prefix.

use std::ffi::OsString;
use std::fs;
use std::io;
use std::os::unix::ffi::OsStringExt as _;
use std::path::{Path, PathBuf};
use std::str;

/// Environment variables that select the wine prefix and its sync primitives.
/// Anything else from the game's environment is deliberately not replicated.
const KEPT_VARS: &[&str] = &["WINEPREFIX", "WINEESYNC", "WINEFSYNC"];
const KEPT_PREFIXES: &[&str] = &["PROTON_"];

#[derive(Debug, thiserror::Error)]
pub enum WineDiscoveryError {
    #[error("Failed to read {path}")]
    Proc {
        path: String,
        #[source]
        source: io::Error,
    },
    #[error("No wine binary found next to {preloader}")]
    WineBinaryNotFound { preloader: PathBuf },
}

/// The pieces needed to run a Windows binary inside the same wine prefix as
/// an already-running process.
#[derive(Debug, Clone)]
pub struct WineContext {
    pub wine_binary: PathBuf,
    pub env: Vec<(OsString, OsString)>,
}

impl WineContext {
    /// Inspects `/proc/<pid>` of a wine process to find its wine binary and
    /// prefix-selecting environment.
    pub fn for_pid(pid: u32) -> Result<Self, WineDiscoveryError> {
        let environ_path = format!("/proc/{pid}/environ");
        let environ =
            fs::read(&environ_path).map_err(|source| WineDiscoveryError::Proc {
                path: environ_path,
                source,
            })?;

        let exe_path = format!("/proc/{pid}/exe");
        let exe = fs::read_link(&exe_path).map_err(|source| WineDiscoveryError::Proc {
            path: exe_path,
            source,
        })?;

        Ok(Self {
            wine_binary: discover_wine_binary(pid, &exe)?,
            env: prefix_env_from_environ(&environ),
        })
    }
}

/// The game may run in Steam's pressure-vessel container, where its exe path
/// is namespace-relative. Try, in order: the path as-is on the host (plain
/// wine), the path with pressure-vessel's `/run/host` prefix mapped back to
/// the host root (running wine through `/proc/<pid>/root/...` breaks its
/// self-location and it fails to load `ntdll.so`), and finally through
/// `/proc/<pid>/root` as a last resort.
fn discover_wine_binary(pid: u32, exe: &Path) -> Result<PathBuf, WineDiscoveryError> {
    let candidates = [
        Some(exe.to_path_buf()),
        exe.strip_prefix("/run/host")
            .ok()
            .map(|rest| Path::new("/").join(rest)),
        exe.strip_prefix("/")
            .ok()
            .map(|rest| PathBuf::from(format!("/proc/{pid}/root")).join(rest)),
    ];

    candidates
        .into_iter()
        .flatten()
        .filter(|candidate| candidate.is_file())
        .find_map(|candidate| wine_binary_near(&candidate).ok())
        .ok_or_else(|| WineDiscoveryError::WineBinaryNotFound {
            preloader: exe.to_path_buf(),
        })
}

/// The game's `/proc/<pid>/exe` points at wine's preloader (or wine itself).
/// Proton keeps the preloader in `files/lib/wine/x86_64-unix/` with the real
/// `wine` entry point in `files/bin/`, so probe `<ancestor>/bin/` walking up
/// from the preloader; that directory is preferred over the preloader's own,
/// which on proton holds only the low-level unix loader (it exits without
/// `WINEDLLPATH` set up by the `bin/` entry point). Plain wine keeps `wine`
/// next to the preloader, which is itself a `bin/` directory. The walk stops
/// before the filesystem root so an unrelated system `/bin/wine` is never
/// picked up.
fn wine_binary_near(exe: &Path) -> Result<PathBuf, WineDiscoveryError> {
    const NAMES: [&str; 2] = ["wine", "wine64"];

    let dirs = exe
        .ancestors()
        .skip(1)
        .take_while(|ancestor| ancestor.parent().is_some_and(|p| p.parent().is_some()))
        .map(|ancestor| ancestor.join("bin"))
        .chain(exe.parent().map(Path::to_path_buf));

    dirs.flat_map(|dir| NAMES.iter().map(move |name| dir.join(name)))
        .find(|candidate| candidate.is_file())
        .ok_or_else(|| WineDiscoveryError::WineBinaryNotFound {
            preloader: exe.to_path_buf(),
        })
}

/// Extracts prefix-selecting variables from NUL-separated `environ` bytes.
fn prefix_env_from_environ(environ: &[u8]) -> Vec<(OsString, OsString)> {
    environ
        .split(|byte| *byte == 0)
        .filter(|entry| !entry.is_empty())
        .filter_map(|entry| {
            let eq = entry.iter().position(|byte| *byte == b'=')?;
            let (key, rest) = entry.split_at(eq);
            let key_str = str::from_utf8(key).ok()?;
            let kept = KEPT_VARS.contains(&key_str)
                || KEPT_PREFIXES
                    .iter()
                    .any(|prefix| key_str.starts_with(prefix));
            kept.then(|| {
                (
                    OsString::from_vec(key.to_vec()),
                    OsString::from_vec(rest.get(1..).unwrap_or_default().to_vec()),
                )
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{prefix_env_from_environ, wine_binary_near};
    use std::env;
    use std::ffi::OsString;
    use std::fs;
    use std::process;

    #[test]
    fn finds_wine_in_proton_bin_dir_from_unix_lib_preloader() {
        let root = env::temp_dir().join(format!("wf-wine-test-{}", process::id()));
        let preloader_dir = root.join("proton/files/lib/wine/x86_64-unix");
        let bin_dir = root.join("proton/files/bin");
        fs::create_dir_all(&preloader_dir).unwrap();
        fs::create_dir_all(&bin_dir).unwrap();
        fs::write(bin_dir.join("wine"), b"").unwrap();

        let found = wine_binary_near(&preloader_dir.join("wine64-preloader")).unwrap();
        assert_eq!(
            found,
            bin_dir.join("wine"),
            "proton layout: wine in files/bin"
        );

        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn keeps_prefix_selecting_vars_and_drops_the_rest() {
        let environ = b"HOME=/home/user\0WINEPREFIX=/games/pfx\0WINEESYNC=1\0\
                        PROTON_NO_FSYNC=1\0PATH=/usr/bin\0LANG=C\0";
        let env = prefix_env_from_environ(environ);

        let get = |key: &str| {
            env.iter()
                .find(|(k, _)| k == key)
                .map(|(_, v)| v.clone())
        };
        assert_eq!(
            get("WINEPREFIX"),
            Some(OsString::from("/games/pfx")),
            "WINEPREFIX must be kept"
        );
        assert_eq!(
            get("WINEESYNC"),
            Some(OsString::from("1")),
            "WINEESYNC must be kept"
        );
        assert_eq!(
            get("PROTON_NO_FSYNC"),
            Some(OsString::from("1")),
            "PROTON_* must be kept"
        );
        assert_eq!(env.len(), 3, "unrelated vars must be dropped");
    }

    #[test]
    fn tolerates_malformed_entries() {
        let environ = b"\0NOEQUALS\0WINEPREFIX=\0";
        let env = prefix_env_from_environ(environ);
        assert_eq!(
            env,
            vec![(OsString::from("WINEPREFIX"), OsString::new())],
            "empty value and malformed entries handled"
        );
    }
}
