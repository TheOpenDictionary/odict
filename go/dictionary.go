package odict

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

type OpenDictionary struct {
	ID string 
	Name string
	Entries []OpenDictionaryEntry
	Version uint16
}
