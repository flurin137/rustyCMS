use gotham::state::State;
use hyper::{Body, Response, StatusCode};

use crate::file_handling::parser::*;
use crate::file_handling::reader::*;
use gotham::helpers::http::response::create_response;

pub fn index(state: State) -> (State, Response<Body>) {

    let file_content = match read_index(){
        Ok(content) => content,
        Err(error) => return (state, Response::new(Body::from(error)))
    };

    let parsed = match parse_file(file_content){
        Ok(html) => html,
        Err(error) => return (state, Response::new(Body::from(error)))
    };

    let body = Body::from(parsed);

    let response = create_response(
        &state,
        StatusCode::OK,
        mime::TEXT_PLAIN,
        body,
    );

    (state, response)
}