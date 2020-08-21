extern crate hyper;
#[macro_use] extern crate nickel;

mod router;
mod handlers;
mod file_handling;

use crate::router::*;

use nickel::{Nickel};

pub fn main() {
    let mut server = Nickel::new();

    server.utilize(explicit_router());
    
    let addr = "127.0.0.1:5050";
    println!("Listening for requests at http://{}", addr);
    server.listen(addr).unwrap();
}
