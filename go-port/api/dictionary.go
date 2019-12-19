package api

type OpenDictionaryDefinitionGroup struct {
	ID          string
	Description string
	Definitions []string
}

type OpenDictionaryUsage struct {
	ID               string
	POS              string
	Definitions      []string
	DefinitionGroups []OpenDictionaryDefinitionGroup
}

type OpenDictionaryEtymology struct {
	ID          string
	Description string
	Usages      []OpenDictionaryUsage
}

type OpenDictionaryEntry struct {
	ID          string
	Term        string
	Etymologies []OpenDictionaryEtymology
}

// func (dict *OpenDictionaryEntry) AsJSON() string {
// 	buffer := dict.buffer

// 	var entry schema.Entry

// 	buffer.Entries(&entry, 1)

// 	print(string(entry.Term()))

// 	return "hi"
// }

type OpenDictionary struct {
	Entries []OpenDictionaryEntry
	Version uint16
}
