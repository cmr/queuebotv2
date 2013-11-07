//! Queuebot web service

#[feature(globs)];

extern mod extra;
extern mod http;
extern mod postgres;

use std::rt::io::Writer;

use http::server::{Config, Server, ServerUtil, Request, ResponseWriter};
use http::server::request::AbsolutePath;
use http::status::{NotImplemented};

mod logic;
pub mod data;

#[deriving(Clone)]
struct QueueBotServer;

impl Server for QueueBotServer {
    fn get_config(&self) -> Config {
        Config {
            bind_address: FromStr::from_str("127.0.0.1:1472").unwrap()
        }
    }
    fn handle_request(&self, r: &Request, w: &mut ResponseWriter) {
        match r.request_uri {
            AbsolutePath(ref p) => logic::dispatch_path(p, r, w),
            _ => {
                w.status = NotImplemented;
                w.write(bytes!("Non-path URIs not implemented"));
                return;
            }
        }
    }
}

fn main() {
    QueueBotServer.serve_forever();
}
