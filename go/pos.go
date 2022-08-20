package odict

type PartOfSpeech string

const (
	Adjective    PartOfSpeech = "adj"
	Adverb       PartOfSpeech = "adv"
	Verb         PartOfSpeech = "v"
	Noun         PartOfSpeech = "n"
	Pronoun      PartOfSpeech = "pro"
	Preposition  PartOfSpeech = "pre"
	Conjugation  PartOfSpeech = "conj"
	Interjection PartOfSpeech = "interj"
	Prefix       PartOfSpeech = "pref"
	Suffix       PartOfSpeech = "suff"
	Particle     PartOfSpeech = "part"
	Article      PartOfSpeech = "art"
	Unknown      PartOfSpeech = "un"
)
