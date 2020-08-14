extern crate hyper;
extern crate mime;

use crate::handlers::*;

use nickel::{Nickel, HttpRouter};
use nickel::hyper::uri::RequestUri;

pub fn explicit_router() -> nickel::Router {
    let mut router = Nickel::router();

    router.get("**", middleware! { |request|

        let asda: Option<String> = match &request.origin.uri{
            RequestUri::AbsolutePath(path) if path != "/" => {
                println!("{}", path);
                Some(format!("{}", path))
            },
            _ => None,
        };

        handle_path(asda)
    });

    router
}