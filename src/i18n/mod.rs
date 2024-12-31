use std::collections::HashMap;

use serde::Deserialize;


#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Language {
    De
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Deserialize)]
pub enum Texts {
    Start,
    End,
    Remark,
    Export,

}

pub struct Translations {
    translations: HashMap<Language, HashMap<Texts, String>>
}

impl Translations {
    pub fn new() -> Self {
        let lang_de = include_str!("./de.json");
        let map_lang_de = serde_json::from_str(&lang_de).unwrap();
        Self {
            translations: HashMap::from([
                (Language::De, map_lang_de)
            ])
        }
    }
}