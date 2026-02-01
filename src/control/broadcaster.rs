use std::sync::OnceLock;

use tokio::sync::broadcast::{self, Receiver, Sender};

use super::events::DaemonEvent;

const CHANNEL_CAPACITY: usize = 256;

static BROADCASTER: OnceLock<Sender<DaemonEvent>> = OnceLock::new();

fn get_sender() -> &'static Sender<DaemonEvent> {
    BROADCASTER.get_or_init(|| {
        let (tx, _rx) = broadcast::channel(CHANNEL_CAPACITY);
        tx
    })
}

/// Emit an event to all subscribers.
/// Returns the number of receivers that received the event.
/// Returns 0 if there are no active subscribers (this is not an error).
pub fn emit(event: DaemonEvent) {
    let sender = get_sender();
    // send() returns Err if there are no receivers, which is fine
    let _ = sender.send(event);
}

/// Subscribe to daemon events.
/// Returns a receiver that will receive all emitted events.
pub fn subscribe() -> Receiver<DaemonEvent> {
    get_sender().subscribe()
}
