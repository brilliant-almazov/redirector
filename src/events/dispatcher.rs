use super::publisher::EventSender;
use super::RedirectEvent;

/// Central event dispatcher.
///
/// Handlers fire events through the dispatcher without knowing
/// about queue backends or transport details.
/// Modelled after Symfony EventDispatcher — dispatch and forget.
#[derive(Clone)]
pub struct EventDispatcher {
    sender: Option<EventSender>,
}

impl EventDispatcher {
    /// Create an active dispatcher backed by an EventSender.
    pub fn new(sender: EventSender) -> Self {
        Self {
            sender: Some(sender),
        }
    }

    /// Create a no-op dispatcher (events are silently discarded).
    pub fn noop() -> Self {
        Self { sender: None }
    }

    /// Dispatch a redirect event (non-blocking, fire-and-forget).
    pub fn dispatch_redirect(&self, event: RedirectEvent) {
        if let Some(ref sender) = self.sender {
            sender.try_send(event);
        }
    }
}

#[cfg(test)]
#[path = "dispatcher_test.rs"]
mod dispatcher_test;
