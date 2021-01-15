package main

// #define CGO_EXPORT_BRIDGE_EXISTS
import "C"

import (
	"encoding/json"

	odict "github.com/odict/odict/go"
)

func getDictionaryFromBuffer(buffer *C.char) odict.Dictionary {
	return odict.DecodeDictionary([]byte(C.GoString(buffer)))
}

//export CompileDictionary
func CompileDictionary(xmlFilePath *C.char) {
	odict.CompileDictionary(C.GoString(xmlFilePath))
}

//export WriteDictionary
func WriteDictionary(xmlStr, outputPath *C.char) {
	odict.WriteDictionary(C.GoString(xmlStr), C.GoString(outputPath))
}

//export ReadDictionary
func ReadDictionary(path *C.char) (C.int, *C.char) {
	dict := odict.ReadDictionary(C.GoString(path))
	buffer := odict.EncodeDictionary(dict)
	print(len(string(buffer)))
	print("|")
	return C.int(len(buffer)), C.CString(string(buffer))
}

//export SearchDictionary
func SearchDictionary(query, dictionaryBuffer *C.char) *C.char {
	q := C.GoString(query)
	dict := getDictionaryFromBuffer(dictionaryBuffer)
	result := odict.SearchDictionary(dict.ID, q)
	b, err := json.Marshal(&result)

	odict.Check(err)

	return C.CString(string(b))
}

//export IndexDictionary
func IndexDictionary(dictionaryBuffer *C.char) {
	print(len(C.GoString(dictionaryBuffer)))
	print("|")
	dict := getDictionaryFromBuffer(dictionaryBuffer)
	odict.IndexDictionary(dict, true)
}

//export LookupEntry
func LookupEntry(term, dictionaryPath *C.char) *C.char {
	dict := odict.ReadDictionary(C.GoString(dictionaryPath))
	query := C.GoString(term)

	if dict.Entries.Has(query) {
		entry := dict.Entries.Get(query)

		b, err := json.Marshal(&entry)

		odict.Check(err)

		return C.CString(string(b))
	}

	return C.CString("{}")
}

func main() {}
