mod config;

use std::{
    cell::RefCell,
    collections::HashMap,
    net::SocketAddr,
    path::PathBuf,
    rc::Rc,
    time::{Duration, Instant},
};

use config::Config;
use neqo_common::{self, event::Provider};
use neqo_crypto::{AllowZeroRtt, AntiReplay, AuthenticationStatus, ZeroRttChecker, init};
use neqo_http3::{Error, Http3Parameters, Http3Server};
use neqo_transport::{Connection, ConnectionId, Output, RandomConnectionIdGenerator, State};

fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let config = Config::default();
    // Initialize NSS (TLS library)
    init().expect("NSS initialization failed");

    // Load certificates
    let certs = std::fs::read_to_string(config.cert).expect("Failed to read certificate file");
    let key = std::fs::read_to_string(config.key).expect("Failed to read private key file");

    // Create HTTP/3 server
    let mut server = Http3Server::new(
        Instant::now(),
        &[certs],
        &[key],
        AntiReplay::new(Instant::now(), Duration::from_millis(500), 1, 3).unwrap(), // TODO: check if this is correct
        Rc::new(RefCell::new(RandomConnectionIdGenerator::new(32))), // connection callback
        Http3Parameters::default().webtransport(true),
        None,
    )
    .expect("Failed to create HTTP/3 server");

    // Bind to address
    let addr = SocketAddr::new("::".parse().unwrap(), config.port);
    server.set_qlog_dir(Some(std::path::PathBuf::from(".")));

    println!("HTTP/3 server running at {:?}", addr);

    // Run the server
    let _socket = std::net::UdpSocket::bind(addr).expect("Failed to bind to address");

    // Main event loop would go here
    // This is a simplified example - a full implementation would include
    // handling of socket I/O and event processing

    Ok(())
}
