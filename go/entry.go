package odict

import (
	"encoding/xml"
	"io"
)

type EntryMap struct {
	Iterable map[string]Entry
}

func (m *EntryMap) Set(key string, value Entry) {
	m.Iterable[key] = value
}

func (m *EntryMap) Get(key string) Entry {
	return m.Iterable[key]
}

func (m *EntryMap) Size() int {
	return len(m.Iterable)
}

func (m EntryMap) MarshalXML(e *xml.Encoder, start xml.StartElement) error {
	for key := range m.Iterable {
		e.Encode(m.Get(key))
	}
	return nil
}

func (m *EntryMap) UnmarshalXML(d *xml.Decoder, start xml.StartElement) error {
	var entry Entry

	d.DecodeElement(&entry, &start)

	if m.Iterable == nil {
		m.Iterable = make(map[string]Entry)
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

type Entry struct {
	Term        string      `json:"term" xml:"term,attr"`
	Etymologies []Etymology `json:"etymologies" xml:"ety"`
	XMLName     xml.Name    `json:"-" xml:"entry"`
}
