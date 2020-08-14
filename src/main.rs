extern crate gotham;

mod router;
mod handlers;
mod file_handling;

use router::*;

pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, setup_router())
}
