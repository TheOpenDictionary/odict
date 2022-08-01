package odict

import (
	"encoding/xml"
	"io"
	"strings"

	"github.com/imdario/mergo"
)

type EntryMap struct {
	Iterable map[string]Entry
}

func (m *EntryMap) Set(key string, value Entry) {
	m.Iterable[strings.ToLower(key)] = value
}

func (m *EntryMap) Get(key string) Entry {
	return m.Iterable[strings.ToLower(key)]
}

func (m *EntryMap) Has(key string) bool {
	_, ok := m.Iterable[strings.ToLower(key)]
	return ok
}

func (m *EntryMap) Keys() []string {
	keys := make([]string, 0, len(m.Iterable))

	for k := range m.Iterable {
		keys = append(keys, k)
	}

	return keys
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

	if m.Has(entry.Term) {
		if err := mergo.Merge(&entry, m.Get(entry.Term), mergo.WithAppendSlice); err != nil {
			Check(err)
		}
	}

	m.Set(strings.ToLower(entry.Term), entry)

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
