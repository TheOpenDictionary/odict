package types

import (
	"encoding/xml"
	"strings"
)

type PartOfSpeech struct {
	Label string
	Buf   POS
}

func (pos PartOfSpeech) Tag() string {
	return strings.ReplaceAll(pos.Buf.String(), "_", "-")
}

/* -------------------------------------------------------------------------- */
/*                            Japanese-specific POS                           */
/* -------------------------------------------------------------------------- */

var AdjectivalPreNoun = PartOfSpeech{Buf: POSadj_pn, Label: "pre-noun adjectival (rentaishi)"}
var AdjectiveKari = PartOfSpeech{Buf: POSadj_kari, Label: "'kari' adjective (archaic)"}
var AdjectiveKu = PartOfSpeech{Buf: POSadj_ku, Label: "'ku' adjective (archaic)"}
var AdjectiveNari = PartOfSpeech{Buf: POSadj_nari, Label: "archaic/formal form of na-adjective"}
var AdjectiveNoun = PartOfSpeech{Buf: POSadj_na, Label: "adjectival nouns or quasi-adjectives (keiyodoshi)"}
var AdjectiveShiku = PartOfSpeech{Buf: POSadj_shiku, Label: "'shiku' adjective (archaic)"}
var AdjectiveTaru = PartOfSpeech{Buf: POSadj_t, Label: "'taru' adjective"}
var AdjectiveYoiIi = PartOfSpeech{Buf: POSadj_ix, Label: "adjective (keiyoushi) - yoi/ii class"}
var AdverbialNoun = PartOfSpeech{Buf: POSn_adv, Label: "adverbial noun (fukushitekimeishi)"}
var AdverbTo = PartOfSpeech{Buf: POSadv_to, Label: "adverb taking the 'to' particle"}
var NounNo = PartOfSpeech{Buf: POSadj_no, Label: "nouns which may take the genitive case particle 'no'"}
var NounPrefix = PartOfSpeech{Buf: POSn_pref, Label: "noun, used as a prefix"}
var NounSuffix = PartOfSpeech{Buf: POSn_suf, Label: "noun, used as a suffix"}
var NounTemporal = PartOfSpeech{Buf: POSn_t, Label: "noun (temporal) (jisoumeishi)"}
var NounVerbPrenominal = PartOfSpeech{Buf: POSadj_f, Label: "noun or verb acting prenominally"}
var VerbGodanBu = PartOfSpeech{Buf: POSv5b, Label: "Godan verb with 'bu' ending"}
var VerbGodanGu = PartOfSpeech{Buf: POSv5g, Label: "Godan verb with 'gu' ending"}
var VerbGodanKu = PartOfSpeech{Buf: POSv5k, Label: "Godan verb with 'ku' ending"}
var VerbGodanMu = PartOfSpeech{Buf: POSv5m, Label: "Godan verb with 'mu' ending"}
var VerbGodanNu = PartOfSpeech{Buf: POSv5n, Label: "Godan verb with 'nu' ending"}
var VerbGodanRu = PartOfSpeech{Buf: POSv5r, Label: "Godan verb with 'ru' ending"}
var VerbGodanRuIrregular = PartOfSpeech{Buf: POSv5r_i, Label: "Godan verb with 'ru' ending (irregular verb)"}
var VerbGodanSpecialAru = PartOfSpeech{Buf: POSv5aru, Label: "Godan verb - -aru special class"}
var VerbGodanSpecialIkuYuku = PartOfSpeech{Buf: POSv5k_s, Label: "Godan verb - Iku/Yuku special class"}
var VerbGodanSu = PartOfSpeech{Buf: POSv5s, Label: "Godan verb with 'su' ending"}
var VerbGodanTsu = PartOfSpeech{Buf: POSv5t, Label: "Godan verb with 'tsu' ending"}
var VerbGodanU = PartOfSpeech{Buf: POSv5u, Label: "Godan verb with 'u' ending"}
var VerbGodanUruOldVerbOldFormOfEru = PartOfSpeech{Buf: POSv5uru, Label: "Godan verb - Uru old class verb (old form of Eru)"}
var VerbGodanUSpecial = PartOfSpeech{Buf: POSv5u_s, Label: "Godan verb with 'u' ending (special class)"}
var VerbIchidan = PartOfSpeech{Buf: POSv1, Label: "Ichidan verb"}
var VerbIchidanSpecial = PartOfSpeech{Buf: POSv1_s, Label: "Ichidan verb - kureru special class"}
var VerbIchidanZuru = PartOfSpeech{Buf: POSvz, Label: "Ichidan verb - zuru verb (alternative form of -jiru verbs)"}
var VerbKuruSpecial = PartOfSpeech{Buf: POSvk, Label: "Kuru verb - special class"}
var VerbNidanBuLower = PartOfSpeech{Buf: POSv2b_s, Label: "Nidan verb (lower class) with 'bu' ending (archaic)"}
var VerbNidanBuUpper = PartOfSpeech{Buf: POSv2b_k, Label: "Nidan verb (upper class) with 'bu' ending (archaic)"}
var VerbNidanDzuLower = PartOfSpeech{Buf: POSv2d_s, Label: "Nidan verb (lower class) with 'dzu' ending (archaic)"}
var VerbNidanDzuUpper = PartOfSpeech{Buf: POSv2d_k, Label: "Nidan verb (upper class) with 'dzu' ending (archaic)"}
var VerbNidanGuLower = PartOfSpeech{Buf: POSv2g_s, Label: "Nidan verb (lower class) with 'gu' ending (archaic)"}
var VerbNidanGuUpper = PartOfSpeech{Buf: POSv2g_k, Label: "Nidan verb (upper class) with 'gu' ending (archaic)"}
var VerbNidanHuFuLower = PartOfSpeech{Buf: POSv2h_s, Label: "Nidan verb (lower class) with 'hu/fu' ending (archaic)"}
var VerbNidanHuFuUpper = PartOfSpeech{Buf: POSv2h_k, Label: "Nidan verb (upper class) with 'hu/fu' ending (archaic)"}
var VerbNidanKuLower = PartOfSpeech{Buf: POSv2k_s, Label: "Nidan verb (lower class) with 'ku' ending (archaic)"}
var VerbNidanKuUpper = PartOfSpeech{Buf: POSv2k_k, Label: "Nidan verb (upper class) with 'ku' ending (archaic)"}
var VerbNidanMuLower = PartOfSpeech{Buf: POSv2m_s, Label: "Nidan verb (lower class) with 'mu' ending (archaic)"}
var VerbNidanMuUpper = PartOfSpeech{Buf: POSv2m_k, Label: "Nidan verb (upper class) with 'mu' ending (archaic)"}
var VerbNidanNuLower = PartOfSpeech{Buf: POSv2n_s, Label: "Nidan verb (lower class) with 'nu' ending (archaic)"}
var VerbNidanRuLower = PartOfSpeech{Buf: POSv2r_s, Label: "Nidan verb (lower class) with 'ru' ending (archaic)"}
var VerbNidanRuUpper = PartOfSpeech{Buf: POSv2r_k, Label: "Nidan verb (upper class) with 'ru' ending (archaic)"}
var VerbNidanSuLower = PartOfSpeech{Buf: POSv2s_s, Label: "Nidan verb (lower class) with 'su' ending (archaic)"}
var VerbNidanTsuLower = PartOfSpeech{Buf: POSv2t_s, Label: "Nidan verb (lower class) with 'tsu' ending (archaic)"}
var VerbNidanTsuUpper = PartOfSpeech{Buf: POSv2t_k, Label: "Nidan verb (upper class) with 'tsu' ending (archaic)"}
var VerbNidanU = PartOfSpeech{Buf: POSv2a_s, Label: "Nidan verb with 'u' ending (archaic)"}
var VerbNidanUWeLower = PartOfSpeech{Buf: POSv2w_s, Label: "Nidan verb (lower class) with 'u' ending and 'we' conjugation (archaic)"}
var VerbNidanYuLower = PartOfSpeech{Buf: POSv2y_s, Label: "Nidan verb (lower class) with 'yu' ending (archaic)"}
var VerbNidanYuUpper = PartOfSpeech{Buf: POSv2y_k, Label: "Nidan verb (upper class) with 'yu' ending (archaic)"}
var VerbNidanZuLower = PartOfSpeech{Buf: POSv2z_s, Label: "Nidan verb (lower class) with 'zu' ending (archaic)"}
var VerbNuIrregular = PartOfSpeech{Buf: POSvn, Label: "irregular nu verb"}
var VerbRuIrregular = PartOfSpeech{Buf: POSvr, Label: "irregular ru verb, plain form ends with -ri"}
var VerbSu = PartOfSpeech{Buf: POSvs_c, Label: "su verb - precursor to the modern suru"}
var VerbSuru = PartOfSpeech{Buf: POSvs, Label: "noun or participle which takes the aux. verb suru"}
var VerbSuruIncluded = PartOfSpeech{Buf: POSvs_i, Label: "suru verb - included"}
var VerbSuruSpecial = PartOfSpeech{Buf: POSvs_s, Label: "suru verb - special class"}
var VerbUnspecified = PartOfSpeech{Buf: POSv_unspec, Label: "verb unspecified"}
var VerbYodanBu = PartOfSpeech{Buf: POSv4b, Label: "Yodan verb with 'bu' ending (archaic)"}
var VerbYodanGu = PartOfSpeech{Buf: POSv4g, Label: "Yodan verb with 'gu' ending (archaic)"}
var VerbYodanHuFu = PartOfSpeech{Buf: POSv4h, Label: "Yodan verb with 'hu/fu' ending (archaic)"}
var VerbYodanKu = PartOfSpeech{Buf: POSv4k, Label: "Yodan verb with 'ku' ending (archaic)"}
var VerbYodanMu = PartOfSpeech{Buf: POSv4m, Label: "Yodan verb with 'mu' ending (archaic)"}
var VerbYodanNu = PartOfSpeech{Buf: POSv4n, Label: "Yodan verb with 'nu' ending (archaic)"}
var VerbYodanRu = PartOfSpeech{Buf: POSv4r, Label: "Yodan verb with 'ru' ending (archaic)"}
var VerbYodanSu = PartOfSpeech{Buf: POSv4s, Label: "Yodan verb with 'su' ending (archaic)"}
var VerbYodanTsu = PartOfSpeech{Buf: POSv4t, Label: "Yodan verb with 'tsu' ending (archaic)"}

/* -------------------------------------------------------------------------- */
/*                                Universal POS                               */
/* -------------------------------------------------------------------------- */

var Abbreviation = PartOfSpeech{Buf: POSabv, Label: "abbreviation"}
var Adfix = PartOfSpeech{Buf: POSadf, Label: "adfix"}
var Adjective = PartOfSpeech{Buf: POSadj, Label: "adjective"}
var AdjectivePhrase = PartOfSpeech{Buf: POSphr_adj, Label: "adjective phrase"}
var Adverb = PartOfSpeech{Buf: POSadv, Label: "adverb"}
var AdverbialPhrase = PartOfSpeech{Buf: POSphr_adv, Label: "adverbial phrase"}
var Affix = PartOfSpeech{Buf: POSaff, Label: "affix"}
var Auxiliary = PartOfSpeech{Buf: POSaux, Label: "auxiliary"}
var AuxiliaryAdjective = PartOfSpeech{Buf: POSaux_adj, Label: "auxiliary adjective"}
var AuxiliaryVerb = PartOfSpeech{Buf: POSaux_v, Label: "auxiliary verb"}
var Character = PartOfSpeech{Buf: POSchr, Label: "character"}
var Circumfix = PartOfSpeech{Buf: POScf, Label: "circumfix"}
var Conjunction = PartOfSpeech{Buf: POSconj, Label: "conjunction"}
var CoordinatingConjunction = PartOfSpeech{Buf: POSconj_c, Label: "coordinating conjunction"}
var Copula = PartOfSpeech{Buf: POScop, Label: "copula"}
var Counter = PartOfSpeech{Buf: POSctr, Label: "counter"}
var Determiner = PartOfSpeech{Buf: POSdet, Label: "determiner"}
var Expression = PartOfSpeech{Buf: POSexpr, Label: "expression"}
var Infix = PartOfSpeech{Buf: POSinf, Label: "infix"}
var Interfix = PartOfSpeech{Buf: POSintf, Label: "interfix"}
var Interjection = PartOfSpeech{Buf: POSintj, Label: "interjection"}
var IntransitiveVerb = PartOfSpeech{Buf: POSvi, Label: "intransitive verb"}
var Name = PartOfSpeech{Buf: POSname, Label: "name"}
var Noun = PartOfSpeech{Buf: POSn, Label: "noun"}
var Numeric = PartOfSpeech{Buf: POSnum, Label: "numeric"}
var Particle = PartOfSpeech{Buf: POSpart, Label: "particle"}
var Phrase = PartOfSpeech{Buf: POSphr, Label: "phrase"}
var Postposition = PartOfSpeech{Buf: POSpostp, Label: "postposition"}
var Prefix = PartOfSpeech{Buf: POSpref, Label: "prefix"}
var Preposition = PartOfSpeech{Buf: POSprep, Label: "preposition"}
var PrepositionalPhrase = PartOfSpeech{Buf: POSphr_prep, Label: "prepositional phrase"}
var Pronoun = PartOfSpeech{Buf: POSpron, Label: "pronoun"}
var ProperNoun = PartOfSpeech{Buf: POSpropn, Label: "proper noun"}
var Proverb = PartOfSpeech{Buf: POSprov, Label: "proverb"}
var Punctuation = PartOfSpeech{Buf: POSpunc, Label: "punctuation"}
var SubordinatingConjunction = PartOfSpeech{Buf: POSconj_s, Label: "subordinating conjunction"}
var Suffix = PartOfSpeech{Buf: POSsuff, Label: "suffix"}
var Symbol = PartOfSpeech{Buf: POSsym, Label: "symbol"}
var TransitiveVerb = PartOfSpeech{Buf: POSvt, Label: "transitive verb"}
var Unknown = PartOfSpeech{Buf: POSun, Label: "unknown"}
var Verb = PartOfSpeech{Buf: POSv, Label: "verb"}

var partsOfSpeech = []PartOfSpeech{

	/* -------------------------------- Universal ------------------------------- */

	Abbreviation,
	Adfix,
	Adjective,
	AdjectivePhrase,
	Adverb,
	AdverbialPhrase,
	Affix,
	Auxiliary,
	AuxiliaryAdjective,
	AuxiliaryVerb,
	Character,
	Circumfix,
	Conjunction,
	CoordinatingConjunction,
	Copula,
	Counter,
	Determiner,
	Expression,
	Infix,
	Interfix,
	Interjection,
	IntransitiveVerb,
	Name,
	Noun,
	Numeric,
	Particle,
	Phrase,
	Postposition,
	Prefix,
	Preposition,
	PrepositionalPhrase,
	Pronoun,
	ProperNoun,
	Proverb,
	Punctuation,
	SubordinatingConjunction,
	Suffix,
	Symbol,
	TransitiveVerb,
	Unknown,
	Verb,

	/* -------------------------------- Japanese -------------------------------- */

	AdjectivalPreNoun,
	AdjectiveKari,
	AdjectiveKu,
	AdjectiveNari,
	AdjectiveNoun,
	AdjectiveShiku,
	AdjectiveTaru,
	AdjectiveYoiIi,
	AdverbialNoun,
	AdverbTo,
	NounNo,
	NounPrefix,
	NounSuffix,
	NounTemporal,
	NounVerbPrenominal,
	VerbGodanBu,
	VerbGodanGu,
	VerbGodanKu,
	VerbGodanMu,
	VerbGodanNu,
	VerbGodanRu,
	VerbGodanRuIrregular,
	VerbGodanSpecialAru,
	VerbGodanSpecialIkuYuku,
	VerbGodanSu,
	VerbGodanTsu,
	VerbGodanU,
	VerbGodanUruOldVerbOldFormOfEru,
	VerbGodanUSpecial,
	VerbIchidan,
	VerbIchidanSpecial,
	VerbIchidanZuru,
	VerbKuruSpecial,
	VerbNidanBuLower,
	VerbNidanBuUpper,
	VerbNidanDzuLower,
	VerbNidanDzuUpper,
	VerbNidanGuLower,
	VerbNidanGuUpper,
	VerbNidanHuFuLower,
	VerbNidanHuFuUpper,
	VerbNidanKuLower,
	VerbNidanKuUpper,
	VerbNidanMuLower,
	VerbNidanMuUpper,
	VerbNidanNuLower,
	VerbNidanRuLower,
	VerbNidanRuUpper,
	VerbNidanSuLower,
	VerbNidanTsuLower,
	VerbNidanTsuUpper,
	VerbNidanU,
	VerbNidanUWeLower,
	VerbNidanYuLower,
	VerbNidanYuUpper,
	VerbNidanZuLower,
	VerbNuIrregular,
	VerbRuIrregular,
	VerbSu,
	VerbSuru,
	VerbSuruIncluded,
	VerbSuruSpecial,
	VerbUnspecified,
	VerbYodanBu,
	VerbYodanGu,
	VerbYodanHuFu,
	VerbYodanKu,
	VerbYodanMu,
	VerbYodanNu,
	VerbYodanRu,
	VerbYodanSu,
	VerbYodanTsu,
}

var posTagPartOfSpeechMap = (func() map[string]PartOfSpeech {
	m := map[string]PartOfSpeech{}

	for _, pos := range partsOfSpeech {
		m[pos.Tag()] = pos
	}

	return m
})()

func (p PartOfSpeech) MarshalText() ([]byte, error) {
	return []byte(p.Tag()), nil
}

func (p *PartOfSpeech) UnmarshalText(text []byte) error {
	tag := string(text)
	*p = strToPartOfSpeech(tag)
	return nil
}

func (pos PartOfSpeech) MarshalXMLAttr(name xml.Name) (xml.Attr, error) {
	return xml.Attr{Name: name, Value: pos.Tag()}, nil
}

func (pos *PartOfSpeech) UnmarshalXMLAttr(attr xml.Attr) error {
	*pos = strToPartOfSpeech(attr.Value)
	return nil
}
