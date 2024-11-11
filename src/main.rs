mod auth;
mod logger;
mod utils;

use auth::authenticator::MyAuthenticator;
use dotenv::dotenv;
use env_logger;
use libunftp::Server;
use log::{self, error};
use logger::listener::MyPresenceListener;
use std::{io::Error, sync::Arc};
use unftp_sbe_fs::ServerExt;
use utils::get_local_ip;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    dotenv().ok();

    let ftp_home = std::env::temp_dir();
    let local_ip = get_local_ip()?;
    let port = 8023;
    env_logger::init();

    let server = Server::with_fs(ftp_home)
        .greeting("Welcome to my FTP server")
        .passive_ports(50000..65535)
        .authenticator(Arc::new(MyAuthenticator::default()))
        .notify_presence(MyPresenceListener)
        .build()
        .unwrap();

    log::info!("FTP server: {local_ip}:{port}");
    if let Err(e) = server.listen(format!("{local_ip}:{port}")).await {
        error!("Failed to start the FTP server: {}", e);
    }
    Ok(())
}
