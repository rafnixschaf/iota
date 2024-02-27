// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Test utilities that relate to local ip addresses.

use std::net::SocketAddr;

use crate::multiaddr::Multiaddr;

/// Returns localhost, which is always 127.0.0.1.
pub fn localhost_for_testing() -> String {
    "127.0.0.1".to_string()
}

/// Return an ephemeral, available port. On unix systems, the port returned will be in the
/// TIME_WAIT state ensuring that the OS won't hand out this port for some grace period.
/// Callers should be able to bind to this port given they use SO_REUSEADDR.
pub fn get_available_port(host: &str) -> u16 {
    const MAX_PORT_RETRIES: u32 = 1000;

    for _ in 0..MAX_PORT_RETRIES {
        if let Ok(port) = get_ephemeral_port(host) {
            return port;
        }
    }

    panic!(
        "Error: could not find an available port on {}: {:?}",
        host,
        get_ephemeral_port(host)
    );
}

fn get_ephemeral_port(host: &str) -> std::io::Result<u16> {
    use std::net::{TcpListener, TcpStream};

    // Request a random available port from the OS
    let listener = TcpListener::bind((host, 0))?;
    let addr = listener.local_addr()?;

    // Create and accept a connection (which we'll promptly drop) in order to force the port
    // into the TIME_WAIT state, ensuring that the port will be reserved from some limited
    // amount of time (roughly 60s on some Linux systems)
    let _sender = TcpStream::connect(addr)?;
    let _incoming = listener.accept()?;

    Ok(addr.port())
}

/// Returns a new unique TCP address for the given host, by finding a new available port.
pub fn new_tcp_address_for_testing(host: &str) -> Multiaddr {
    format!("/ip4/{}/tcp/{}/http", host, get_available_port(host))
        .parse()
        .unwrap()
}

/// Returns a new unique UDP address for the given host, by finding a new available port.
pub fn new_udp_address_for_testing(host: &str) -> Multiaddr {
    format!("/ip4/{}/udp/{}", host, get_available_port(host))
        .parse()
        .unwrap()
}

/// Returns a new unique TCP address (SocketAddr) for localhost, by finding a new available port on localhost.
pub fn new_local_tcp_socket_for_testing() -> SocketAddr {
    format!(
        "{}:{}",
        localhost_for_testing(),
        get_available_port(&localhost_for_testing())
    )
    .parse()
    .unwrap()
}

/// Returns a new unique TCP address (Multiaddr) for localhost, by finding a new available port on localhost.
pub fn new_local_tcp_address_for_testing() -> Multiaddr {
    new_tcp_address_for_testing(&localhost_for_testing())
}

/// Returns a new unique UDP address for localhost, by finding a new available port.
pub fn new_local_udp_address_for_testing() -> Multiaddr {
    new_udp_address_for_testing(&localhost_for_testing())
}

pub fn new_deterministic_tcp_address_for_testing(host: &str, port: u16) -> Multiaddr {
    format!("/ip4/{host}/tcp/{port}/http").parse().unwrap()
}

pub fn new_deterministic_udp_address_for_testing(host: &str, port: u16) -> Multiaddr {
    format!("/ip4/{host}/udp/{port}/http").parse().unwrap()
}
