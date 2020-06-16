package odict

import (
	"encoding/xml"
	"io"
)

type DictionaryEntryMap struct {
	Iterable map[string]DictionaryEntry
}

func (m *DictionaryEntryMap) Set(key string, value DictionaryEntry) {
	m.Iterable[key] = value
}

func (m *DictionaryEntryMap) Get(key string) DictionaryEntry {
	return m.Iterable[key]
}

func (m DictionaryEntryMap) MarshalXML(e *xml.Encoder, start xml.StartElement) error {
	for key := range m.Iterable {
		e.Encode(m.Get(key))
	}
	return nil
}

func (m *DictionaryEntryMap) UnmarshalXML(d *xml.Decoder, start xml.StartElement) error {
	var entry DictionaryEntry

	d.DecodeElement(&entry, &start)

	if m.Iterable == nil {
		m.Iterable = make(map[string]DictionaryEntry)
	}

	m.Set(entry.Term, entry)

	for {
		_, err := d.Token()

		if err != nil {
			if err == io.EOF {
				return nil
			}
			return err
		}
	}
}

type DictionaryEntry struct {
	Term        string                `json:"term" xml:"term,attr"`
	Etymologies []DictionaryEtymology `json:"etymologies" xml:"ety"`
	XMLName     xml.Name              `json:"-" xml:"entry"`
}
