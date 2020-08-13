
extern crate gotham;
extern crate hyper;
extern crate mime;

use gotham::router::Router;
use gotham::router::builder::*;

use crate::handlers::*;

pub fn setup_router() -> Router {
    build_simple_router(|route| {
        route.get_or_head("/").to(index);
    })
}