package main

// #define CGO_EXPORT_BRIDGE_EXISTS
import "C"

import (
	"encoding/json"
	"unsafe"

	odict "github.com/odict/odict/go"
)

//export CompileDictionary
func CompileDictionary(xmlFilePath *C.char) {
	odict.CompileDictionary(C.GoString(xmlFilePath))
}

//export WriteDictionary
func WriteDictionary(xmlStr, outputPath *C.char) {
	odict.WriteDictionary(C.GoString(xmlStr), C.GoString(outputPath))
}

//export ReadDictionaryBuffer
func ReadDictionaryBuffer(path *C.char) (int, unsafe.Pointer) {
	_, _, buffer := odict.ReadODictFile(C.GoString(path))
	return C.int(len(buffer)), unsafe.Pointer(&buffer[0])
}

//export SearchDictionary
func SearchDictionary(query, dictionaryPath *C.char) *C.char {
	path := C.GoString(dictionaryPath)
	q := C.GoString(query)
	dict := odict.ReadDictionary(path)
	result := odict.SearchDictionary(dict, q)

	b, err := json.Marshal(&result)

	odict.Check(err)

	return C.CString(string(b))
}

//export IndexDictionary
func IndexDictionary(dictionaryPath *C.char) {
	dict := odict.ReadDictionary(C.GoString(dictionaryPath))
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
