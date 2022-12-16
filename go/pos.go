package odict

type PartOfSpeech struct {
	Tag   string
	Label string
	Buf   POS
}

/* -------------------------------------------------------------------------- */
/*                            Japanese-specific POS                           */
/* -------------------------------------------------------------------------- */

var AdjectivalPreNoun = PartOfSpeech{Tag: "adj-pn", Buf: POSadj_pn, Label: "pre-noun adjectival (rentaishi)"}
var AdjectiveKari = PartOfSpeech{Tag: "adj-kari", Buf: POSadj_kari, Label: "'kari' adjective (archaic)"}
var AdjectiveKu = PartOfSpeech{Tag: "adj-ku", Buf: POSadj_ku, Label: "'ku' adjective (archaic)"}
var AdjectiveNari = PartOfSpeech{Tag: "adj-nari", Buf: POSadj_nari, Label: "archaic/formal form of na-adjective"}
var AdjectiveNoun = PartOfSpeech{Tag: "adj-na", Buf: POSadj_na, Label: "adjectival nouns or quasi-adjectives (keiyodoshi)"}
var AdjectiveShiku = PartOfSpeech{Tag: "adj-shiku", Buf: POSadj_shiku, Label: "'shiku' adjective (archaic)"}
var AdjectiveTaru = PartOfSpeech{Tag: "adj-t", Buf: POSadj_t, Label: "'taru' adjective"}
var AdjectiveYoiIi = PartOfSpeech{Tag: "adj-ix", Buf: POSadj_ix, Label: "adjective (keiyoushi) - yoi/ii class"}
var AdverbialNoun = PartOfSpeech{Tag: "n-adv", Buf: POSn_adv, Label: "adverbial noun (fukushitekimeishi)"}
var AdverbTo = PartOfSpeech{Tag: "adv-to", Buf: POSadv_to, Label: "adverb taking the 'to' particle"}
var NounNo = PartOfSpeech{Tag: "adj-no", Buf: POSadj_no, Label: "nouns which may take the genitive case particle 'no'"}
var NounPrefix = PartOfSpeech{Tag: "n-pref", Buf: POSn_pref, Label: "noun, used as a prefix"}
var NounSuffix = PartOfSpeech{Tag: "n-suf", Buf: POSn_suf, Label: "noun, used as a suffix"}
var NounTemporal = PartOfSpeech{Tag: "n-t", Buf: POSn_t, Label: "noun (temporal) (jisoumeishi)"}
var NounVerbPrenominal = PartOfSpeech{Tag: "adj-f", Buf: POSadj_f, Label: "noun or verb acting prenominally"}
var VerbGodanBu = PartOfSpeech{Tag: "v5b", Buf: POSv5b, Label: "Godan verb with 'bu' ending"}
var VerbGodanGu = PartOfSpeech{Tag: "v5g", Buf: POSv5g, Label: "Godan verb with 'gu' ending"}
var VerbGodanKu = PartOfSpeech{Tag: "v5k", Buf: POSv5k, Label: "Godan verb with 'ku' ending"}
var VerbGodanMu = PartOfSpeech{Tag: "v5m", Buf: POSv5m, Label: "Godan verb with 'mu' ending"}
var VerbGodanNu = PartOfSpeech{Tag: "v5n", Buf: POSv5n, Label: "Godan verb with 'nu' ending"}
var VerbGodanRu = PartOfSpeech{Tag: "v5r", Buf: POSv5r, Label: "Godan verb with 'ru' ending"}
var VerbGodanRuIrregular = PartOfSpeech{Tag: "v5r-i", Buf: POSv5r_i, Label: "Godan verb with 'ru' ending (irregular verb)"}
var VerbGodanSpecialAru = PartOfSpeech{Tag: "v5aru", Buf: POSv5aru, Label: "Godan verb - -aru special class"}
var VerbGodanSpecialIkuYuku = PartOfSpeech{Tag: "v5k-s", Buf: POSv5k_s, Label: "Godan verb - Iku/Yuku special class"}
var VerbGodanSu = PartOfSpeech{Tag: "v5s", Buf: POSv5s, Label: "Godan verb with 'su' ending"}
var VerbGodanTsu = PartOfSpeech{Tag: "v5t", Buf: POSv5t, Label: "Godan verb with 'tsu' ending"}
var VerbGodanU = PartOfSpeech{Tag: "v5u", Buf: POSv5u, Label: "Godan verb with 'u' ending"}
var VerbGodanUruOldVerbOldFormOfEru = PartOfSpeech{Tag: "v5uru", Buf: POSv5uru, Label: "Godan verb - Uru old class verb (old form of Eru)"}
var VerbGodanUSpecial = PartOfSpeech{Tag: "v5u-s", Buf: POSv5u_s, Label: "Godan verb with 'u' ending (special class)"}
var VerbIchidan = PartOfSpeech{Tag: "v1", Buf: POSv1, Label: "Ichidan verb"}
var VerbIchidanSpecial = PartOfSpeech{Tag: "v1-s", Buf: POSv1_s, Label: "Ichidan verb - kureru special class"}
var VerbIchidanZuru = PartOfSpeech{Tag: "vz", Buf: POSvz, Label: "Ichidan verb - zuru verb (alternative form of -jiru verbs)"}
var VerbKuruSpecial = PartOfSpeech{Tag: "vk", Buf: POSvk, Label: "Kuru verb - special class"}
var VerbNidanBuLower = PartOfSpeech{Tag: "v2b-s", Buf: POSv2b_s, Label: "Nidan verb (lower class) with 'bu' ending (archaic)"}
var VerbNidanBuUpper = PartOfSpeech{Tag: "v2b-k", Buf: POSv2b_k, Label: "Nidan verb (upper class) with 'bu' ending (archaic)"}
var VerbNidanDzuLower = PartOfSpeech{Tag: "v2d-s", Buf: POSv2d_s, Label: "Nidan verb (lower class) with 'dzu' ending (archaic)"}
var VerbNidanDzuUpper = PartOfSpeech{Tag: "v2d-k", Buf: POSv2d_k, Label: "Nidan verb (upper class) with 'dzu' ending (archaic)"}
var VerbNidanGuLower = PartOfSpeech{Tag: "v2g-s", Buf: POSv2g_s, Label: "Nidan verb (lower class) with 'gu' ending (archaic)"}
var VerbNidanGuUpper = PartOfSpeech{Tag: "v2g-k", Buf: POSv2g_k, Label: "Nidan verb (upper class) with 'gu' ending (archaic)"}
var VerbNidanHuFuLower = PartOfSpeech{Tag: "v2h-s", Buf: POSv2h_s, Label: "Nidan verb (lower class) with 'hu/fu' ending (archaic)"}
var VerbNidanHuFuUpper = PartOfSpeech{Tag: "v2h-k", Buf: POSv2h_k, Label: "Nidan verb (upper class) with 'hu/fu' ending (archaic)"}
var VerbNidanKuLower = PartOfSpeech{Tag: "v2k-s", Buf: POSv2k_s, Label: "Nidan verb (lower class) with 'ku' ending (archaic)"}
var VerbNidanKuUpper = PartOfSpeech{Tag: "v2k-k", Buf: POSv2k_k, Label: "Nidan verb (upper class) with 'ku' ending (archaic)"}
var VerbNidanMuLower = PartOfSpeech{Tag: "v2m-s", Buf: POSv2m_s, Label: "Nidan verb (lower class) with 'mu' ending (archaic)"}
var VerbNidanMuUpper = PartOfSpeech{Tag: "v2m-k", Buf: POSv2m_k, Label: "Nidan verb (upper class) with 'mu' ending (archaic)"}
var VerbNidanNuLower = PartOfSpeech{Tag: "v2n-s", Buf: POSv2n_s, Label: "Nidan verb (lower class) with 'nu' ending (archaic)"}
var VerbNidanRuLower = PartOfSpeech{Tag: "v2r-s", Buf: POSv2r_s, Label: "Nidan verb (lower class) with 'ru' ending (archaic)"}
var VerbNidanRuUpper = PartOfSpeech{Tag: "v2r-k", Buf: POSv2r_k, Label: "Nidan verb (upper class) with 'ru' ending (archaic)"}
var VerbNidanSuLower = PartOfSpeech{Tag: "v2s-s", Buf: POSv2s_s, Label: "Nidan verb (lower class) with 'su' ending (archaic)"}
var VerbNidanTsuLower = PartOfSpeech{Tag: "v2t-s", Buf: POSv2t_s, Label: "Nidan verb (lower class) with 'tsu' ending (archaic)"}
var VerbNidanTsuUpper = PartOfSpeech{Tag: "v2t-k", Buf: POSv2t_k, Label: "Nidan verb (upper class) with 'tsu' ending (archaic)"}
var VerbNidanU = PartOfSpeech{Tag: "v2a-s", Buf: POSv2a_s, Label: "Nidan verb with 'u' ending (archaic)"}
var VerbNidanUWeLower = PartOfSpeech{Tag: "v2w-s", Buf: POSv2w_s, Label: "Nidan verb (lower class) with 'u' ending and 'we' conjugation (archaic)"}
var VerbNidanYuLower = PartOfSpeech{Tag: "v2y-s", Buf: POSv2y_s, Label: "Nidan verb (lower class) with 'yu' ending (archaic)"}
var VerbNidanYuUpper = PartOfSpeech{Tag: "v2y-k", Buf: POSv2y_k, Label: "Nidan verb (upper class) with 'yu' ending (archaic)"}
var VerbNidanZuLower = PartOfSpeech{Tag: "v2z-s", Buf: POSv2z_s, Label: "Nidan verb (lower class) with 'zu' ending (archaic)"}
var VerbNuIrregular = PartOfSpeech{Tag: "vn", Buf: POSvn, Label: "irregular nu verb"}
var VerbRuIrregular = PartOfSpeech{Tag: "vr", Buf: POSvr, Label: "irregular ru verb, plain form ends with -ri"}
var VerbSu = PartOfSpeech{Tag: "vs-c", Buf: POSvs_c, Label: "su verb - precursor to the modern suru"}
var VerbSuru = PartOfSpeech{Tag: "vs", Buf: POSvs, Label: "noun or participle which takes the aux. verb suru"}
var VerbSuruIncluded = PartOfSpeech{Tag: "vs-i", Buf: POSvs_i, Label: "suru verb - included"}
var VerbSuruSpecial = PartOfSpeech{Tag: "vs-s", Buf: POSvs_s, Label: "suru verb - special class"}
var VerbUnspecified = PartOfSpeech{Tag: "v-unspec", Buf: POSv_unspec, Label: "verb unspecified"}
var VerbYodanBu = PartOfSpeech{Tag: "v4b", Buf: POSv4b, Label: "Yodan verb with 'bu' ending (archaic)"}
var VerbYodanGu = PartOfSpeech{Tag: "v4g", Buf: POSv4g, Label: "Yodan verb with 'gu' ending (archaic)"}
var VerbYodanHuFu = PartOfSpeech{Tag: "v4h", Buf: POSv4h, Label: "Yodan verb with 'hu/fu' ending (archaic)"}
var VerbYodanKu = PartOfSpeech{Tag: "v4k", Buf: POSv4k, Label: "Yodan verb with 'ku' ending (archaic)"}
var VerbYodanMu = PartOfSpeech{Tag: "v4m", Buf: POSv4m, Label: "Yodan verb with 'mu' ending (archaic)"}
var VerbYodanNu = PartOfSpeech{Tag: "v4n", Buf: POSv4n, Label: "Yodan verb with 'nu' ending (archaic)"}
var VerbYodanRu = PartOfSpeech{Tag: "v4r", Buf: POSv4r, Label: "Yodan verb with 'ru' ending (archaic)"}
var VerbYodanSu = PartOfSpeech{Tag: "v4s", Buf: POSv4s, Label: "Yodan verb with 'su' ending (archaic)"}
var VerbYodanTsu = PartOfSpeech{Tag: "v4t", Buf: POSv4t, Label: "Yodan verb with 'tsu' ending (archaic)"}

/* -------------------------------------------------------------------------- */
/*                                Universal POS                               */
/* -------------------------------------------------------------------------- */

var Adverb = PartOfSpeech{Tag: "adv", Label: "adverb", Buf: POSadv}
var Auxiliary = PartOfSpeech{Tag: "aux", Label: "auxiliary", Buf: POSaux}
var AuxiliaryAdjective = PartOfSpeech{Tag: "aux-adj", Buf: POSaux_adj, Label: "auxiliary adjective"}
var AuxiliaryVerb = PartOfSpeech{Tag: "aux-v", Buf: POSaux_v, Label: "auxiliary verb"}
var Conjunction = PartOfSpeech{Tag: "conj", Buf: POSconj, Label: "conjunction"}
var CoordinatingConjunction = PartOfSpeech{Tag: "cconj", Buf: POScconj, Label: "coordinating conjunction"}
var Copula = PartOfSpeech{Tag: "cop", Buf: POScop, Label: "copula"}
var Counter = PartOfSpeech{Tag: "ctr", Buf: POSctr, Label: "counter"}
var Expression = PartOfSpeech{Tag: "expr", Buf: POSexpr, Label: "expression"}
var Interjection = PartOfSpeech{Tag: "intj", Buf: POSintj, Label: "interjection"}
var IntransitiveVerb = PartOfSpeech{Tag: "vi", Buf: POSvi, Label: "intransitive verb"}
var Noun = PartOfSpeech{Tag: "n", Buf: POSn, Label: "noun"}
var Numeric = PartOfSpeech{Tag: "num", Buf: POSnum, Label: "numeric"}
var Particle = PartOfSpeech{Tag: "part", Buf: POSpart, Label: "particle"}
var Prefix = PartOfSpeech{Tag: "pref", Buf: POSpart, Label: "prefix"}
var Preposition = PartOfSpeech{Tag: "prep", Buf: POSprep, Label: "preposition"}
var Pronoun = PartOfSpeech{Tag: "pron", Buf: POSpron, Label: "pronoun"}
var ProperNoun = PartOfSpeech{Tag: "propn", Buf: POSpropn, Label: "proper noun"}
var SubordinatingConjunction = PartOfSpeech{Tag: "sconj", Buf: POSsconj, Label: "subordinating conjunction"}
var Suffix = PartOfSpeech{Tag: "suff", Buf: POSsuff, Label: "suffix"}
var Symbol = PartOfSpeech{Tag: "sym", Buf: POSsym, Label: "symbol"}
var TransitiveVerb = PartOfSpeech{Tag: "vt", Buf: POSvt, Label: "transitive verb"}
var Unknown = PartOfSpeech{Tag: "un", Buf: POSun, Label: "unknown"}
var Verb = PartOfSpeech{Tag: "v", Buf: POSv, Label: "verb"}

var partsOfSpeech = []PartOfSpeech{

	/* -------------------------------- Universal ------------------------------- */

	Adverb,
	Auxiliary,
	AuxiliaryAdjective,
	AuxiliaryVerb,
	Conjunction,
	CoordinatingConjunction,
	Copula,
	Counter,
	Expression,
	Interjection,
	IntransitiveVerb,
	Noun,
	Numeric,
	Particle,
	Prefix,
	Preposition,
	Pronoun,
	ProperNoun,
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

var posPartOfSpeechMap = (func() map[POS]PartOfSpeech {
	m := map[POS]PartOfSpeech{}

	for _, pos := range partsOfSpeech {
		m[pos.Buf] = pos
	}

	return m
})()

var posTagPartOfSpeechMap = (func() map[string]PartOfSpeech {
	m := map[string]PartOfSpeech{}

	for _, pos := range partsOfSpeech {
		m[pos.Tag] = pos
	}

	return m
})()
