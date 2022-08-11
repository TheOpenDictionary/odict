package main

/*
#include <stdint.h>

struct DictionaryFile {
    uint16_t version;
		uint16_t length;
		char* bytes;
};
*/
import "C"

import (
	"encoding/json"
	"unsafe"

	odict "github.com/TheOpenDictionary/odict/go"
)

//export CompileDictionary
func CompileDictionary(xmlFilePath *C.char) {
	odict.CompileDictionary(C.GoString(xmlFilePath))
}

//export WriteDictionary
func WriteDictionary(xmlStr, outputPath *C.char) {
	odict.WriteDictionary(C.GoString(xmlStr), C.GoString(outputPath))
}

//export ReadDictionary
func ReadDictionary(path *C.char) *C.struct_DictionaryFile {
	returnStruct := (*C.struct_DictionaryFile)(C.malloc(C.size_t(unsafe.Sizeof(C.struct_DictionaryFile{}))))

	version, bytes := odict.ReadFile(C.GoString(path))

	returnStruct.version = C.uint16_t(version)
	returnStruct.bytes = C.CString(string(bytes))
	returnStruct.length = C.uint16_t(len(bytes))

	return returnStruct
}

//export SearchDictionary
func SearchDictionary(query, dictionaryID *C.char) *C.char {
	q := C.GoString(query)
	id := C.GoString(dictionaryID)
	result := odict.SearchDictionary(id, q, false)
	b, err := json.Marshal(&result)

	odict.Check(err)

	return C.CString(string(b))
}

//export IndexDictionary
func IndexDictionary(path *C.char, force bool) {
	d := odict.ReadDictionaryFromPath(C.GoString(path))
	odict.IndexDictionary(d, force)
}

//export LookupEntry
func LookupEntry(term, dictionaryID *C.char) *C.char {
	q := C.GoString(term)
	id := C.GoString(dictionaryID)
	result := odict.SearchDictionary(id, q, true)

	if len(result) > 0 {
		r := result[0]

		b, err := json.Marshal(&r)

		odict.Check(err)

		return C.CString(string(b))
	}

	return C.CString("{}")
}

func main() {}
