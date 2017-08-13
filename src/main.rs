// GUI
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
use std::io::prelude::*;

// use std::thread;
// use std::sync::mpsc;

const BASE_URL: &str = "https://hacker-news.firebaseio.com/";

// {
//   "by" : "artsandsci",
//   "descendants" : 16,
//   "id" : 14970171,
//   "kids" : [ 14971679, 14971331, 14971597, 14970967, 14970908, 14971481, 14970936, 14970742 ],
//   "score" : 48,
//   "time" : 1502292702,
//   "title" : "Bitcoin Makes Even Smart People Feel Dumb",
//   "type" : "story",
//   "url" : "https://www.wired.com/story/bitcoin-makes-even-smart-people-feel-dumb"
// }
#[derive(Serialize, Deserialize, Debug)]
struct HnArticle {
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
    gtk::init().unwrap_or_else(|_| panic!("Failed to start GTK"));

    let builder = gtk::Builder::new();
    let glade_src = include_str!("../ui/gui.ui");
    builder.add_from_string(glade_src).unwrap();

    let window: gtk::Window   = builder.get_object("window1").unwrap();
    let listbox: gtk::ListBox = builder.get_object("listbox1").unwrap();

    window.connect_delete_event(move |_, _| {
        gtk::main_quit();
        gtk::Inhibit(false)
    });

    window.set_title("HackerNews Reader");
    window.show_all();

    let url = topstories_url().unwrap().to_string();
    let client        = reqwest::Client::new().unwrap();
    let mut response_body = String::new();
    let mut res           = client.get(&url).unwrap().send().unwrap();

    res.read_to_string(&mut response_body).unwrap();
    let json: serde_json::Value = serde_json::from_str(&response_body).unwrap();

    if let Array(values) = json {
        let mut i = 0;

        for id in &values {
            let id  = id.as_i64().unwrap();
            let url = id_url(id).unwrap();
            response_body.clear();

            let mut res = client.get(url).unwrap().send().unwrap();
            res.read_to_string(&mut response_body).unwrap();

            match serde_json::from_str::<HnArticle>(&response_body) {
                Ok(json) => {
                    listbox.add(&gtk::Button::new_with_label(&json.title));
                    window.show_all();

                    // println!("{}: {}", i, json.title);
                    // if json.url.is_some() {
                    //     println!("\t{}", json.url.unwrap());
                    // }
                }
                Err(e) => {
                    println!("{}", e);
                }
            }

            i += 1;
        }
        println!("Retrieved {} out of {} posts", i, values.len());
    }

    gtk::main();
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
