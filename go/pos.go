package odict

type PartOfSpeech string

// Japanese-specific part-of-speech
const (
	PrenominalNounOrVerb                          PartOfSpeech = "adj-f"
	Keiyoushi                                     PartOfSpeech = "adj-i"
	KeiyoushiYoiIi                                PartOfSpeech = "adj-ix"
	KariAdjective                                 PartOfSpeech = "adj-kari"
	KuAdjective                                   PartOfSpeech = "adj-ku"
	AdjectivalNounsOrQuasiAdjectivesKeiyodoshi    PartOfSpeech = "adj-na"
	FormalFormOfNaAdjective                       PartOfSpeech = "adj-nari"
	NounsWhichMayTakeTheGenitiveCaseParticleNo    PartOfSpeech = "adj-no"
	PreNounAdjectivalRentaishi                    PartOfSpeech = "adj-pn"
	ShikuAdjective                                PartOfSpeech = "adj-shiku"
	TaruAdjective                                 PartOfSpeech = "adj-t"
	AdverbTakingTheToParticle                     PartOfSpeech = "adv-to"
	NounFutsuumeishi                              PartOfSpeech = "n"
	AdverbialNoun                                 PartOfSpeech = "n-adv"
	NounUsedAsAPrefix                             PartOfSpeech = "n-pref"
	NounUsedAsASuffix                             PartOfSpeech = "n-suf"
	NounTemporal                                  PartOfSpeech = "n-t"
	Unclassified                                  PartOfSpeech = "unc"
	VerbUnspecified                               PartOfSpeech = "v-unspec"
	IchidanVerb                                   PartOfSpeech = "v1"
	IchidanVerbKureruSpecialClass                 PartOfSpeech = "v1-s"
	NidanVerbUEnding                              PartOfSpeech = "v2a-s"
	NidanVerbUpperClassBuEnding                   PartOfSpeech = "v2b-k"
	NidanVerbLowerClassBuEnding                   PartOfSpeech = "v2b-s"
	NidanVerbUpperClassDzuEnding                  PartOfSpeech = "v2d-k"
	NidanVerbLowerClassDzuEnding                  PartOfSpeech = "v2d-s"
	NidanVerbUpperClassGuEnding                   PartOfSpeech = "v2g-k"
	NidanVerbLowerClassGuEnding                   PartOfSpeech = "v2g-s"
	NidanVerbUpperClassHuFuEnding                 PartOfSpeech = "v2h-k"
	NidanVerbLowerClassHuFuEnding                 PartOfSpeech = "v2h-s"
	NidanVerbUpperClassKuEnding                   PartOfSpeech = "v2k-k"
	NidanVerbLowerClassKuEnding                   PartOfSpeech = "v2k-s"
	NidanVerbUpperClassMuEnding                   PartOfSpeech = "v2m-k"
	NidanVerbLowerClassMuEnding                   PartOfSpeech = "v2m-s"
	NidanVerbLowerClassNuEnding                   PartOfSpeech = "v2n-s"
	NidanVerbUpperClassRuEnding                   PartOfSpeech = "v2r-k"
	NidanVerbLowerClassRuEnding                   PartOfSpeech = "v2r-s"
	NidanVerbLowerClassSuEnding                   PartOfSpeech = "v2s-s"
	NidanVerbUpperClassTsuEnding                  PartOfSpeech = "v2t-k"
	NidanVerbLowerClassTsuEnding                  PartOfSpeech = "v2t-s"
	NidanVerbLowerClassUEndingAndWeConjugation    PartOfSpeech = "v2w-s"
	NidanVerbUpperClassYuEnding                   PartOfSpeech = "v2y-k"
	NidanVerbLowerClassYuEnding                   PartOfSpeech = "v2y-s"
	NidanVerbLowerClassZuEnding                   PartOfSpeech = "v2z-s"
	YodanVerbBuEnding                             PartOfSpeech = "v4b"
	YodanVerbGuEnding                             PartOfSpeech = "v4g"
	YodanVerbHuFuEnding                           PartOfSpeech = "v4h"
	YodanVerbKuEnding                             PartOfSpeech = "v4k"
	YodanVerbMuEnding                             PartOfSpeech = "v4m"
	YodanVerbNuEnding                             PartOfSpeech = "v4n"
	YodanVerbRuEnding                             PartOfSpeech = "v4r"
	YodanVerbSuEnding                             PartOfSpeech = "v4s"
	YodanVerbTsuEnding                            PartOfSpeech = "v4t"
	GodanVerbAruSpecialClass                      PartOfSpeech = "v5aru"
	GodanVerbBuEnding                             PartOfSpeech = "v5b"
	GodanVerbGuEnding                             PartOfSpeech = "v5g"
	GodanVerbKuEnding                             PartOfSpeech = "v5k"
	GodanVerbIkuYukuSpecialClass                  PartOfSpeech = "v5k-s"
	GodanVerbMuEnding                             PartOfSpeech = "v5m"
	GodanVerbNuEnding                             PartOfSpeech = "v5n"
	GodanVerbRuEnding                             PartOfSpeech = "v5r"
	GodanVerbRuEndingIrregularVerb                PartOfSpeech = "v5r-i"
	GodanVerbSuEnding                             PartOfSpeech = "v5s"
	GodanVerbTsuEnding                            PartOfSpeech = "v5t"
	GodanVerbUEnding                              PartOfSpeech = "v5u"
	GodanVerbUEndingSpecialClass                  PartOfSpeech = "v5u-s"
	GodanVerbUruOldClassVerbOldFormOfEru          PartOfSpeech = "v5uru"
	KuruVerbSpecialClass                          PartOfSpeech = "vk"
	IrregularNuVerb                               PartOfSpeech = "vn"
	IrregularRuVerbPlainFormEndsRi                PartOfSpeech = "vr"
	NounOrParticipleWhichTakesTheAuxVerbSuru      PartOfSpeech = "vs"
	SuVerbPrecursorToTheModernSuru                PartOfSpeech = "vs-c"
	SuruVerbIncluded                              PartOfSpeech = "vs-i"
	SuruVerbSpecialClass                          PartOfSpeech = "vs-s"
	IchidanVerbZuruVerbAlternativeFormOfJiruVerbs PartOfSpeech = "vz"
)

const (
	Adjective                PartOfSpeech = "adj"
	Adverb                   PartOfSpeech = "adv"
	Auxiliary                PartOfSpeech = "aux"
	AuxiliaryAdjective       PartOfSpeech = "aux-adj"
	AuxiliaryVerb            PartOfSpeech = "aux-v"
	Copula                   PartOfSpeech = "cop"
	Counter                  PartOfSpeech = "ctr"
	Conjunction              PartOfSpeech = "conj"
	CoordinatingConjunction  PartOfSpeech = "cconj"
	SubordinatingConjunction PartOfSpeech = "sconj"
	Expression               PartOfSpeech = "expr"
	Interjection             PartOfSpeech = "intj"
	IntransitiveVerb         PartOfSpeech = "vi"
	Noun                     PartOfSpeech = "n"
	Numeric                  PartOfSpeech = "num"
	Particle                 PartOfSpeech = "part"
	Prefix                   PartOfSpeech = "pref"
	Preposition              PartOfSpeech = "prep"
	Pronoun                  PartOfSpeech = "pron"
	ProperNoun               PartOfSpeech = "propn"
	Suffix                   PartOfSpeech = "suff"
	Symbol                   PartOfSpeech = "sym"
	TransitiveVerb           PartOfSpeech = "vt"
	Unknown                  PartOfSpeech = "un"
	Verb                     PartOfSpeech = "v"
)

var PartOfSpeechNameMap = map[PartOfSpeech]string{
	Adjective:                "adjective",
	Adverb:                   "adverb",
	Auxillary:                "auxillary",
	Conjunction:              "conjunction",
	CoordinatingConjunction:  "coordinating conjunction",
	SubordinatingConjunction: "subordinating conjunction",
	Expression:               "Expression",
	Interjection:             "interjection",
	Noun:                     "noun",
	Numeric:                  "numeric",
	Particle:                 "particle",
	Prefix:                   "prefix",
	Preposition:              "preposition",
	Pronoun:                  "pronoun",
	ProperNoun:               "proper noun",
	Suffix:                   "suffix",
	Symbol:                   "symbol",
	Unknown:                  "unknown part-of-speech",
	Verb:                     "verb",
}

var PartOfSpeechPOSMap = map[PartOfSpeech]POS{
	Adjective:    POSadj,
	Adverb:       POSadv,
	Article:      POSart,
	Conjugation:  POSconj,
	Interjection: POSintj,
	Noun:         POSn,
	Particle:     POSpart,
	Prefix:       POSpref,
	Preposition:  POSprep,
	Pronoun:      POSpro,
	Suffix:       POSsuff,
	Unknown:      POSun,
	Verb:         POSv,
}
