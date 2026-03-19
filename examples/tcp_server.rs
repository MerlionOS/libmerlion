//! Example: TCP echo server on MerlionOS
//!
//! Demonstrates std::net equivalent using MerlionOS syscalls.

#![no_std]
#![no_main]

extern crate merlion_std;

use merlion_std::io::{self, Read, Write};
use merlion_std::net::{TcpListener, TcpStream, SocketAddr, Ipv4Addr};
use merlion_std::process;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    io::println("Starting TCP echo server on :8080...");

    let addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED, 8080);
    match TcpListener::bind(addr) {
        Ok(listener) => {
            io::println("Listening on :8080");
            loop {
                match listener.accept() {
                    Ok((mut stream, _peer)) => {
                        io::println("New connection!");
                        let mut buf = [0u8; 256];
                        match stream.read(&mut buf) {
                            Ok(n) if n > 0 => {
                                let _ = stream.write(&buf[..n]); // echo back
                            }
                            _ => {}
                        }
                    }
                    Err(_) => {
                        merlion_std::thread::yield_now();
                    }
                }
            }
        }
        Err(e) => {
            io::println("bind failed");
            process::exit(1);
        }
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    process::exit(1);
}
