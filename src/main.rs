extern crate iron;
extern crate staticfile;
extern crate mount;

use std::path::Path;
use iron::Iron;
use staticfile::Static;
use mount::Mount;

fn main() {
    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("")));
    println!("Serve running on http://localhost:8080/");

    Iron::new(mount).http("0.0.0.0:8080").unwrap();
}
