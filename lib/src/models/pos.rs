use std::str::FromStr;

use crate::serializable_enum;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum::EnumProperty;

serializable_enum! {
    #[derive(strum::EnumProperty)]
    pub enum PartOfSpeech {
        /* -------------------------------------------------------------------------- */
        /*                            Japanese-specific POS                           */
        /* -------------------------------------------------------------------------- */
        #[strum(props(description = "pre-noun adjectival (rentaishi)"))]
        AdjPn,
        #[strum(props(description = "'kari' adjective (archaic)"))]
        AdjKari,
        #[strum(props(description = "article"))]
        Art,
        #[strum(props(description = "'ku' adjective (archaic)"))]
        AdjKu,
        #[strum(props(description = "archaic/formal form of na-adjective"))]
        AdjNari,
        #[strum(props(description = "adjectival nouns or quasi-adjectives (keiyodoshi)"))]
        AdjNa,
        #[strum(props(description = "'shiku' adjective (archaic)"))]
        AdjShiku,
        #[strum(props(description = "'taru' adjective"))]
        AdjT,
        #[strum(props(description = "adjective (keiyoushi) - yoi/ii class"))]
        AdjIx,
        #[strum(props(description = "adverbial noun (fukushitekimeishi)"))]
        NAdv,
        #[strum(props(description = "adverb taking the 'to' particle"))]
        AdvTo,
        #[strum(props(description = "nouns which may take the genitive case particle 'no'"))]
        AdjNo,
        #[strum(props(description = "noun, used as a prefix"))]
        NPref,
        #[strum(props(description = "noun, used as a suffix"))]
        NSuf,
        #[strum(props(description = "noun (temporal) (jisoumeishi)"))]
        NT,
        #[strum(props(description = "noun or verb acting prenominally"))]
        AdjF,
        #[strum(props(description = "Godan verb with 'bu' ending"))]
        V5b,
        #[strum(props(description = "Godan verb with 'gu' ending"))]
        V5g,
        #[strum(props(description = "Godan verb with 'ku' ending"))]
        V5k,
        #[strum(props(description = "Godan verb with 'mu' ending"))]
        V5m,
        #[strum(props(description = "Godan verb with 'nu' ending"))]
        V5n,
        #[strum(props(description = "Godan verb with 'ru' ending"))]
        V5r,
        #[strum(props(description = "Godan verb with 'ru' ending (irregular verb)"))]
        V5rI,
        #[strum(props(description = "Godan verb - -aru special class"))]
        V5aru,
        #[strum(props(description = "Godan verb - Iku/Yuku special class"))]
        V5kS,
        #[strum(props(description = "Godan verb with 'su' ending"))]
        V5s,
        #[strum(props(description = "Godan verb with 'tsu' ending"))]
        V5t,
        #[strum(props(description = "Godan verb with 'u' ending"))]
        V5u,
        #[strum(props(description = "Godan verb - Uru old class verb (old form of Eru)"))]
        V5uru,
        #[strum(props(description = "Godan verb with 'u' ending (special class)"))]
        V5uS,
        #[strum(props(description = "Ichidan verb"))]
        V1,
        #[strum(props(description = "Ichidan verb - kureru special class"))]
        V1S,
        #[strum(props(description = "Ichidan verb - zuru verb (alternative form of -jiru verbs)"))]
        Vz,
        #[strum(props(description = "Kuru verb - special class"))]
        Vk,
        #[strum(props(description = "Nidan verb (lower class) with 'bu' ending (archaic)"))]
        V2bS,
        #[strum(props(description = "Nidan verb (upper class) with 'bu' ending (archaic)"))]
        V2bK,
        #[strum(props(description = "Nidan verb (lower class) with 'dzu' ending (archaic)"))]
        V2dS,
        #[strum(props(description = "Nidan verb (upper class) with 'dzu' ending (archaic)"))]
        V2dK,
        #[strum(props(description = "Nidan verb (lower class) with 'gu' ending (archaic)"))]
        V2gS,
        #[strum(props(description = "Nidan verb (upper class) with 'gu' ending (archaic)"))]
        V2gK,
        #[strum(props(description = "Nidan verb (lower class) with 'hu/fu' ending (archaic)"))]
        V2hS,
        #[strum(props(description = "Nidan verb (upper class) with 'hu/fu' ending (archaic)"))]
        V2hK,
        #[strum(props(description = "Nidan verb (lower class) with 'ku' ending (archaic)"))]
        V2kS,
        #[strum(props(description = "Nidan verb (upper class) with 'ku' ending (archaic)"))]
        V2kK,
        #[strum(props(description = "Nidan verb (lower class) with 'mu' ending (archaic)"))]
        V2mS,
        #[strum(props(description = "Nidan verb (upper class) with 'mu' ending (archaic)"))]
        V2mK,
        #[strum(props(description = "Nidan verb (lower class) with 'nu' ending (archaic)"))]
        V2nS,
        #[strum(props(description = "Nidan verb (lower class) with 'ru' ending (archaic)"))]
        V2rS,
        #[strum(props(description = "Nidan verb (upper class) with 'ru' ending (archaic)"))]
        V2rK,
        #[strum(props(description = "Nidan verb (lower class) with 'su' ending (archaic)"))]
        V2sS,
        #[strum(props(description = "Nidan verb (lower class) with 'tsu' ending (archaic)"))]
        V2tS,
        #[strum(props(description = "Nidan verb (upper class) with 'tsu' ending (archaic)"))]
        V2tK,
        #[strum(props(description = "Nidan verb with 'u' ending (archaic)"))]
        V2aS,
        #[strum(props(description = "Nidan verb (lower class) with 'u' ending and 'we' conjugation (archaic)"))]
        V2wS,
        #[strum(props(description = "Nidan verb (lower class) with 'yu' ending (archaic)"))]
        V2yS,
        #[strum(props(description = "Nidan verb (upper class) with 'yu' ending (archaic)"))]
        V2yK,
        #[strum(props(description = "Nidan verb (lower class) with 'zu' ending (archaic)"))]
        V2zS,
        #[strum(props(description = "irregular nu verb"))]
        Vn,
        #[strum(props(description = "irregular ru verb, plain form ends with -ri"))]
        Vr,
        #[strum(props(description = "su verb - precursor to the modern suru"))]
        VsC,
        #[strum(props(description = "noun or participle which takes the aux. verb suru"))]
        Vs,
        #[strum(props(description = "suru verb - included"))]
        VsI,
        #[strum(props(description = "suru verb - special class"))]
        VsS,
        #[strum(props(description = "verb unspecified"))]
        VUnspec,
        #[strum(props(description = "Yodan verb with 'bu' ending (archaic)"))]
        V4b,
        #[strum(props(description = "Yodan verb with 'gu' ending (archaic)"))]
        V4g,
        #[strum(props(description = "Yodan verb with 'hu/fu' ending (archaic)"))]
        V4h,
        #[strum(props(description = "Yodan verb with 'ku' ending (archaic)"))]
        V4k,
        #[strum(props(description = "Yodan verb with 'mu' ending (archaic)"))]
        V4m,
        #[strum(props(description = "Yodan verb with 'nu' ending (archaic)"))]
        V4n,
        #[strum(props(description = "Yodan verb with 'ru' ending (archaic)"))]
        V4r,
        #[strum(props(description = "Yodan verb with 'su' ending (archaic)"))]
        V4s,
        #[strum(props(description = "Yodan verb with 'tsu' ending (archaic)"))]
        V4t,

        /* -------------------------------------------------------------------------- */
        /*                                Universal POS                               */
        /* -------------------------------------------------------------------------- */
        #[strum(props(description = "abbreviation"))]
        Abv,
        #[strum(props(description = "adfix"))]
        Adf,
        #[strum(props(description = "adjective"))]
        Adj,
        #[strum(props(description = "adjective phrase"))]
        PhrAdj,
        #[strum(props(description = "adverb"))]
        Adv,
        #[strum(props(description = "adverbial phrase"))]
        PhrAdv,
        #[strum(props(description = "affix"))]
        Aff,
        #[strum(props(description = "auxiliary"))]
        Aux,
        #[strum(props(description = "auxiliary adjective"))]
        AuxAdj,
        #[strum(props(description = "auxiliary verb"))]
        AuxV,
        #[strum(props(description = "character"))]
        Chr,
        #[strum(props(description = "circumfix"))]
        Cf,
        #[strum(props(description = "classifier"))]
        Cls,
        #[strum(props(description = "conjunction"))]
        Conj,
        #[strum(props(description = "coordinating conjunction"))]
        ConjC,
        #[strum(props(description = "contraction"))]
        Contr,
        #[strum(props(description = "copula"))]
        Cop,
        #[strum(props(description = "counter"))]
        Ctr,
        #[strum(props(description = "determiner"))]
        Det,
        #[strum(props(description = "expression"))]
        Expr,
        #[strum(props(description = "infix"))]
        Inf,
        #[strum(props(description = "interfix"))]
        Intf,
        #[strum(props(description = "interjection"))]
        Intj,
        #[strum(props(description = "intransitive verb"))]
        Vi,
        #[strum(props(description = "name"))]
        Name,
        #[strum(props(description = "noun"))]
        N,
        #[strum(props(description = "numeric"))]
        Num,
        #[strum(props(description = "particle"))]
        Part,
        #[strum(props(description = "phrase"))]
        Phr,
        #[strum(props(description = "postposition"))]
        Postp,
        #[strum(props(description = "prefix"))]
        Pref,
        #[strum(props(description = "preposition"))]
        Prep,
        #[strum(props(description = "prepositional phrase"))]
        PhrPrep,
        #[strum(props(description = "pronoun"))]
        Pron,
        #[strum(props(description = "proper noun"))]
        Propn,
        #[strum(props(description = "proverb"))]
        Prov,
        #[strum(props(description = "punctuation"))]
        Punc,
        #[strum(props(description = "subordinating conjunction"))]
        ConjS,
        #[strum(props(description = "suffix"))]
        Suff,
        #[strum(props(description = "symbol"))]
        Sym,
        #[strum(props(description = "transitive verb"))]
        Vt,
        #[strum(props(description = "unknown"))]
        Un,
        #[strum(props(description = "verb"))]
        V,
        /* -------------------------------------------------------------------------- */
        /*                                Custom POS                                  */
        /* -------------------------------------------------------------------------- */
        Other(String),
    }
}

impl Default for PartOfSpeech {
    fn default() -> Self {
        PartOfSpeech::Un
    }
}

impl From<PartOfSpeech> for String {
    fn from(pos: PartOfSpeech) -> Self {
        match pos {
            PartOfSpeech::Other(ref s) => s.to_owned(),
            _ => pos.to_string(),
        }
    }
}

impl PartOfSpeech {
    pub fn description(&self) -> &str {
        match self {
            PartOfSpeech::Other(ref s) => s,
            _ => self.get_str("description").unwrap_or(""),
        }
    }
}

impl Serialize for PartOfSpeech {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(String::from(self.clone()).as_str())
    }
}

impl<'de> Deserialize<'de> for PartOfSpeech {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(PartOfSpeech::from_str(s.as_str()).unwrap_or(PartOfSpeech::Other(s.to_string())))
    }
}
