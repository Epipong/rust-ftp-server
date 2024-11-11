use libunftp::notification::{EventMeta, PresenceEvent, PresenceListener};
use log::info;

#[derive(Debug)]
pub struct MyPresenceListener;

impl PresenceListener for MyPresenceListener {
    #[doc = " Called after the event happened. Event metadata is also passed to allow pinpointing the user"]
    #[doc = " session for which it happened."]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn receive_presence_event<'life0, 'async_trait>(
        &'life0 self,
        event: PresenceEvent,
        meta: EventMeta,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            match event {
                PresenceEvent::LoggedIn => {
                    info!(
                        "User '{}' logged in, trace-id: {}",
                        meta.username, meta.trace_id,
                    );
                }
                PresenceEvent::LoggedOut => {
                    info!(
                        "User '{}' logged out, trace-id: {}",
                        meta.username, meta.trace_id
                    );
                }
            }
        })
    }
}
