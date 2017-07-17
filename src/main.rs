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
    //for t in ts {
    for t in ts.iter() {
        println!("{}:{}:[{}]", t.id, t.name, t.country);
        call_http(t);
    }

    //number_test(20);

    //call_http("http://eikaiwa.dmm.com/teacher/index/20262/".to_string());

}

fn teachers_data() -> [TeacherInfo; 5] {
    //static TEACHERS: TeacherInfo = TeacherInfo{ id: 20262, name: "Mia Ca", country: "Serbia"};
    //let t: [TeacherInfo; 5] = [
    [
        TeacherInfo{ id: 20262, name: "Mia Ca", country: "Serbia"},
        TeacherInfo{ id: 1381, name: "Anna O", country: "Russia"},
        TeacherInfo{ id: 2464, name: "Emilia", country: "Serbia"},
        TeacherInfo{ id: 3230, name: "Niki", country: "Bosnia"},
        TeacherInfo{ id: 3486, name: "Indre T", country: "Lithuania"}
    ]
}

//number_test is just test
/*
fn number_test(z: i32) {
    let x = 5;
    let y = 10;

    println!("x: {}, y:{}, z{}", x, y, z);
}
*/

//call_http
//fn call_http(url: String) {
fn call_http(t: &TeacherInfo) {
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
            println!("Response: {}", res.status());
            println!("Headers: \n{}", res.headers());

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
