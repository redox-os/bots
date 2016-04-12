extern crate mars;

use mars::{Bot, Response};

fn main() {
    Bot::new(env!("TOKEN"), move |req| {
        let query = req.text.trim_left_matches("docbot:").trim_left_matches("docbot ").trim();

        Response {
            username: Some("docbot"),
            text: format!("[\"{}\"](https://doc.rust-lang.org/nightly/std/?search={})", query, query),
            icon_url: Some("http://www.modulesca.com/phototeque/Documentation_6.jpg"),
        }
    }).init(env!("IP")).unwrap();
}
