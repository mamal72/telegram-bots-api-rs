/*extern crate hyper;
extern crate rustc_serialize;*/

//use std::io;
use std::io::Read;

use super::rustc_serialize::json;
use super::rustc_serialize::Decodable;

use super::hyper::Client;
use super::hyper::header::Connection;

static BASE_PATH: &'static str = "https://api.telegram.org/";

pub struct Bot {
    api_key: String
}


#[derive(RustcDecodable)]
struct _GetMeResult {
    id: i32,
    first_name: String,
    username: String
}

#[derive(RustcDecodable)]
pub struct GetMe {
    pub ok: bool,
    pub result: _GetMeResult
}

impl Bot {
    pub fn new(api_key: &str) -> Bot {
        Bot {
            api_key: api_key.to_string()
        }
    }

    pub fn get_me(&self) -> String {
        let path = format!("{}bot{}/getMe", BASE_PATH, self.api_key);
        let client = Client::new();
        let mut res = client.get(&path)
            .send()
            .unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        return body;
    }

    pub fn send_message(&self, chat_id: i32, text: String) -> String {
        let path = format!("{}bot{}/sendMessage?chat_id={}&text={}", BASE_PATH, self.api_key, chat_id, text);
        let client = Client::new();
        let mut res = client.get(&path)
            .send()
            .unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        return body;
    }
}