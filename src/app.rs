extern crate lib;
use lib::arith;

extern crate serde;
extern crate serde_json;

//extern crate serde_derive;

//use serde_json::Error;

fn main() {
    println!("Hello, world!");
    let term = arith::Term::TmSucc {term: Box::new(arith::Term::TmZero {})};

    let j = serde_json::to_string(&term);
    match j {
        Ok(x) => println!("{}", x),
        Err(_) => (),
    }
}
