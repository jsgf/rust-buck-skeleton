extern crate project1;
extern crate tokio;

use tokio::prelude::*;

fn main() {
    let one = future::lazy(|| Ok(1));
    let add = project1::addone(one);
    let done = add.and_then(|v| {
        println!("v={}", v);
        Ok(())
    });

    tokio::run(done);
}
