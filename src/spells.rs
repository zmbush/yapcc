use csv;
use error::YAPCCResult;
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::Path;

macro_rules! gen_spell_traits {
    ($($name:ident),+) => {
        #[derive(Debug, Clone, RustcEncodable)]
        pub struct SpellTraits {
            pub $($name: bool),*
        }

        impl Decodable for SpellTraits {
            fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
                Ok(SpellTraits {$($name: try!(d.read_str()) == "x"),*})
            }
        }
    }
}

gen_spell_traits! { ritual, verbal, somatic, material, concentration,
                    bard, cleric, druid, paladin, ranger, sorcerer, warlock, wizard }

#[derive(Debug, RustcDecodable, RustcEncodable, Clone)]
pub struct Spell {
    pub name: String,
    pub page: u32,
    pub level: u8,
    pub school: String,
    pub casting_time: String,
    pub traits: SpellTraits
}

#[derive(Debug, RustcEncodable)]
pub struct SpellBook {
    pub spells: HashMap<String, Spell>
}

impl SpellBook {
    pub fn new<P: AsRef<OsStr>>(f: P) -> YAPCCResult<SpellBook> {
        let mut file = try!(File::open(Path::new(&f)));
        let mut cont = String::new();
        try!(file.read_to_string(&mut cont));
        let mut rdr = csv::Reader::from_string(cont).has_headers(true);
        Ok(SpellBook {
            spells: rdr.decode()
                .filter_map(|s| s.ok())
                .map(|s: Spell| (s.name.clone(), s))
                .collect()
        })
    }
}
