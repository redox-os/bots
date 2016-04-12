extern crate mars;

use mars::{Bot, Response};
use std::fs;
use std::io::Read;
use std::sync::{Arc, Mutex};

fn main() {
    let rand = Arc::new(Mutex::new(fs::File::open("/dev/random").unwrap()
        .bytes()
        .filter_map(|x| {
            x.ok().map(|x| if x & 1 == 0 { Some(()) } else { None })
        })));

    Bot::new(env!("TOKEN"), move |_| {
        Response {
            username: Some("tossbot"),
            text: if rand.lock().unwrap().next().is_some() { ":+1:".into() } else { ":-1:".into() },
            icon_url: Some("https://upload.wikimedia.org/wikipedia/commons/2/28/Coin_of_Mysia_4th_century_BCE.jpg"),
        }
    }).init(env!("IP")).unwrap();
}
