extern crate futures;
extern crate project2;

use futures::prelude::*;

pub fn addone(
    f: impl Future<Item = u32, Error = ()> + Send,
) -> impl Future<Item = u32, Error = ()> + Send {
    f.map(|v| project2::add(v, 1))
}
