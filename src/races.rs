use error::{YAPCCError, YAPCCResult};
use rustc_serialize::{Decodable, Encodable, Decoder, Encoder};
use toml;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::ops::Add;
use std::path::Path;

type Score = u8;
#[derive(Debug, Clone, Default)]
pub struct AbilityScores {
    strength: Score,
    dexterity: Score,
    constitution: Score,
    intelligence: Score,
    wisdom: Score,
    charisma: Score,
}

impl Add for AbilityScores {
    type Output = AbilityScores;

    fn add(self, rhs: AbilityScores) -> AbilityScores {
        macro_rules! gen_score {
            ($($name:ident),*) => {
                AbilityScores {$(
                    $name: self.$name + rhs.$name
                ),*}
            }
        }

        gen_score![strength, dexterity, constitution, intelligence, wisdom, charisma]
    }
}

impl Decodable for AbilityScores {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        let mut abilities: HashMap<String,Result<u8, D::Error>> = try!(d.read_str())
            .split(";")
            .map(|e| {
                let mut i = e.split("+");
                let name = i.next().unwrap_or(e).to_owned();
                let value = i.next()
                    .ok_or(d.error("Unable to parse abilities"))
                    .and_then(|v| v.parse().map_err(|_| d.error("unable to parse")));
                (name, value)
            })
            .collect();
        macro_rules! get_score {
            ($name:ident) => (match abilities.remove(stringify!($name)) {
                Some(v) => try!(v).clone(),
                None => 0
            })
        }
        Ok(AbilityScores {
            strength: get_score!(str),
            dexterity: get_score!(dex),
            constitution: get_score!(con),
            intelligence: get_score!(int),
            wisdom: get_score!(wis),
            charisma: get_score!(cha)
        })
    }
}

impl Encodable for AbilityScores {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        let mut val = String::new();
        macro_rules! write_scores {
            ($($from:ident => $to:ident),+) => ({
                $(if self.$from > 0 {
                    let stat = format!("{}+{}", stringify!($to), self.$from);
                    if val.len() > 0 {
                        val = val + ";" + &stat;
                    } else {
                        val = stat;
                    }
                })*
            })
        }

        write_scores! {
            strength => str,
            dexterity => dex,
            constitution => con,
            intelligence => int,
            wisdom => wis,
            charisma => cha
        };

        try!(e.emit_str(&val));
        Ok(())
    }
}

#[derive(Debug, RustcDecodable, RustcEncodable, Clone)]
pub struct Trait {
    pub name: String,
    pub desc: String
}

#[derive(Debug, RustcDecodable, RustcEncodable, Clone, Default)]
pub struct PartialRace {
    pub speed: Option<u8>,
    pub abilities: AbilityScores,
    pub subraces: Option<HashMap<String, PartialRace>>,
    pub hp_bonus: Option<u8>,
    pub page: Option<u16>,
    pub traits: Vec<Trait>,

    pub weapon_proficiencies: Vec<String>,
    pub tool_proficiencies: Vec<String>,
    pub armor_proficiencies: Vec<String>,
}

#[derive(Debug, RustcDecodable, RustcEncodable, Clone)]
pub struct Race {
    pub speed: u8,
    pub abilities: AbilityScores,
    pub hp_bonus: Option<u8>,
    pub page: Option<u16>,
    pub traits: Vec<Trait>,

    pub weapon_proficiencies: Vec<String>,
    pub tool_proficiencies: Vec<String>,
    pub armor_proficiencies: Vec<String>,
}

impl PartialRace {
    pub fn solidify(self, subrace: PartialRace) -> Race {
        macro_rules! merge {
            ($name:ident) => {{
                let mut tmp = self.$name.clone();
                tmp.append(&mut subrace.$name.clone());
                tmp
            }}
        }
        Race {
            speed: subrace.speed.or(self.speed).expect("Either subrace or race need a speed"),
            abilities: subrace.abilities + self.abilities,
            hp_bonus: subrace.hp_bonus.or(self.hp_bonus),
            page: subrace.page.or(self.page),
            traits: merge!(traits),
            weapon_proficiencies: merge!(weapon_proficiencies),
            tool_proficiencies: merge!(tool_proficiencies),
            armor_proficiencies: merge!(armor_proficiencies)
        }
    }
}

#[derive(Debug)]
pub struct PartialRaces {
    pub races: HashMap<String, PartialRace>
}

impl PartialRaces {
    pub fn new<P: AsRef<OsStr>>(f: P) -> YAPCCResult<PartialRaces> {
        let mut file = try!(File::open(Path::new(&f)));
        let mut cont = String::new();
        try!(file.read_to_string(&mut cont));
        let mut parser = toml::Parser::new(&cont);
        match parser.parse() {
            Some(res) => return Ok(PartialRaces {
                races: try!(Decodable::decode(&mut toml::Decoder::new(toml::Value::Table(res))))
            }),
            None => { }
        }

        println!("{:?}", parser.errors);
        Err(YAPCCError::GenericError)
    }
}
