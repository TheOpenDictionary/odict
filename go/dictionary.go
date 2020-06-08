package odict

type OpenDictionaryDefinitionGroup struct {
	ID          string		`json:"id"`
	Description string		`json:"description"`
	Definitions []string	`json:"definitions"`
}

type OpenDictionaryUsage struct {
	ID               string														`json:"id"`
	POS              string														`json:"pos"`
	Definitions      []string													`json:"definitions"`
	DefinitionGroups []OpenDictionaryDefinitionGroup	`json:"groups"`
}

type OpenDictionaryEtymology struct {
	ID          string								`json:"id"`
	Description string								`json:"description"`
	Usages      []OpenDictionaryUsage `json:"usages"`
}

type OpenDictionaryEntry struct {
	ID          string 										`json:"id"`
	Term        string 										`json:"term"`
	Etymologies []OpenDictionaryEtymology `json:"etymologies"`
}

type OpenDictionary struct {
	ID string 										`json:"id"`
	Name string										`json:"name"`
	Entries []OpenDictionaryEntry `json:"entries"`
	Version uint16								`json:"version"`
}
