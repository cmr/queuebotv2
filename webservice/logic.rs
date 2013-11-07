use data;
use data::to_json;

use http::server::{ResponseWriter, Request};
use http::method::{Get, Put, Delete, Post};

use http::status::{Status, NotFound, MethodNotAllowed};

pub fn dispatch_path(path: &~str, r: &Request, w: &mut ResponseWriter) {
    if path[0] != '/' as u8 {
        warn2!("Request to {} invalid", *path);
        return;
    }

    let mut paths = path.slice_from(1).split_iter('/');
    let x = paths.next();
    match x {
        Some("user") => {
            let username = match paths.next() {
                Some("") | None => { bail(NotFound, w); return; }
                Some(s) => s,
            };
            if paths.clone().len() != 0 {
                bail(NotFound, w); return
            }
            match r.method.clone() {
                Get => {
                    match data::User::get_by_name(username) {
                        Some(user) => {
                            let userstr = to_json(&user);
                            w.write(userstr.as_bytes());
                            return;
                        },
                        None => { bail(NotFound, w); return; }
                    }
                },
                x@Post | x@Put | x@Delete => {
                    debug2!("Got a {}, paths is {:?}", x.to_str(), paths);
                    if !paths.next().is_none() { bail(NotFound, w); return }
                    debug!("And it's empty, let's finish!");
                    // decode body, check if user exists, update if so
                },
                _ => { bail(MethodNotAllowed, w); return }
            }
        },
        _ => bail(NotFound, w)
    }
}

fn bail(s: Status, w: &mut ResponseWriter) {
    w.status = s;
    let r = w.status.to_str();
    w.write(r.as_bytes());
    w.write(bytes!("\n"));
}
