use std::fmt;

use crate::serializable;

macro_rules! create_pos {
  ( { $( $variant:ident, )* }) => {
      serializable! {
        #[derive(Hash, Ord, PartialOrd)]
        pub enum PartOfSpeech {
            $( $variant, )*
        }
      }

      pub const POS_TAGS: &[&str] = &[
          $( stringify!($variant), )*
      ];
  }
}

impl From<PartOfSpeech> for String {
    fn from(pos: PartOfSpeech) -> Self {
        return format!("{:?}", pos);
    }
}

impl fmt::Display for PartOfSpeech {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

create_pos!({
  /* -------------------------------------------------------------------------- */
  /*                            Japanese-specific POS                           */
  /* -------------------------------------------------------------------------- */
  adj_pn,
  adj_kari,
  art,
  adj_ku,
  adj_nari,
  adj_na,
  adj_shiku,
  adj_t,
  adj_ix,
  n_adv,
  adv_to,
  adj_no,
  n_pref,
  n_suf,
  n_t,
  adj_f,
  v5b,
  v5g,
  v5k,
  v5m,
  v5n,
  v5r,
  v5r_i,
  v5aru,
  v5k_s,
  v5s,
  v5t,
  v5u,
  v5uru,
  v5u_s,
  v1,
  v1_s,
  vz,
  vk,
  v2b_s,
  v2b_k,
  v2d_s,
  v2d_k,
  v2g_s,
  v2g_k,
  v2h_s,
  v2h_k,
  v2k_s,
  v2k_k,
  v2m_s,
  v2m_k,
  v2n_s,
  v2r_s,
  v2r_k,
  v2s_s,
  v2t_s,
  v2t_k,
  v2a_s,
  v2w_s,
  v2y_s,
  v2y_k,
  v2z_s,
  vn,
  vr,
  vs_c,
  vs,
  vs_i,
  vs_s,
  v_unspec,
  v4b,
  v4g,
  v4h,
  v4k,
  v4m,
  v4n,
  v4r,
  v4s,
  v4t,

  /* -------------------------------------------------------------------------- */
  /*                                Universal POS                               */
  /* -------------------------------------------------------------------------- */
  abv,
  adf,
  adj,
  phr_adj,
  adv,
  phr_adv,
  aff,
  aux,
  aux_adj,
  aux_v,
  chr,
  cf,
  cls,
  contr,
  conj,
  conj_c,
  cop,
  ctr,
  det,
  expr,
  inf,
  intf,
  intj,
  vi,
  name,
  n,
  num,
  part,
  phr,
  postp,
  pref,
  prep,
  phr_prep,
  pron,
  propn,
  prov,
  punc,
  conj_s,
  suff,
  sym,
  vt,
  un,
  v,
});

impl Default for PartOfSpeech {
    fn default() -> Self {
        PartOfSpeech::un
    }
}

impl PartOfSpeech {
    pub fn description(&self) -> &str {
        match *self {
            /* -------------------------------------------------------------------------- */
            /*                            Japanese-specific POS                           */
            /* -------------------------------------------------------------------------- */
            PartOfSpeech::adj_pn => "pre-noun adjectival (rentaishi)",
            PartOfSpeech::adj_kari => "'kari' adjective (archaic)",
            PartOfSpeech::adj_ku => "'ku' adjective (archaic)",
            PartOfSpeech::adj_nari => "archaic/formal form of na-adjective",
            PartOfSpeech::adj_na => "adjectival nouns or quasi-adjectives (keiyodoshi)",
            PartOfSpeech::adj_shiku => "'shiku' adjective (archaic)",
            PartOfSpeech::adj_t => "'taru' adjective",
            PartOfSpeech::adj_ix => "adjective (keiyoushi) - yoi/ii class",
            PartOfSpeech::n_adv => "adverbial noun (fukushitekimeishi)",
            PartOfSpeech::adv_to => "adverb taking the 'to' particle",
            PartOfSpeech::adj_no => "nouns which may take the genitive case particle 'no'",
            PartOfSpeech::n_pref => "noun, used as a prefix",
            PartOfSpeech::n_suf => "noun, used as a suffix",
            PartOfSpeech::n_t => "noun (temporal) (jisoumeishi)",
            PartOfSpeech::adj_f => "noun or verb acting prenominally",
            PartOfSpeech::v5b => "Godan verb with 'bu' ending",
            PartOfSpeech::v5g => "Godan verb with 'gu' ending",
            PartOfSpeech::v5k => "Godan verb with 'ku' ending",
            PartOfSpeech::v5m => "Godan verb with 'mu' ending",
            PartOfSpeech::v5n => "Godan verb with 'nu' ending",
            PartOfSpeech::v5r => "Godan verb with 'ru' ending",
            PartOfSpeech::v5r_i => "Godan verb with 'ru' ending (irregular verb)",
            PartOfSpeech::v5aru => "Godan verb - -aru special class",
            PartOfSpeech::v5k_s => "Godan verb - Iku/Yuku special class",
            PartOfSpeech::v5s => "Godan verb with 'su' ending",
            PartOfSpeech::v5t => "Godan verb with 'tsu' ending",
            PartOfSpeech::v5u => "Godan verb with 'u' ending",
            PartOfSpeech::v5uru => "Godan verb - Uru old class verb (old form of Eru)",
            PartOfSpeech::v5u_s => "Godan verb with 'u' ending (special class)",
            PartOfSpeech::v1 => "Ichidan verb",
            PartOfSpeech::v1_s => "Ichidan verb - kureru special class",
            PartOfSpeech::vz => "Ichidan verb - zuru verb (alternative form of -jiru verbs)",
            PartOfSpeech::vk => "Kuru verb - special class",
            PartOfSpeech::v2b_s => "Nidan verb (lower class) with 'bu' ending (archaic)",
            PartOfSpeech::v2b_k => "Nidan verb (upper class) with 'bu' ending (archaic)",
            PartOfSpeech::v2d_s => "Nidan verb (lower class) with 'dzu' ending (archaic)",
            PartOfSpeech::v2d_k => "Nidan verb (upper class) with 'dzu' ending (archaic)",
            PartOfSpeech::v2g_s => "Nidan verb (lower class) with 'gu' ending (archaic)",
            PartOfSpeech::v2g_k => "Nidan verb (upper class) with 'gu' ending (archaic)",
            PartOfSpeech::v2h_s => "Nidan verb (lower class) with 'hu/fu' ending (archaic)",
            PartOfSpeech::v2h_k => "Nidan verb (upper class) with 'hu/fu' ending (archaic)",
            PartOfSpeech::v2k_s => "Nidan verb (lower class) with 'ku' ending (archaic)",
            PartOfSpeech::v2k_k => "Nidan verb (upper class) with 'ku' ending (archaic)",
            PartOfSpeech::v2m_s => "Nidan verb (lower class) with 'mu' ending (archaic)",
            PartOfSpeech::v2m_k => "Nidan verb (upper class) with 'mu' ending (archaic)",
            PartOfSpeech::v2n_s => "Nidan verb (lower class) with 'nu' ending (archaic)",
            PartOfSpeech::v2r_s => "Nidan verb (lower class) with 'ru' ending (archaic)",
            PartOfSpeech::v2r_k => "Nidan verb (upper class) with 'ru' ending (archaic)",
            PartOfSpeech::v2s_s => "Nidan verb (lower class) with 'su' ending (archaic)",
            PartOfSpeech::v2t_s => "Nidan verb (lower class) with 'tsu' ending (archaic)",
            PartOfSpeech::v2t_k => "Nidan verb (upper class) with 'tsu' ending (archaic)",
            PartOfSpeech::v2a_s => "Nidan verb with 'u' ending (archaic)",
            PartOfSpeech::v2w_s => {
                "Nidan verb (lower class) with 'u' ending and 'we' conjugation (archaic)"
            }
            PartOfSpeech::v2y_s => "Nidan verb (lower class) with 'yu' ending (archaic)",
            PartOfSpeech::v2y_k => "Nidan verb (upper class) with 'yu' ending (archaic)",
            PartOfSpeech::v2z_s => "Nidan verb (lower class) with 'zu' ending (archaic)",
            PartOfSpeech::vn => "irregular nu verb",
            PartOfSpeech::vr => "irregular ru verb, plain form ends with -ri",
            PartOfSpeech::vs_c => "su verb - precursor to the modern suru",
            PartOfSpeech::vs => "noun or participle which takes the aux. verb suru",
            PartOfSpeech::vs_i => "suru verb - included",
            PartOfSpeech::vs_s => "suru verb - special class",
            PartOfSpeech::v_unspec => "verb unspecified",
            PartOfSpeech::v4b => "Yodan verb with 'bu' ending (archaic)",
            PartOfSpeech::v4g => "Yodan verb with 'gu' ending (archaic)",
            PartOfSpeech::v4h => "Yodan verb with 'hu/fu' ending (archaic)",
            PartOfSpeech::v4k => "Yodan verb with 'ku' ending (archaic)",
            PartOfSpeech::v4m => "Yodan verb with 'mu' ending (archaic)",
            PartOfSpeech::v4n => "Yodan verb with 'nu' ending (archaic)",
            PartOfSpeech::v4r => "Yodan verb with 'ru' ending (archaic)",
            PartOfSpeech::v4s => "Yodan verb with 'su' ending (archaic)",
            PartOfSpeech::v4t => "Yodan verb with 'tsu' ending (archaic)",

            /* -------------------------------------------------------------------------- */
            /*                                Universal POS                               */
            /* -------------------------------------------------------------------------- */
            PartOfSpeech::abv => "abbreviation",
            PartOfSpeech::adf => "adfix",
            PartOfSpeech::art => "article",
            PartOfSpeech::adj => "adjective",
            PartOfSpeech::phr_adj => "adjective phrase",
            PartOfSpeech::adv => "adverb",
            PartOfSpeech::phr_adv => "adverbial phrase",
            PartOfSpeech::aff => "affix",
            PartOfSpeech::aux => "auxiliary",
            PartOfSpeech::aux_adj => "auxiliary adjective",
            PartOfSpeech::aux_v => "auxiliary verb",
            PartOfSpeech::chr => "character",
            PartOfSpeech::cf => "circumfix",
            PartOfSpeech::cls => "classifier",
            PartOfSpeech::conj => "conjunction",
            PartOfSpeech::conj_c => "coordinating conjunction",
            PartOfSpeech::contr => "contraction",
            PartOfSpeech::cop => "copula",
            PartOfSpeech::ctr => "counter",
            PartOfSpeech::det => "determiner",
            PartOfSpeech::expr => "expression",
            PartOfSpeech::inf => "infix",
            PartOfSpeech::intf => "interfix",
            PartOfSpeech::intj => "interjection",
            PartOfSpeech::vi => "intransitive verb",
            PartOfSpeech::name => "name",
            PartOfSpeech::n => "noun",
            PartOfSpeech::num => "numeric",
            PartOfSpeech::part => "particle",
            PartOfSpeech::phr => "phrase",
            PartOfSpeech::postp => "postposition",
            PartOfSpeech::pref => "prefix",
            PartOfSpeech::prep => "preposition",
            PartOfSpeech::phr_prep => "prepositional phrase",
            PartOfSpeech::pron => "pronoun",
            PartOfSpeech::propn => "proper noun",
            PartOfSpeech::prov => "proverb",
            PartOfSpeech::punc => "punctuation",
            PartOfSpeech::conj_s => "subordinating conjunction",
            PartOfSpeech::suff => "suffix",
            PartOfSpeech::sym => "symbol",
            PartOfSpeech::vt => "transitive verb",
            PartOfSpeech::un => "unknown",
            PartOfSpeech::v => "verb",
        }
    }
}
