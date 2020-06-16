package odict

import (
	"encoding/xml"
)

type Dictionary struct {
	ID      string             `json:"id" xml:"id,attr"`
	Name    string             `json:"name" xml:"name,attr,omitempty"`
	Entries DictionaryEntryMap `json:"entries" xml:"entry"`
	Version uint16             `json:"version" xml:"-"`
	XMLName xml.Name           `json:"-" xml:"dictionary"`
}
