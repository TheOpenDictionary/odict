package odict

// Lexicon returns all originally-cased entries
// of a compiled dictionary
func (dict *Dictionary) Lexicon() []string {
	words := []string{}

	for i := 0; i < dict.EntriesLength(); i++ {
		var entry Entry
		dict.Entries(&entry, i)
		words = append(words, string(entry.Term()))
	}

	return words
}
