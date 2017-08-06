#![allow(warnings)] // Don't bother with warnings for now
extern crate gtk;
use gtk::traits::*;

extern crate futures;
use futures::Future;
use futures::stream::Stream;

extern crate hyper;
use hyper::Client;

extern crate url;
use url::Url;

extern crate tokio_core;

use std::io;
use std::io::prelude::*;

const NROWS:    u64  = 1000;
const BASE_URL: &str = "https://hacker-news.firebaseio.com/";

// TODO -> Get HTTPS working
fn main() {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle   = core.handle();
    let client   = Client::new(&handle);


    let mut url = Url::parse(BASE_URL).unwrap();
    let url = url.join("/v0/topstories.json?print=pretty").unwrap();
    let url = url.to_string().parse::<hyper::Uri>().unwrap();

    let work = client.get(url).and_then(|res| {
        println!("Response: {}", res.status());
        println!("Headers: \n{}", res.headers());

        res.body().for_each(|chunk| {
            io::stdout().write_all(&chunk).map_err(From::from)
        })
    }).map(|_| {
        println!("\n\nDone.");
    });

    match core.run(work) {
        Ok(result) => {
            println!("OK: {:?}", result);
        }
        Err(e) => {
            println!("[[FAIL]] {:#?}", e);
        }
    };

    // gtk_stuff();
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
