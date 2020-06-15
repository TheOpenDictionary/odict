package odict

import "encoding/xml"

type OpenDictionaryDefinitionGroup struct {
	ID          string   `json:"id" xml:"id,attr"`
	Description string   `json:"description" xml:"description,attr,omitempty"`
	Definitions []string `json:"definitions" xml:"definition"`
}

type OpenDictionaryUsage struct {
	ID               string                          `json:"id" xml:"id,attr"`
	POS              string                          `json:"pos" xml:"pos,attr,omitempty"`
	Definitions      []string                        `json:"definitions" xml:"definition"`
	DefinitionGroups []OpenDictionaryDefinitionGroup `json:"groups" xml:"group"`
	XMLName          xml.Name                        `xml:"usage"`
}

type OpenDictionaryUsageMap map[string]OpenDictionaryUsage

func (m OpenDictionaryUsageMap) MarshalXML(e *xml.Encoder, start xml.StartElement) error {
	for _, v := range m {
		e.Encode(v)
	}

	return nil
}

type OpenDictionaryEtymology struct {
	ID          string                 `json:"id" xml:"id,attr"`
	Description string                 `json:"description" xml:"description,attr,omitempty"`
	Usages      OpenDictionaryUsageMap `json:"usages"`
}

type OpenDictionaryEntry struct {
	ID          string                    `json:"id" xml:"id,attr"`
	Term        string                    `json:"term" xml:"term,attr"`
	Etymologies []OpenDictionaryEtymology `json:"etymologies" xml:"ety"`
	XMLName     xml.Name                  `xml:"entry"`
}

type OpenDictionaryEntryMap map[string]OpenDictionaryEntry

func (m OpenDictionaryEntryMap) MarshalXML(e *xml.Encoder, start xml.StartElement) error {
	for _, v := range m {
		e.Encode(v)
	}

	return nil
}

type OpenDictionary struct {
	ID      string                 `json:"id" xml:"id,attr"`
	Name    string                 `json:"name" xml:"name,attr,omitempty"`
	Entries OpenDictionaryEntryMap `json:"entries"`
	Version uint16                 `json:"version" xml:"-"`
	XMLName xml.Name               `xml:"dictionary"`
}
