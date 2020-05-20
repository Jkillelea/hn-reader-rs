// GUI
#[allow(warnings)]
extern crate gtk;
use gtk::traits::*;

extern crate reqwest;
extern crate url;
use url::{Url, ParseError};

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use serde_json::Value;
use Value::{Array};

// use std::io;
// use std::io::BufWriter;
use std::io::prelude::*;

use std::thread;
use std::sync::mpsc;

const BASE_URL: &str = "https://hacker-news.firebaseio.com/";

#[derive(Serialize, Deserialize, Debug)]
struct HnArticle {
    // Option wrappers around fields that may or may not be present.
    // HackerNews also returns a 'type' field but that's a keyword in
    // Rust so I can't use that field for now...
    id:           usize,
    by:           String,
    url:          Option<String>,
    score:        usize,
    time:         usize,
    title:        String,
    descendants:  Option<usize>,
    kids:         Option<Vec<usize>>,
}


fn main() {
    // gtk::init().unwrap_or_else(|_| panic!("Failed to start GTK"));
    //
    // let builder   = gtk::Builder::new();
    // let glade_src = include_str!("../ui/gui.ui");
    // builder.add_from_string(glade_src).unwrap();
    //
    // let window: gtk::Window   = builder.get_object("window1").unwrap();
    // let listbox: gtk::ListBox = builder.get_object("listbox1").unwrap();
    //
    // window.connect_delete_event(move |_, _| {
    //     gtk::main_quit();
    //     gtk::Inhibit(false)
    // });
    //
    // window.set_title("HackerNews Reader");
    // window.show_all();

    let     url           = topstories_url().unwrap().to_string();
    let     client        = reqwest::Client::new().unwrap();
    let mut response_body = String::new();
    let mut res           = client.get(&url).unwrap().send().unwrap();
    let (tx, rx)          = mpsc::channel();

    res.read_to_string(&mut response_body).unwrap();
    let json: serde_json::Value = serde_json::from_str(&response_body).unwrap();

    let ids: Vec<i64> = match json {
        Array(values) => {
            values.into_iter()
                  .map(|v| v.as_i64().unwrap_or_else(|| 0))
                  .filter(|v| *v != 0)
                  .collect()
        },
        _ => {
            println!("Recieved unexpected response from server:\n{}", json);
            return
        },
    };


    for id in ids {
        let thread_tx = tx.clone();
        thread::spawn(move || {
            let url    = id_url(id).unwrap().to_string();
            let client = reqwest::Client::new().unwrap_or_else(|e| {
                            eprintln!("{:?}", e);
                            panic!();
                         });
            let mut response_body = String::new();
            let mut res           = client.get(&url).unwrap().send().unwrap();
            res.read_to_string(&mut response_body).unwrap();
            thread_tx.send(response_body).unwrap();
        });
    }

    for recv in rx {
        let article: HnArticle = serde_json::from_str(&recv).unwrap();
        println!("{}", article.title);
    }

    // gtk::main();
}

fn topstories_url() -> Result<Url, ParseError> {
    let url = Url::parse(BASE_URL)?
                  .join("/v0/topstories.json?print=pretty")?;
    Ok(url)
}

fn id_url(id: i64) -> Result<Url, ParseError> {
    let url = Url::parse(BASE_URL)?
                  .join(&format!("/v0/item/{}.json?print=pretty", id))?;
    Ok(url)
}
