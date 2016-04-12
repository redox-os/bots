extern crate mars;

use mars::{Bot, Response};
use std::fs;
use std::io::Read;
use std::sync::{Arc, Mutex};

fn main() {
    let rand = Arc::new(Mutex::new(fs::File::open("/dev/urandom").unwrap()));

    Bot::new(env!("TOKEN"), move |_| {
        let mut outcome = [0];
        rand.lock().unwrap().read_exact(&mut outcome).unwrap();
        Response {
            username: Some("tossbot"),
            text: if outcome[0] % 2 == 0 { ":+1:".into() } else { ":-1:".into() },
            icon_url: Some("https://upload.wikimedia.org/wikipedia/commons/2/28/Coin_of_Mysia_4th_century_BCE.jpg"),
        }
    }).init(env!("IP")).unwrap();
}
