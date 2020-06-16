package odict

type DictionaryEtymology struct {
	ID          string             `json:"id" xml:"id,attr"`
	Description string             `json:"description" xml:"description,attr,omitempty"`
	Usages      DictionaryUsageMap `json:"usages" xml:"usage"`
}
