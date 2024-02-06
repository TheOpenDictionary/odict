package types

import (
	"bytes"
	"encoding/gob"
	"strings"

	"github.com/samber/lo"
)

func formatPOSTag(tag string) string {
	return strings.ReplaceAll(tag, "_", "-")
}

func strToPartOfSpeech(str string) PartOfSpeech {
	if val, ok := posTagPartOfSpeechMap[formatPOSTag(str)]; ok {
		return val
	}

	return Unknown
}

// EncodeDictionary encodes a dictionary struct
// into a byte array
func EncodeDictionary(dictionary Dictionary) ([]byte, error) {
	var buffer bytes.Buffer

	enc := gob.NewEncoder(&buffer)
	err := enc.Encode(dictionary)

	if err != nil {
		return nil, err
	}

	return buffer.Bytes(), nil
}

// DecodeDictionary decodes a byte array into
// a dictionary object
func DecodeDictionary(b []byte) (*Dictionary, error) {
	var dict Dictionary

	buffer := bytes.NewBuffer(b)
	dec := gob.NewDecoder(buffer)
	err := dec.Decode(&dict)

	if err != nil {
		return nil, err
	}

	return &dict, nil
}

// EncodeDictionary encodes an entry struct
// into a byte array
func EncodeEntry(entry Entry) ([]byte, error) {
	var buffer bytes.Buffer

	enc := gob.NewEncoder(&buffer)
	err := enc.Encode(entry)

	if err != nil {
		return nil, err
	}

	return buffer.Bytes(), nil
}

// DecodeDictionary decodes a byte array into
// an entry object
func DecodeEntry(b []byte) (*Entry, error) {
	var entry Entry

	buffer := bytes.NewBuffer(b)
	dec := gob.NewDecoder(buffer)
	err := dec.Decode(&entry)

	if err != nil {
		return nil, err
	}

	return &entry, nil
}

func NestedEntriesToRepresentables(entries [][]Entry) [][]EntryRepresentable {
	return lo.Map(entries, func(e []Entry, _ int) []EntryRepresentable {
		return EntriesToRepresentables(e)
	})
}

func EntriesToRepresentables(entries []Entry) []EntryRepresentable {
	return lo.Map(entries, func(entry Entry, _ int) EntryRepresentable {
		return entry.AsRepresentable()

	})
}
