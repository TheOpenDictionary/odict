package main

// #define CGO_EXPORT_BRIDGE_EXISTS
// #include <stdlib.h>
import "C"

import (
	"encoding/base64"
	"encoding/json"
	"unsafe"

	odict "github.com/odict/odict/go"
)

func getDictionaryFromBuffer(db *C.char) odict.Dictionary {
	b, err := base64.StdEncoding.DecodeString(C.GoString(db))

	odict.Check(err)

	return odict.DecodeDictionary(b)
}

// We're sadly left to pass the dictionary around as a base64-encoded string
// because passing pointers from Go doesn't always work as intended
func convertDictionaryToBuffer(dict odict.Dictionary) *C.char {
	d := odict.EncodeDictionary(dict)
	s := base64.StdEncoding.EncodeToString(d)

	return C.CString(s)
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
func ReadDictionary(path *C.char) *C.char {
	return convertDictionaryToBuffer(odict.ReadDictionary(C.GoString(path)))
}

//export SearchDictionary
func SearchDictionary(query, dictionaryID *C.char) *C.char {
	q := C.GoString(query)
	id := C.GoString(dictionaryID)
	result := odict.SearchDictionary(id, q)
	b, err := json.Marshal(&result)

	odict.Check(err)

	return C.CString(string(b))
}

//export Free
func Free(res *C.char) {
	C.free(unsafe.Pointer(res))
}

//export IndexDictionary
func IndexDictionary(path *C.char) {
	d := odict.ReadDictionary(C.GoString(path))
	odict.IndexDictionary(d, true)
}

//export LookupEntry
func LookupEntry(term, dictionary *C.char) *C.char {
	dict := getDictionaryFromBuffer(dictionary)
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
