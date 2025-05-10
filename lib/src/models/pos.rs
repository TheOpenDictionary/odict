use crate::serializable_enum;

serializable_enum! {
    pub enum PartOfSpeech {
        /* -------------------------------------------------------------------------- */
        /*                            Japanese-specific POS                           */
        /* -------------------------------------------------------------------------- */
        #[strum(to_string = "pre-noun adjectival (rentaishi)")]
        AdjPn,
        #[strum(to_string = "'kari' adjective (archaic)")]
        AdjKari,
        #[strum(to_string = "article")]
        Art,
        #[strum(to_string = "'ku' adjective (archaic)")]
        AdjKu,
        #[strum(to_string = "archaic/formal form of na-adjective")]
        AdjNari,
        #[strum(to_string = "adjectival nouns or quasi-adjectives (keiyodoshi)")]
        AdjNa,
        #[strum(to_string = "'shiku' adjective (archaic)")]
        AdjShiku,
        #[strum(to_string = "'taru' adjective")]
        AdjT,
        #[strum(to_string = "adjective (keiyoushi) - yoi/ii class")]
        AdjIx,
        #[strum(to_string = "adverbial noun (fukushitekimeishi)")]
        NAdv,
        #[strum(to_string = "adverb taking the 'to' particle")]
        AdvTo,
        #[strum(to_string = "nouns which may take the genitive case particle 'no'")]
        AdjNo,
        #[strum(to_string = "noun, used as a prefix")]
        NPref,
        #[strum(to_string = "noun, used as a suffix")]
        NSuf,
        #[strum(to_string = "noun (temporal) (jisoumeishi)")]
        NT,
        #[strum(to_string = "noun or verb acting prenominally")]
        AdjF,
        #[strum(to_string = "Godan verb with 'bu' ending")]
        V5b,
        #[strum(to_string = "Godan verb with 'gu' ending")]
        V5g,
        #[strum(to_string = "Godan verb with 'ku' ending")]
        V5k,
        #[strum(to_string = "Godan verb with 'mu' ending")]
        V5m,
        #[strum(to_string = "Godan verb with 'nu' ending")]
        V5n,
        #[strum(to_string = "Godan verb with 'ru' ending")]
        V5r,
        #[strum(to_string = "Godan verb with 'ru' ending (irregular verb)")]
        V5rI,
        #[strum(to_string = "Godan verb - -aru special class")]
        V5aru,
        #[strum(to_string = "Godan verb - Iku/Yuku special class")]
        V5kS,
        #[strum(to_string = "Godan verb with 'su' ending")]
        V5s,
        #[strum(to_string = "Godan verb with 'tsu' ending")]
        V5t,
        #[strum(to_string = "Godan verb with 'u' ending")]
        V5u,
        #[strum(to_string = "Godan verb - Uru old class verb (old form of Eru)")]
        V5uru,
        #[strum(to_string = "Godan verb with 'u' ending (special class)")]
        V5uS,
        #[strum(to_string = "Ichidan verb")]
        V1,
        #[strum(to_string = "Ichidan verb - kureru special class")]
        V1S,
        #[strum(to_string = "Ichidan verb - zuru verb (alternative form of -jiru verbs)")]
        Vz,
        #[strum(to_string = "Kuru verb - special class")]
        Vk,
        #[strum(to_string = "Nidan verb (lower class) with 'bu' ending (archaic)")]
        V2bS,
        #[strum(to_string = "Nidan verb (upper class) with 'bu' ending (archaic)")]
        V2bK,
        #[strum(to_string = "Nidan verb (lower class) with 'dzu' ending (archaic)")]
        V2dS,
        #[strum(to_string = "Nidan verb (upper class) with 'dzu' ending (archaic)")]
        V2dK,
        #[strum(to_string = "Nidan verb (lower class) with 'gu' ending (archaic)")]
        V2gS,
        #[strum(to_string = "Nidan verb (upper class) with 'gu' ending (archaic)")]
        V2gK,
        #[strum(to_string = "Nidan verb (lower class) with 'hu/fu' ending (archaic)")]
        V2hS,
        #[strum(to_string = "Nidan verb (upper class) with 'hu/fu' ending (archaic)")]
        V2hK,
        #[strum(to_string = "Nidan verb (lower class) with 'ku' ending (archaic)")]
        V2kS,
        #[strum(to_string = "Nidan verb (upper class) with 'ku' ending (archaic)")]
        V2kK,
        #[strum(to_string = "Nidan verb (lower class) with 'mu' ending (archaic)")]
        V2mS,
        #[strum(to_string = "Nidan verb (upper class) with 'mu' ending (archaic)")]
        V2mK,
        #[strum(to_string = "Nidan verb (lower class) with 'nu' ending (archaic)")]
        V2nS,
        #[strum(to_string = "Nidan verb (lower class) with 'ru' ending (archaic)")]
        V2rS,
        #[strum(to_string = "Nidan verb (upper class) with 'ru' ending (archaic)")]
        V2rK,
        #[strum(to_string = "Nidan verb (lower class) with 'su' ending (archaic)")]
        V2sS,
        #[strum(to_string = "Nidan verb (lower class) with 'tsu' ending (archaic)")]
        V2tS,
        #[strum(to_string = "Nidan verb (upper class) with 'tsu' ending (archaic)")]
        V2tK,
        #[strum(to_string = "Nidan verb with 'u' ending (archaic)")]
        V2aS,
        #[strum(to_string = "Nidan verb (lower class) with 'u' ending and 'we' conjugation (archaic)")]
        V2wS,
        #[strum(to_string = "Nidan verb (lower class) with 'yu' ending (archaic)")]
        V2yS,
        #[strum(to_string = "Nidan verb (upper class) with 'yu' ending (archaic)")]
        V2yK,
        #[strum(to_string = "Nidan verb (lower class) with 'zu' ending (archaic)")]
        V2zS,
        #[strum(to_string = "irregular nu verb")]
        Vn,
        #[strum(to_string = "irregular ru verb, plain form ends with -ri")]
        Vr,
        #[strum(to_string = "su verb - precursor to the modern suru")]
        VsC,
        #[strum(to_string = "noun or participle which takes the aux. verb suru")]
        Vs,
        #[strum(to_string = "suru verb - included")]
        VsI,
        #[strum(to_string = "suru verb - special class")]
        VsS,
        #[strum(to_string = "verb unspecified")]
        VUnspec,
        #[strum(to_string = "Yodan verb with 'bu' ending (archaic)")]
        V4b,
        #[strum(to_string = "Yodan verb with 'gu' ending (archaic)")]
        V4g,
        #[strum(to_string = "Yodan verb with 'hu/fu' ending (archaic)")]
        V4h,
        #[strum(to_string = "Yodan verb with 'ku' ending (archaic)")]
        V4k,
        #[strum(to_string = "Yodan verb with 'mu' ending (archaic)")]
        V4m,
        #[strum(to_string = "Yodan verb with 'nu' ending (archaic)")]
        V4n,
        #[strum(to_string = "Yodan verb with 'ru' ending (archaic)")]
        V4r,
        #[strum(to_string = "Yodan verb with 'su' ending (archaic)")]
        V4s,
        #[strum(to_string = "Yodan verb with 'tsu' ending (archaic)")]
        V4t,

        /* -------------------------------------------------------------------------- */
        /*                                Universal POS                               */
        /* -------------------------------------------------------------------------- */
        #[strum(to_string = "abbreviation")]
        Abv,
        #[strum(to_string = "adfix")]
        Adf,
        #[strum(to_string = "adjective")]
        Adj,
        #[strum(to_string = "adjective phrase")]
        PhrAdj,
        #[strum(to_string = "adverb")]
        Adv,
        #[strum(to_string = "adverbial phrase")]
        PhrAdv,
        #[strum(to_string = "affix")]
        Aff,
        #[strum(to_string = "auxiliary")]
        Aux,
        #[strum(to_string = "auxiliary adjective")]
        AuxAdj,
        #[strum(to_string = "auxiliary verb")]
        AuxV,
        #[strum(to_string = "character")]
        Chr,
        #[strum(to_string = "circumfix")]
        Cf,
        #[strum(to_string = "classifier")]
        Cls,
        #[strum(to_string = "conjunction")]
        Conj,
        #[strum(to_string = "coordinating conjunction")]
        ConjC,
        #[strum(to_string = "contraction")]
        Contr,
        #[strum(to_string = "copula")]
        Cop,
        #[strum(to_string = "counter")]
        Ctr,
        #[strum(to_string = "determiner")]
        Det,
        #[strum(to_string = "expression")]
        Expr,
        #[strum(to_string = "infix")]
        Inf,
        #[strum(to_string = "interfix")]
        Intf,
        #[strum(to_string = "interjection")]
        Intj,
        #[strum(to_string = "intransitive verb")]
        Vi,
        #[strum(to_string = "name")]
        Name,
        #[strum(to_string = "noun")]
        N,
        #[strum(to_string = "numeric")]
        Num,
        #[strum(to_string = "particle")]
        Part,
        #[strum(to_string = "phrase")]
        Phr,
        #[strum(to_string = "postposition")]
        Postp,
        #[strum(to_string = "prefix")]
        Pref,
        #[strum(to_string = "preposition")]
        Prep,
        #[strum(to_string = "prepositional phrase")]
        PhrPrep,
        #[strum(to_string = "pronoun")]
        Pron,
        #[strum(to_string = "proper noun")]
        Propn,
        #[strum(to_string = "proverb")]
        Prov,
        #[strum(to_string = "punctuation")]
        Punc,
        #[strum(to_string = "subordinating conjunction")]
        ConjS,
        #[strum(to_string = "suffix")]
        Suff,
        #[strum(to_string = "symbol")]
        Sym,
        #[strum(to_string = "transitive verb")]
        Vt,
        #[strum(to_string = "unknown")]
        Un,
        #[strum(to_string = "verb")]
        V,
        /* -------------------------------------------------------------------------- */
        /*                                Custom POS                                  */
        /* -------------------------------------------------------------------------- */
        #[strum(to_string = "{0}")]
        #[serde(untagged)]
        Other(String),
    }
}

impl Default for PartOfSpeech {
    fn default() -> Self {
        PartOfSpeech::Un
    }
}
