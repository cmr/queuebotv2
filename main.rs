//! Queuebot web service

extern mod extra;
extern mod http;

use std::rt::io::Writer;

use http::server::{Config, Server, ServerUtil, Request, ResponseWriter};
use http::server::request::AbsolutePath;
use http::status::{NotImplemented};

#[deriving(Clone)]
struct QueueBotServer;

impl Server for QueueBotServer {
    fn get_config(&self) -> Config {
        Config {
            bind_address: FromStr::from_str("127.0.0.1:1472").unwrap()
        }
    }
    fn handle_request(&self, r: &Request, w: &mut ResponseWriter) {
        let path = match r.request_uri {
            AbsolutePath(ref p) => p.clone(),
            _ => {
                w.status = NotImplemented;
                w.write(bytes!("Non-path URIs not implemented"));
                return;
            }
        };
        w.write(path.as_bytes());
    }
}

fn main() {
    QueueBotServer.serve_forever();
}
