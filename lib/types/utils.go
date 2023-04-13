package types

import (
	"bytes"
	"encoding/gob"

	"github.com/TheOpenDictionary/odict/lib/utils"
)

func strToPartOfSpeech(str string) PartOfSpeech {
	if val, ok := posTagPartOfSpeechMap[str]; ok {
		return val
	}

	return Unknown
}

// EncodeDictionary encodes a dictionary struct
// into a byte array
func EncodeDictionary(dictionary Dictionary) []byte {
	var buffer bytes.Buffer

	enc := gob.NewEncoder(&buffer)
	err := enc.Encode(dictionary)

	utils.Check(err)

	return buffer.Bytes()
}

// DecodeDictionary decodes a byte array into
// a dictionary object
func DecodeDictionary(b []byte) Dictionary {
	var dict Dictionary

	buffer := bytes.NewBuffer(b)
	dec := gob.NewDecoder(buffer)
	err := dec.Decode(&dict)

	utils.Check(err)

	return dict
}

// EncodeDictionary encodes an entry struct
// into a byte array
func EncodeEntry(entry Entry) []byte {
	var buffer bytes.Buffer

	enc := gob.NewEncoder(&buffer)
	err := enc.Encode(entry)

	utils.Check(err)

	return buffer.Bytes()
}

// DecodeDictionary decodes a byte array into
// an entry object
func DecodeEntry(b []byte) Entry {
	var entry Entry

	buffer := bytes.NewBuffer(b)
	dec := gob.NewDecoder(buffer)
	err := dec.Decode(&entry)

	utils.Check(err)

	return entry
}
