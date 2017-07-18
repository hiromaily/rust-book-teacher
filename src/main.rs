extern crate futures;
extern crate hyper;
extern crate tokio_core;

//use std::env;
use std::io::{self, Write};

use futures::Future;
use futures::stream::Stream;

use hyper::Client;


struct TeacherInfo {
  id:   i16,
  name: &'static str,
  country: &'static str,
}

const BASE_URL: &'static str = "http://eikaiwa.dmm.com/teacher/index/";
//static TEACHER: TeacherInfo = TeacherInfo{ id: 20262, name: "Mia Ca", country: "Serbia"};

//main
fn main() {
    println!("[rust-book-teacher]");

    let ts = teachers_data();
    for t in ts.iter() {
        println!("{}:{}:[{}]", t.id, t.name, t.country);
        get_http(t);
        //scraping(t);
    }

}

fn teachers_data() -> [TeacherInfo; 5] {
    //let t: [TeacherInfo; 5] = [
    [
        TeacherInfo{ id: 20262, name: "Mia Ca", country: "Serbia"},
        TeacherInfo{ id: 1381, name: "Anna O", country: "Russia"},
        TeacherInfo{ id: 2464, name: "Emilia", country: "Serbia"},
        TeacherInfo{ id: 3230, name: "Niki", country: "Bosnia"},
        TeacherInfo{ id: 3486, name: "Indre T", country: "Lithuania"}
    ]
}

//get_http
fn get_http(t: &TeacherInfo) {
    let raw_url: String = format!("{}{}{}", BASE_URL, t.id, "/");
    let url = raw_url.parse::<hyper::Uri>().unwrap();
    if url.scheme() != Some("http") {
        println!("This example only works with 'http' URLs.");
        return;
    }

    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client = Client::new(&handle);

    //*
    let work = client
        .get(url)
        .and_then(|res| {
            //println!("Response: {}", res.status());
            //println!("Headers: \n{}", res.headers());
            res.body().for_each(|chunk| {
                io::stdout().write_all(&chunk).map_err(From::from)
            })
        })
        .map(|_| {
            println!("\n\nDone.");
        });

    core.run(work).unwrap();
    //*/
}

//scraping
//fn scraping(t: &TeacherInfo) -> String{
//}