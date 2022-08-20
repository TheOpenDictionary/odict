package odict

import (
	"encoding/xml"
	"io"
	"sort"
	"strings"

	flatbuffers "github.com/google/flatbuffers/go"
	"github.com/imdario/mergo"
)

type DictionaryRepresentable struct {
	ID      string                        `json:"id" xml:"id,attr"`
	Name    string                        `json:"name" xml:"name,attr,omitempty"`
	Entries map[string]EntryRepresentable `json:"entries" xml:"entry"`
	XMLName xml.Name                      `json:"-" xml:"dictionary"`
}

// func (m DictionaryRepresentable) MarshalXML(e *xml.Encoder, start xml.StartElement) error {
// 	e.Encode(m.Entries)

// 	for key := range m.Iterable {
// 		e.Encode(m.Get(key))
// 	}

// 	return nil
// }

func (m *DictionaryRepresentable) UnmarshalXML(d *xml.Decoder, start xml.StartElement) error {
	var entry EntryRepresentable

	d.DecodeElement(&entry, &start)

	if m.Entries == nil {
		m.Entries = make(map[string]EntryRepresentable)
	}

	if _, ok := m.Entries[entry.Term]; ok {
		if err := mergo.Merge(&entry, m.Entries[entry.Term], mergo.WithAppendSlice); err != nil {
			Check(err)
		}
	}

	m.Entries[strings.ToLower(entry.Term)] = entry

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

func (dict *Dictionary) AsRepresentable() DictionaryRepresentable {
	var entry Entry

	entries := make(map[string]EntryRepresentable)

	for a := 0; a < dict.EntriesLength(); a++ {
		dict.Entries(&entry, a)
		representable := entry.AsRepresentable()
		representable.Key = strings.ToLower(representable.Term)
		entries[representable.Key] = representable
	}

	return DictionaryRepresentable{
		ID:      string(dict.Id()),
		Name:    string(dict.Name()),
		Entries: entries,
	}
}

func (dict *DictionaryRepresentable) AsBuffer(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	id := builder.CreateString(dict.ID)
	name := builder.CreateString(dict.Name)
	entries := dict.buildEntryVector(builder)

	DictionaryStart(builder)
	DictionaryAddId(builder, id)
	DictionaryAddName(builder, name)
	DictionaryAddEntries(builder, entries)

	return DictionaryEnd(builder)
}

func (dict *DictionaryRepresentable) buildEntryVector(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	entries := dict.Entries
	entryCount := len(entries)
	keys := make([]string, 0, len(entries))

	// EXTREMELY IMPORTANT!!
	// Because FlatBuffers performs key lookups via binary search, if the keys are not sorted
	// in the vector there may be a number of false negatives when searching
	sort.Strings(keys)

	for key := range entries {
		keys = append(keys, key)
	}

	DictionaryStartEntriesVector(builder, entryCount)

	for i := entryCount - 1; i >= 0; i-- {
		entry := entries[keys[i]]
		builder.PrependUOffsetT(entry.AsBuffer(builder))
	}

	return builder.EndVector(entryCount)
}
