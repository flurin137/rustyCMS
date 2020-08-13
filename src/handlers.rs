use gotham::state::State;
use hyper::{Body, Response, StatusCode};

use crate::parser::*;
use gotham::helpers::http::response::create_response;

pub fn index(state: State) -> (State, Response<Body>) {

    let parsed = match parse_file("../../../user/index.md"){
        Ok(html) => html,
        Err(error) => return (state, Response::new(Body::from(error)))
    };

    let body = Body::from(parsed);

    let response = create_response(
        &state,
        StatusCode::OK,
        mime::TEXT_JAVSCRIPT,
        body,
    );

    (state, response)
}