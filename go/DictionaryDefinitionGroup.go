package odict

type DictionaryDefinitionGroup struct {
	ID          string   `json:"id" xml:"id,attr"`
	Description string   `json:"description" xml:"description,attr,omitempty"`
	Definitions []string `json:"definitions" xml:"definition"`
}
