// A microframework for Rust.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

//! Pencil is a microframework for Rust inspired by [Flask](http://flask.pocoo.org/).
//!
//! # Installation
//!
//! This crate is called `pencil` and you can depend on it via cargo:
//!
//! ```ini
//! [dependencies.pencil]
//! git = "https://github.com/fengsp/pencil.git"
//! ```
//!
//! # Quickstart
//!
//! A short introduction to Pencil.
//!
//! ## A Minimal Application
//!
//! A minimal Pencil application looks something like this:
//!
//! ```rust,no_run
//! extern crate pencil;
//!
//! use pencil::Pencil;
//! use pencil::{Request, PencilResult, PenString};
//!
//!
//! fn hello(_: Request) -> PencilResult {
//!     Ok(PenString(String::from_str("Hello World!")))
//! }
//!
//!
//! fn main() {
//!     let mut app = Pencil::new("/web/hello");
//!     app.route(r"/", &["GET"], "hello", hello);
//!     app.run();
//! }
//! ```

#![allow(unused_attributes)]
#![crate_name = "pencil"]
#![crate_type = "lib"]
#![license = "BSD"]
#![comment = "A microframework for Rust."]
#![doc(html_logo_url = "https://raw.githubusercontent.com/fengsp/pencil/master/logo/pencil.png",
       html_favicon_url = "https://raw.githubusercontent.com/fengsp/pencil/master/logo/favicon.ico",
       html_root_url = "http://fengsp.github.io/pencil/")]

#![experimental]

#![deny(non_camel_case_types)]

#![feature(macro_rules)]

extern crate core;
extern crate serialize;
extern crate regex;
extern crate http;
extern crate url;

/* public api */
pub use app::Pencil;
pub use types::{
    PencilValue,
        PenString,
        PenResponse,
    PencilError,
        PenHTTPError,
        PenUserError,
    UserError,
    PencilResult,
    ViewArgs,
    ViewFunc,
    UserErrorHandler,
    HTTPErrorHandler,
    BeforeRequestFunc,
    AfterRequestFunc,
    TeardownRequestFunc,
};
pub use datastructures::{
    Headers,
    MultiDict,
};
pub use wrappers::{
    Request,
    Response,
};
pub use json::jsonify;
pub use config::{
    Config,
};
pub use helpers::{
    make_response,
    PathBound,
    safe_join,
    abort,
    redirect,
    escape,
    send_file,
    send_from_directory,
};
pub use serving::{
    run_server,
};
pub use errors::{
    HTTPError,
        BadRequest,
        Unauthorized,
        Forbidden,
        NotFound,
        MethodNotAllowed,
        NotAcceptable,
        RequestTimeout,
        Conflict,
        Gone,
        LengthRequired,
        PreconditionFailed,
        RequestEntityTooLarge,
        RequestURITooLarge,
        UnsupportedMediaType,
        RequestedRangeNotSatisfiable,
        ExpectationFailed,
        ImATeapot,
        UnprocessableEntity,
        PreconditionRequired,
        TooManyRequests,
        RequestHeaderFieldsTooLarge,
        InternalServerError,
        NotImplemented,
        BadGateway,
        ServiceUnavailable,
};
pub use testing::PencilClient;
pub use httputils::{
    get_name_by_http_code,
};

mod app;
mod types;
mod datastructures;
mod wrappers;
mod json;
mod config;
mod logging;
mod helpers;
mod serving;
mod errors;
mod routing;
mod testing;
mod httputils;
