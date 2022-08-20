package odict

import (
	"encoding/xml"
	"fmt"
	"html"
	"regexp"

	flatbuffers "github.com/google/flatbuffers/go"
)

func xmlToDictionaryRepresentable(xmlStr string) DictionaryRepresentable {
	var dictionary DictionaryRepresentable

	xml.Unmarshal([]byte(xmlStr), &dictionary)

	r := regexp.MustCompile("<entry.*?term=\"(.*?)\">")
	entries := r.FindAllStringSubmatch(xmlStr, -1)
	expectedEntries := len(entries)
	actualEntries := len(dictionary.Entries)

	if expectedEntries != actualEntries {

		fmt.Printf("WARNING: The dictionary that was read into memory from XML is missing entries. %d entries were read when there should be %d total. Are you sure your XML is 100%% valid and there are no duplicate entries?\n", actualEntries, expectedEntries)

		for _, entry := range entries {
			v := html.UnescapeString(entry[1])

			if _, ok := dictionary.Entries[v]; ok {
				fmt.Printf("- %s\n", v)
			}
		}
	}

	return dictionary
}

func dictionaryToBytes(dictionary DictionaryRepresentable) []byte {
	builder := flatbuffers.NewBuilder(0)
	buffer := dictionary.AsBuffer(builder)

	builder.Finish(buffer)

	return builder.FinishedBytes()
}

func entryToBytes(entry EntryRepresentable) []byte {
	builder := flatbuffers.NewBuilder(0)
	buffer := entry.AsBuffer(builder)

	builder.Finish(buffer)

	return builder.FinishedBytes()
}
