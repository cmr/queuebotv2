use std;
use extra::json;
use extra::uuid::Uuid;
use extra::time::Timespec;
use extra::treemap::TreeMap;
use extra::serialize::{Encoder, Encodable};

use postgres::*;
use postgres::types::ToSql;

pub fn to_json<T: Encodable<json::Encoder>>(x: &T) -> ~str {
    do std::io::with_str_writer |w| {
        x.encode(&mut json::Encoder(w));
    }
}

#[deriving(Encodable)]
struct User {
    id: Uuid,
    username: ~str,
    // XXX: this is plaintext!
    password: ~str,
}

#[deriving(Encodable)]
struct Alias {
    userid: Uuid,
    name: ~str,
}

#[deriving(Encodable)]
struct Queue {
    id: Uuid,
    userid: Uuid,
    title: ~str,
}

#[deriving(Encodable)]
struct Task {
    creation_time: Timespec,
    due_date: Timespec,
    priority: int,
    creator: Uuid,
    content: ~str,
    title: ~str,
    queue: Uuid,
}

impl User {
    pub fn get_by_name(name: &str) -> Option<User> {
        let conn = open();
        let stmt = conn.prepare("SELECT userid, username, password FROM users WHERE username = $1");
        for row in stmt.query([&name as &ToSql]) {
            // TODO: assert there's only one
            return Some(User { id: row[0], username: row[1], password: row[2] });
        }
        User::get_by_alias(name)
    }

    pub fn get_by_id(id: &Uuid) -> Option<User> {
        let conn = open();
        let stmt = conn.prepare("SELECT userid, username, password FROM users WHERE userid = $1");
        for row in stmt.query([id as &ToSql]) {
            // TODO: assert there's only one
            return Some(User { id: row[0], username: row[1], password: row[2] });
        }
        None
    }

    pub fn get_by_alias(name: &str) -> Option<User> {
        let conn = open();
        let stmt = conn.prepare("SELECT userid FROM aliases WHERE name = $1");
        for row in stmt.query([&name as &ToSql]) {
            return User::get_by_id(&row[0]);
        }
        None
    }

    pub fn create(name: &str, password: &str) -> Option<Uuid> {
        // TODO: error checking
        let conn = open();
        let id = Uuid::new_v4();
        conn.update("INSERT INTO users VALUES ($1, $2, $3)",
                    &[&name as &ToSql, &password as &ToSql, &id as &ToSql]);
        Some(id)
    }

    pub fn has_alias(&self, alias: &str) -> bool {
        let conn = open();
        let stmt = conn.prepare("SELECT 1 FROM aliases WHERE name = $1 and userid = $2");
        for _ in stmt.query([&alias as &ToSql, &self.id as &ToSql]) {
            return true;
        }
        false
    }

    pub fn get_aliases(&self) -> ~[~str] {
        let conn = open();
        let mut names = ~[];

        let stmt = conn.prepare("SELECT name FROM aliases WHERE userid = $1");
        for row in stmt.query([&self.id as &ToSql]) {
            names.push(row[0]);
        }
        names
    }
}

fn open() -> PostgresConnection {
    let conn = PostgresConnection::try_connect("postgres://cmr@localhost/queue");
    match conn {
        Ok(s) => return s,
        Err(e) => fail2!("Could not open db connection: {}", e.to_str())
    }
}
