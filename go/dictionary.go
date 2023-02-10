package odict

import (
	"encoding/xml"
	"sort"

	flatbuffers "github.com/google/flatbuffers/go"
)

type DictionaryRepresentable struct {
	ID      string                            `json:"id" xml:"id,attr" gorm:"column:id;primary_key;AUTO_INCREMENT"`
	Name    string                            `json:"name" xml:"name,attr,omitempty"`
	Entries KVMap[string, EntryRepresentable] `json:"entries" xml:"entry"`
	XMLName xml.Name                          `json:"-" xml:"dictionary"`
}

func (dict *Dictionary) AsRepresentable() DictionaryRepresentable {
	var entry Entry

	entries := make(map[string]EntryRepresentable)

	for a := 0; a < dict.EntriesLength(); a++ {
		dict.Entries(&entry, a)
		representable := entry.AsRepresentable()
		entries[representable.Key()] = representable
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
	entryCount := len(dict.Entries)
	keys := make([]string, 0, len(entries))
	bufs := make([]flatbuffers.UOffsetT, 0, len(entries))

	for key := range entries {
		keys = append(keys, key)
	}

	// EXTREMELY IMPORTANT!!
	// Because FlatBuffers performs key lookups via binary search, if the keys are not sorted
	// in the vector there may be a number of false negatives when searching
	sort.Strings(keys)

	for _, key := range keys {
		entry := entries[key]
		bufs = append(bufs, entry.AsBuffer(builder))
	}

	DictionaryStartEntriesVector(builder, entryCount)

	for i := entryCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(bufs[i])
	}

	return builder.EndVector(entryCount)
}
