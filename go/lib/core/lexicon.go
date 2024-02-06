package core

import "github.com/TheOpenDictionary/odict/lib/types"

// Lexicon returns all originally-cased entries
// of a compiled dictionary
func Lexicon(dict *types.Dictionary) []string {
	words := []string{}

	for i := 0; i < dict.EntriesLength(); i++ {
		var entry types.Entry
		dict.Entries(&entry, i)
		words = append(words, string(entry.Term()))
	}

	return words
}
