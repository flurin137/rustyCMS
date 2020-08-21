extern crate hyper;
extern crate mime;

use crate::handlers::handle_path;
use crate::handlers::handle_static_path;

use nickel::{Nickel, HttpRouter};
use nickel::hyper::uri::RequestUri;

pub fn explicit_router() -> nickel::Router {
    let mut router = Nickel::router();

    router.get("**.css", middleware! { |request|
        handle_static_path("splendor.min.css")
    });

    router.get("**", middleware! { |request|
        let request_url: Option<&str> = match &request.origin.uri{
            RequestUri::AbsolutePath(path) if path != "/" => {
                println!("{}", path);
                Some(path)
            },
            _ => None,
        };

        handle_path(request_url)
    });

    router
}