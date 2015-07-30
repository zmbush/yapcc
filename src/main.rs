#![feature(append, result_expect)]
#![deny(unused)]

extern crate csv;
extern crate rustc_serialize;
extern crate toml;

mod error;
mod races;
mod spells;

use error::YAPCCResult;
use races::{PartialRaces, PartialRace};
use spells::SpellBook;

use std::collections::HashMap;
use std::io::Write;

fn choose_race(sub: bool, races: &HashMap<String, PartialRace>) -> PartialRace {
    loop {
        print! {
            "Which {}race: {}? ",
            if sub { "sub" } else { "" },
            races.keys().fold(String::new(), |acc, item| {
                if acc.len() > 0 {
                    acc + ", " + item
                } else {
                    acc + item
                }
            })
        };
        std::io::stdout().flush().expect("Unable to flush to stdout");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Unable to read input");
        if races.contains_key(choice.trim()) {
            return races[choice.trim()].clone();
        }
    }
}

fn do_main() -> YAPCCResult<()> {
    let book = try!(SpellBook::new("spells.csv"));
    println!("{:#?}", book.spells.get("Magic Missile"));
    let races = try!(PartialRaces::new("races.toml"));
    let race = choose_race(false, &races.races);
    let subrace = race.subraces.as_ref().map(|sr| choose_race(true, &sr)).unwrap_or(Default::default()).clone();
    println!("{}", toml::encode_str(&race.solidify(subrace)));
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
