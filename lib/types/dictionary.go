package types

import (
	"encoding/xml"
	"fmt"
	"html"
	"regexp"
	"sort"

	flatbuffers "github.com/google/flatbuffers/go"
	"github.com/google/uuid"
)

type Dictionary struct {
	ID      string               `json:"id" xml:"id,attr,omitempty"`
	Name    string               `json:"name" xml:"name,attr,omitempty"`
	Entries KVMap[string, Entry] `json:"entries" xml:"entry"`
	XMLName xml.Name             `json:"-" xml:"dictionary"`
}

func NewDictionary(xmlStr string) *Dictionary {
	var dictionary Dictionary

	xml.Unmarshal([]byte(xmlStr), &dictionary)

	r := regexp.MustCompile("<entry.*?term=\"(.*?)\".*?>")
	entries := r.FindAllStringSubmatch(xmlStr, -1)
	expectedEntries := len(entries)
	actualEntries := len(dictionary.Entries)

	if expectedEntries != actualEntries {
		fmt.Printf("WARNING: The dictionary that was read into memory from XML is missing entries. %d entries were read when there should be %d total. Are you sure your XML is 100%% valid and there are no duplicate entries?\n", actualEntries, expectedEntries)

		for _, entry := range entries {
			v := html.UnescapeString(entry[1])

			if _, ok := dictionary.Entries[v]; !ok {
				fmt.Printf("- %s\n", v)
			}
		}
	}

	if len(dictionary.ID) == 0 {
		dictionary.ID = uuid.New().String()
	}

	return &dictionary
}

func (dict *Dictionary) Bytes() []byte {
	return getBytes(dict)
}

func (dict *Dictionary) Buffer() *DictionaryBuffer {
	return GetRootAsDictionaryBuffer(dict.Bytes(), 0)
}

func (dict *DictionaryBuffer) Struct() Dictionary {
	var entry EntryBuffer

	entries := make(map[string]Entry)

	for a := 0; a < dict.EntriesLength(); a++ {
		dict.Entries(&entry, a)
		s := entry.Struct()
		entries[s.Key()] = s
	}

	return Dictionary{
		ID:      string(dict.Id()),
		Name:    string(dict.Name()),
		Entries: entries,
	}
}

func (dict *Dictionary) Table(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	id := builder.CreateString(dict.ID)
	name := builder.CreateString(dict.Name)
	entries := dict.buildEntryVector(builder)

	DictionaryBufferStart(builder)
	DictionaryBufferAddId(builder, id)
	DictionaryBufferAddName(builder, name)
	DictionaryBufferAddEntries(builder, entries)

	return DictionaryBufferEnd(builder)
}

func (dict *Dictionary) buildEntryVector(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
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
		bufs = append(bufs, entry.Table(builder))
	}

	DictionaryBufferStartEntriesVector(builder, entryCount)

	for i := entryCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(bufs[i])
	}

	return builder.EndVector(entryCount)
}
