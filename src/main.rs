#![feature(append, result_expect)]
#![deny(unused)]

extern crate csv;
extern crate rustc_serialize;
extern crate toml;

mod error;
mod races;
mod spells;

use error::YAPCCResult;
use races::PartialRaces;
use spells::SpellBook;

fn do_main() -> YAPCCResult<()> {
    let book = try!(SpellBook::new("spells.csv"));
    println!("{:#?}", book.spells.get("Magic Missile"));
    let races = try!(PartialRaces::new("races.toml"));
    let race = races.choose();
    println!("{}", toml::encode_str(&race));
    println!("{}", toml::encode_str(&book.spells.get("Magic Missile")));
    Ok(())
}

fn main() {
    match do_main() {
        Err(e) => {
            println!("Error: {}", e);
            return;
        },
        _ => {}
    }
}
