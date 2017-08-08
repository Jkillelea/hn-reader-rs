#![allow(warnings)] // Don't bother with warnings for now
// GUI
extern crate gtk;
use gtk::traits::*;

extern crate reqwest;
extern crate serde_json;
use serde_json::Value;
use Value::{Array, Number};

extern crate url;
use url::{Url, ParseError};

use std::io;
use std::io::Read;
use std::io::BufWriter;
use std::io::prelude::*;

use std::thread;
use std::sync::mpsc;

const NROWS:    u64  = 1000;
const BASE_URL: &str = "https://hacker-news.firebaseio.com/";

fn main() {
    let mut stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut response_body = String::new();
    let url = topstories_url().unwrap().to_string();

    let mut client = reqwest::Client::new().unwrap();
    let mut res    = client.get(&url).unwrap().send().unwrap();

    res.read_to_string(&mut response_body);
    let json: serde_json::Value = serde_json::from_str(&response_body).unwrap();

    if let Array(values) = json {
        let mut i = 0;
        let (tx, rx) = mpsc::channel();
        for id in values {
            let thread_tx = tx.clone();
            let id        = id.as_i64().unwrap();
            let mut body  = String::new();

            thread::spawn(move || {
                let mut client = client_or_sleep();
                let url        = id_url(id).unwrap();
                let mut res    = client.get(url).unwrap().send().unwrap();
                res.read_to_string(&mut body);

                thread_tx.send((i, body));
            });

            // println!("{:?}", response_body);
            // stdout.write(response_body.as_bytes()).unwrap();
            i += 1;
        }

        for (i, body) in rx {
            println!("{} {}", i, body);
        }
    }

    // gtk_stuff();
}

// Return a new Client type or sleep 1 second and try again
fn client_or_sleep() -> reqwest::Client {
    let mut client = match reqwest::Client::new() {
        Ok(client) => return client,
        Err(_) => {
            thread::sleep(::std::time::Duration::new(1, 0));
            return client_or_sleep();
        }
    };
}

fn topstories_url() -> Result<Url, ParseError> {
    let mut url = Url::parse(BASE_URL)?
                      .join("/v0/topstories.json?print=pretty")?;
    Ok(url)
}

fn id_url(id: i64) -> Result<Url, ParseError> {
    let mut url = Url::parse(BASE_URL)?
                      .join(&format!("/v0/item/{}.json?print=pretty", id))?;
    Ok(url)
}

fn gtk_stuff() {
    // gtk::init().unwrap_or_else(|_| panic!("Failed to start GTK!"));
    //
    // let builder = gtk::Builder::new();
    // let glade_src = include_str!("../ui/gui.ui");
    // builder.add_from_string(glade_src).unwrap();
    //
    // // type annotation nessecary here
    // let window:  gtk::Window  = builder.get_object("window1").unwrap();
    // let button1: gtk::Button  = builder.get_object("button1").unwrap();
    // let button2: gtk::Button  = builder.get_object("button2").unwrap();
    // let list:    gtk::ListBox = builder.get_object("listbox1").unwrap();
    //
    // window.connect_delete_event(move |_, _| {
    //     gtk::main_quit();
    //     gtk::Inhibit(true)
    // });
    // window.set_title("HackerNews Reader");
    //
    // for button in &[button1, button2] {
    //     button.connect_clicked(|_| {
    //         println!("Button!")
    //     });
    // }
    // for _ in 0..NROWS {
    //     let row = gtk::ListBoxRow::new();
    //     list.add(&row);
    // }
    //
    // let mut i = 0;
    // while let Some(row) = list.get_row_at_index(i) {
    //     let button = gtk::Button::new_with_label(&format!("Test {}", i));
    //     button.connect_clicked(move |_| { println!("Button {} clicked!", i) });
    //     row.add(&button);
    //     i += 1;
    // }
    //
    // window.show_all();
    // gtk::main();
}
