use std::fmt;

use crate::{case::SnakeCase, serializable};

serializable! {
    #[derive(Hash, Ord, PartialOrd)]
    #[rkyv(derive(PartialEq, Eq, Hash))]
    #[repr(u8)]
    #[serde(rename_all = "snake_case")]
    pub enum PartOfSpeech {
        /* -------------------------------------------------------------------------- */
        /*                            Japanese-specific POS                           */
        /* -------------------------------------------------------------------------- */
        AdjPn,
        AdjKari,
        Art,
        AdjKu,
        AdjNari,
        AdjNa,
        AdjShiku,
        AdjT,
        AdjIx,
        NAdv,
        AdvTo,
        AdjNo,
        NPref,
        NSuf,
        NT,
        AdjF,
        V5b,
        V5g,
        V5k,
        V5m,
        V5n,
        V5r,
        V5rI,
        V5aru,
        V5kS,
        V5s,
        V5t,
        V5u,
        V5uru,
        V5uS,
        V1,
        V1S,
        Vz,
        Vk,
        V2bS,
        V2bK,
        V2dS,
        V2dK,
        V2gS,
        V2gK,
        V2hS,
        V2hK,
        V2kS,
        V2kK,
        V2mS,
        V2mK,
        V2nS,
        V2rS,
        V2rK,
        V2sS,
        V2tS,
        V2tK,
        V2aS,
        V2wS,
        V2yS,
        V2yK,
        V2zS,
        Vn,
        Vr,
        VsC,
        Vs,
        VsI,
        VsS,
        VUnspec,
        V4b,
        V4g,
        V4h,
        V4k,
        V4m,
        V4n,
        V4r,
        V4s,
        V4t,

        /* -------------------------------------------------------------------------- */
        /*                                Universal POS                               */
        /* -------------------------------------------------------------------------- */
        Abv,
        Adf,
        Adj,
        PhrAdj,
        Adv,
        PhrAdv,
        Aff,
        Aux,
        AuxAdj,
        AuxV,
        Chr,
        Cf,
        Cls,
        Contr,
        Conj,
        ConjC,
        Cop,
        Ctr,
        Det,
        Expr,
        Inf,
        Intf,
        Intj,
        Vi,
        Name,
        N,
        Num,
        Part,
        Phr,
        Postp,
        Pref,
        Prep,
        PhrPrep,
        Pron,
        Propn,
        Prov,
        Punc,
        ConjS,
        Suff,
        Sym,
        Vt,
        Un,
        V,
        /* -------------------------------------------------------------------------- */
        /*                                Custom POS                                  */
        /* -------------------------------------------------------------------------- */
        Other(String),
    }
}

impl From<PartOfSpeech> for String {
    fn from(pos: PartOfSpeech) -> Self {
        match pos {
            PartOfSpeech::Other(s) => s,
            _ => format!("{:?}", pos).snake_case(),
        }
    }
}

impl fmt::Display for PartOfSpeech {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PartOfSpeech::Other(s) => write!(f, "{}", s),
            _ => write!(f, "{}", format!("{:?}", self).snake_case()),
        }
    }
}

impl Default for PartOfSpeech {
    fn default() -> Self {
        PartOfSpeech::Un
    }
}

impl PartOfSpeech {
    pub fn description(&self) -> &str {
        match *self {
            /* -------------------------------------------------------------------------- */
            /*                            Japanese-specific POS                           */
            /* -------------------------------------------------------------------------- */
            PartOfSpeech::AdjPn => "pre-noun adjectival (rentaishi)",
            PartOfSpeech::AdjKari => "'kari' adjective (archaic)",
            PartOfSpeech::Art => "article",
            PartOfSpeech::AdjKu => "'ku' adjective (archaic)",
            PartOfSpeech::AdjNari => "archaic/formal form of na-adjective",
            PartOfSpeech::AdjNa => "adjectival nouns or quasi-adjectives (keiyodoshi)",
            PartOfSpeech::AdjShiku => "'shiku' adjective (archaic)",
            PartOfSpeech::AdjT => "'taru' adjective",
            PartOfSpeech::AdjIx => "adjective (keiyoushi) - yoi/ii class",
            PartOfSpeech::NAdv => "adverbial noun (fukushitekimeishi)",
            PartOfSpeech::AdvTo => "adverb taking the 'to' particle",
            PartOfSpeech::AdjNo => "nouns which may take the genitive case particle 'no'",
            PartOfSpeech::NPref => "noun, used as a prefix",
            PartOfSpeech::NSuf => "noun, used as a suffix",
            PartOfSpeech::NT => "noun (temporal) (jisoumeishi)",
            PartOfSpeech::AdjF => "noun or verb acting prenominally",
            PartOfSpeech::V5b => "Godan verb with 'bu' ending",
            PartOfSpeech::V5g => "Godan verb with 'gu' ending",
            PartOfSpeech::V5k => "Godan verb with 'ku' ending",
            PartOfSpeech::V5m => "Godan verb with 'mu' ending",
            PartOfSpeech::V5n => "Godan verb with 'nu' ending",
            PartOfSpeech::V5r => "Godan verb with 'ru' ending",
            PartOfSpeech::V5rI => "Godan verb with 'ru' ending (irregular verb)",
            PartOfSpeech::V5aru => "Godan verb - -aru special class",
            PartOfSpeech::V5kS => "Godan verb - Iku/Yuku special class",
            PartOfSpeech::V5s => "Godan verb with 'su' ending",
            PartOfSpeech::V5t => "Godan verb with 'tsu' ending",
            PartOfSpeech::V5u => "Godan verb with 'u' ending",
            PartOfSpeech::V5uru => "Godan verb - Uru old class verb (old form of Eru)",
            PartOfSpeech::V5uS => "Godan verb with 'u' ending (special class)",
            PartOfSpeech::V1 => "Ichidan verb",
            PartOfSpeech::V1S => "Ichidan verb - kureru special class",
            PartOfSpeech::Vz => "Ichidan verb - zuru verb (alternative form of -jiru verbs)",
            PartOfSpeech::Vk => "Kuru verb - special class",
            PartOfSpeech::V2bS => "Nidan verb (lower class) with 'bu' ending (archaic)",
            PartOfSpeech::V2bK => "Nidan verb (upper class) with 'bu' ending (archaic)",
            PartOfSpeech::V2dS => "Nidan verb (lower class) with 'dzu' ending (archaic)",
            PartOfSpeech::V2dK => "Nidan verb (upper class) with 'dzu' ending (archaic)",
            PartOfSpeech::V2gS => "Nidan verb (lower class) with 'gu' ending (archaic)",
            PartOfSpeech::V2gK => "Nidan verb (upper class) with 'gu' ending (archaic)",
            PartOfSpeech::V2hS => "Nidan verb (lower class) with 'hu/fu' ending (archaic)",
            PartOfSpeech::V2hK => "Nidan verb (upper class) with 'hu/fu' ending (archaic)",
            PartOfSpeech::V2kS => "Nidan verb (lower class) with 'ku' ending (archaic)",
            PartOfSpeech::V2kK => "Nidan verb (upper class) with 'ku' ending (archaic)",
            PartOfSpeech::V2mS => "Nidan verb (lower class) with 'mu' ending (archaic)",
            PartOfSpeech::V2mK => "Nidan verb (upper class) with 'mu' ending (archaic)",
            PartOfSpeech::V2nS => "Nidan verb (lower class) with 'nu' ending (archaic)",
            PartOfSpeech::V2rS => "Nidan verb (lower class) with 'ru' ending (archaic)",
            PartOfSpeech::V2rK => "Nidan verb (upper class) with 'ru' ending (archaic)",
            PartOfSpeech::V2sS => "Nidan verb (lower class) with 'su' ending (archaic)",
            PartOfSpeech::V2tS => "Nidan verb (lower class) with 'tsu' ending (archaic)",
            PartOfSpeech::V2tK => "Nidan verb (upper class) with 'tsu' ending (archaic)",
            PartOfSpeech::V2aS => "Nidan verb with 'u' ending (archaic)",
            PartOfSpeech::V2wS => {
                "Nidan verb (lower class) with 'u' ending and 'we' conjugation (archaic)"
            }
            PartOfSpeech::V2yS => "Nidan verb (lower class) with 'yu' ending (archaic)",
            PartOfSpeech::V2yK => "Nidan verb (upper class) with 'yu' ending (archaic)",
            PartOfSpeech::V2zS => "Nidan verb (lower class) with 'zu' ending (archaic)",
            PartOfSpeech::Vn => "irregular nu verb",
            PartOfSpeech::Vr => "irregular ru verb, plain form ends with -ri",
            PartOfSpeech::VsC => "su verb - precursor to the modern suru",
            PartOfSpeech::Vs => "noun or participle which takes the aux. verb suru",
            PartOfSpeech::VsI => "suru verb - included",
            PartOfSpeech::VsS => "suru verb - special class",
            PartOfSpeech::VUnspec => "verb unspecified",
            PartOfSpeech::V4b => "Yodan verb with 'bu' ending (archaic)",
            PartOfSpeech::V4g => "Yodan verb with 'gu' ending (archaic)",
            PartOfSpeech::V4h => "Yodan verb with 'hu/fu' ending (archaic)",
            PartOfSpeech::V4k => "Yodan verb with 'ku' ending (archaic)",
            PartOfSpeech::V4m => "Yodan verb with 'mu' ending (archaic)",
            PartOfSpeech::V4n => "Yodan verb with 'nu' ending (archaic)",
            PartOfSpeech::V4r => "Yodan verb with 'ru' ending (archaic)",
            PartOfSpeech::V4s => "Yodan verb with 'su' ending (archaic)",
            PartOfSpeech::V4t => "Yodan verb with 'tsu' ending (archaic)",

            /* -------------------------------------------------------------------------- */
            /*                                Universal POS                               */
            /* -------------------------------------------------------------------------- */
            PartOfSpeech::Abv => "abbreviation",
            PartOfSpeech::Adf => "adfix",
            PartOfSpeech::Adj => "adjective",
            PartOfSpeech::PhrAdj => "adjective phrase",
            PartOfSpeech::Adv => "adverb",
            PartOfSpeech::PhrAdv => "adverbial phrase",
            PartOfSpeech::Aff => "affix",
            PartOfSpeech::Aux => "auxiliary",
            PartOfSpeech::AuxAdj => "auxiliary adjective",
            PartOfSpeech::AuxV => "auxiliary verb",
            PartOfSpeech::Chr => "character",
            PartOfSpeech::Cf => "circumfix",
            PartOfSpeech::Cls => "classifier",
            PartOfSpeech::Conj => "conjunction",
            PartOfSpeech::ConjC => "coordinating conjunction",
            PartOfSpeech::Contr => "contraction",
            PartOfSpeech::Cop => "copula",
            PartOfSpeech::Ctr => "counter",
            PartOfSpeech::Det => "determiner",
            PartOfSpeech::Expr => "expression",
            PartOfSpeech::Inf => "infix",
            PartOfSpeech::Intf => "interfix",
            PartOfSpeech::Intj => "interjection",
            PartOfSpeech::Vi => "intransitive verb",
            PartOfSpeech::Name => "name",
            PartOfSpeech::N => "noun",
            PartOfSpeech::Num => "numeric",
            PartOfSpeech::Part => "particle",
            PartOfSpeech::Phr => "phrase",
            PartOfSpeech::Postp => "postposition",
            PartOfSpeech::Pref => "prefix",
            PartOfSpeech::Prep => "preposition",
            PartOfSpeech::PhrPrep => "prepositional phrase",
            PartOfSpeech::Pron => "pronoun",
            PartOfSpeech::Propn => "proper noun",
            PartOfSpeech::Prov => "proverb",
            PartOfSpeech::Punc => "punctuation",
            PartOfSpeech::ConjS => "subordinating conjunction",
            PartOfSpeech::Suff => "suffix",
            PartOfSpeech::Sym => "symbol",
            PartOfSpeech::Vt => "transitive verb",
            PartOfSpeech::Un => "unknown",
            PartOfSpeech::V => "verb",
            PartOfSpeech::Other(ref s) => s,
        }
    }
}
