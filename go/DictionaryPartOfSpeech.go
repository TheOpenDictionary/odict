package odict

type DictionaryPartOfSpeech string

const (
	Adjective    DictionaryPartOfSpeech = "adj"
	Adverb                              = "adv"
	Verb                                = "v"
	Noun                                = "n"
	Pronoun                             = "pro"
	Preposition                         = "pre"
	Conjugation                         = "conj"
	Interjection                        = "interj"
	Prefix                              = "pref"
	Suffix                              = "suff"
	Particle                            = "part"
	Article                             = "art"
	Unknown                             = "un"
)
