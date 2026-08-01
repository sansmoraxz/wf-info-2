use std::sync::Arc;

use tokio::sync::broadcast;

use crate::events::DaemonEvent;
use crate::market::WfmCache;
use crate::screenshot::ScreenshotConfig;
use crate::search::CachedInventoryIndex;
use crate::wfm_auth::WfmState;

const CHANNEL_CAPACITY: usize = 256;

/// Owned runtime state for the daemon, created once in `main` and shared via
/// [`Arc`] across the server, watcher, and websocket tasks.
pub struct AppState {
    broadcaster: broadcast::Sender<DaemonEvent>,
    pub(crate) wfm: tokio::sync::RwLock<WfmState>,
    pub(crate) market_cache: std::sync::RwLock<Option<WfmCache>>,
    pub(crate) inventory_index: std::sync::RwLock<Option<CachedInventoryIndex>>,
    pub(crate) screenshot: ScreenshotState,
}

pub(crate) struct ScreenshotState {
    pub config: ScreenshotConfig,
    #[cfg(unix)]
    pub backend_cache: std::sync::Mutex<Option<crate::screenshot::BackendCacheEntry>>,
    #[cfg(windows)]
    pub window_cache: std::sync::Mutex<Option<crate::screenshot::WindowCacheEntry>>,
    #[cfg(any(unix, windows))]
    pub warframe_pid: std::sync::Mutex<Option<u32>>,
}

impl Default for AppState {
    fn default() -> Self {
        let (broadcaster, _) = broadcast::channel(CHANNEL_CAPACITY);
        Self {
            broadcaster,
            wfm: tokio::sync::RwLock::new(WfmState::default()),
            market_cache: std::sync::RwLock::new(None),
            inventory_index: std::sync::RwLock::new(None),
            screenshot: ScreenshotState {
                config: ScreenshotConfig::default(),
                #[cfg(unix)]
                backend_cache: std::sync::Mutex::new(None),
                #[cfg(windows)]
                window_cache: std::sync::Mutex::new(None),
                #[cfg(any(unix, windows))]
                warframe_pid: std::sync::Mutex::new(None),
            },
        }
    }
}

impl AppState {
    pub fn new(screenshot_config: ScreenshotConfig) -> Arc<Self> {
        let mut state = Self::default();
        state.screenshot.config = screenshot_config;
        Arc::new(state)
    }

    /// Broadcast an event to all subscribers. A send error only means there
    /// are currently no subscribers, which is fine.
    pub fn emit(&self, event: DaemonEvent) {
        let _ = self.broadcaster.send(event);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<DaemonEvent> {
        self.broadcaster.subscribe()
    }
}
