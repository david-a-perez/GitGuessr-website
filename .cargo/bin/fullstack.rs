mod dsync;
mod tsync;

use std::{
    net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6, TcpListener, ToSocketAddrs},
    ops::Range,
};

/// binds a [`TcpListener`] to the given [`addr`](`ToSocketAddrs`)
fn test_bind<A: ToSocketAddrs>(addr: A) -> bool {
    TcpListener::bind(addr)
        .map(|t| t.local_addr().is_ok())
        .unwrap_or(false)
}

/// is the given port free on this machine
pub fn is_port_free(port: u16) -> bool {
    let ipv4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    let ipv6 = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, port, 0, 0);

    test_bind(ipv6) && test_bind(ipv4)
}

/// max range is 65535
pub fn find_free_port(mut range: Range<u16>) -> Option<u16> {
    range.find(|port| is_port_free(*port))
}

pub fn main() {
    if !create_rust_app::net::is_port_free(21012) {
        println!("========================================================");
        println!(" ViteJS (the frontend compiler/bundler) needs to run on");
        println!(" port 21012 but it seems to be in use.");
        println!("========================================================");
        panic!("Port 21012 is taken but is required for development!")
    }

    let project_dir = env!("CARGO_MANIFEST_DIR");

    dsync::main();
    tsync::main();
    
    if std::env::var("DEV_SERVER_PORT").is_err() {
        std::env::set_var(
            "DEV_SERVER_PORT",
            find_free_port(9000..9100)
                .expect("FATAL: Could not find a free port for the development server.")
                .to_string(),
        );
    }

    create_rust_app::dev::run_server(project_dir);
}
