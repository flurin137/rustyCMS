extern crate hyper;
extern crate mime;

use crate::handlers::*;

use nickel::{Nickel, HttpRouter};

pub fn explicit_router() -> nickel::Router {
    let mut router = Nickel::router();

    router.get("/", middleware! {
        index()
    });

    router
}