use std::sync::{ Arc, Mutex };
use std::collections::{ VecDeque, HashMap };

use axum::{
    extract::{Query, Request, State}, 
    response::{Response, IntoResponse, Json},
    http::{header, Method, HeaderMap},
    routing::{get, post, MethodRouter},
    body::Bytes,
    Router,
};
use serde_json::{Value, json};


use super::ctx::*;


include!("test.rs");


pub fn routes() -> Router<ApiCtx> {

    Router::new().route(&query("testapi1234563847653475"), get(testapi1234563847653475))

}






