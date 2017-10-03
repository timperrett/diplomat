
use std::borrow::{Borrow, Cow};
use std::io::{stdout, Write};
use curl::easy::Easy;

pub struct Client {
  // pub catalog: Catalog,
}

impl Client {
    pub fn new<'a, S>(address: S) -> Client where S: Into<Cow<'a, str>> {
        let cow = address.into();
        // let addr = cow.borrow();
        // let catalog = Catalog::new(addr);
        Client {

        }
    }
}

// pub struct Catalog {

// }

// impl Catalog {

// }




// Write the contents of rust-lang.org to stdout
// let mut easy = Easy::new();
// easy.url("https://www.rust-lang.org/").unwrap();
// easy.write_function(|data| {
//     Ok(stdout().write(data).unwrap())
// }).unwrap();
// easy.perform().unwrap();

