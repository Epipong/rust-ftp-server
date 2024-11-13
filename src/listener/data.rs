use libunftp::notification::{DataEvent, DataListener, EventMeta};
use log::{info, warn};

#[derive(Debug)]
pub struct MyDataListener {
    pub ftp_home: String,
}

impl DataListener for MyDataListener {
    #[doc = "Called after the event happened. Event metadata is also passed to allow pinpointing the user"]
    #[doc = "session for which it happened."]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn receive_data_event<'life0, 'async_trait>(
        &'life0 self,
        e: DataEvent,
        m: EventMeta,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            match e {
                DataEvent::Got { path, bytes } => {
                    info!(
                        "Download completed: User: {}, Path: {}, Bytes transferred: {}",
                        m.username,
                        format!("{}{path}", self.ftp_home),
                        bytes
                    );
                }
                DataEvent::Put { path, bytes } => {
                    info!(
                        "Upload completed: User: {}, Path: {}, Bytes stored: {}",
                        m.username,
                        format!("{}{path}", self.ftp_home),
                        bytes
                    );
                }
                DataEvent::Deleted { path } => {
                    warn!(
                        "File deleted: User: {}, Path: {}",
                        m.username,
                        format!("{}{path}", self.ftp_home)
                    );
                }
                DataEvent::MadeDir { path } => {
                    info!(
                        "Directory created: User: {}, Path: {}",
                        m.username,
                        format!("{}{path}", self.ftp_home)
                    );
                }
                DataEvent::RemovedDir { path } => {
                    warn!(
                        "Directory removed: User: {}, Path: {}",
                        m.username,
                        format!("{}{path}", self.ftp_home)
                    );
                }
                DataEvent::Renamed { from, to } => {
                    info!(
                        "File or directory renamed: User: {}, From: {}, To: {}",
                        m.username, from, to
                    );
                }
            }
        })
    }
}
