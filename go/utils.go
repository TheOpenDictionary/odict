package odict

import (
	"bytes"
	"encoding/gob"
	"encoding/binary"
	"log"
)

// EncodeDictionary encodes a dictionary struct 
// into a byte array
func EncodeDictionary(dictionary Dictionary) []byte {
	var buffer bytes.Buffer

	enc := gob.NewEncoder(&buffer)
	err := enc.Encode(dictionary)

	Check(err)

	return buffer.Bytes()
}

// DecodeDictionary decodes a byte array into 
// a dictionary object
func DecodeDictionary(b []byte) Dictionary {
	var dict Dictionary 
	
	buffer := bytes.NewBuffer(b)
	dec := gob.NewDecoder(buffer)
	err := dec.Decode(&dict)

	Check(err)
	
	return dict
}

// EncodeDictionary encodes an entry struct 
// into a byte array
func EncodeEntry(entry Entry) []byte {
	var buffer bytes.Buffer

	enc := gob.NewEncoder(&buffer)
	err := enc.Encode(entry)

	Check(err)

	return buffer.Bytes()
}

// DecodeDictionary decodes a byte array into 
// an entry object
func DecodeEntry(b []byte) Entry {
	var entry Entry 

	buffer := bytes.NewBuffer(b)
	dec := gob.NewDecoder(buffer)
	err := dec.Decode(&entry)

	Check(err)
	
	return entry
}

// Check panics the program if an error exists
func Check(e error) {
	if e != nil {
		panic(e)
	}
}

// Assert panics the program if a condition is not met
func Assert(condition bool, errorMessage string) {
	if !condition {
		log.SetFlags(0)
		log.Fatalln(errorMessage)
	}
}

// Uint16ToBytes converts a uint16 type to a byte array
func Uint16ToBytes(n uint16) []byte {
	bytes := make([]byte, 2)

	// TODO: normalize
	binary.LittleEndian.PutUint16(bytes, uint16(n))

	return bytes
}

// Uint64ToBytes converts a uint64 type to a byte array
func Uint64ToBytes(n uint64) []byte {
	bytes := make([]byte, 8)

	// TODO: normalize
	binary.LittleEndian.PutUint64(bytes, uint64(n))

	return bytes
}
