package main

/*
#include <stdint.h>

struct DictionaryFile {
		unsigned int version;
		unsigned int length;
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
	version, bytes := odict.ReadFile(C.GoString(path))

	returnStruct := (*C.struct_DictionaryFile)(C.malloc(C.size_t(unsafe.Sizeof(C.struct_DictionaryFile{}))))
	returnStruct.version = C.unsigned(version)
	returnStruct.bytes = C.CString(string(bytes))
	returnStruct.length = C.unsigned(len(bytes))

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
func IndexDictionary(path *C.char, force bool) *C.char {
	d := odict.ReadDictionaryFromPath(C.GoString(path))

	odict.IndexDictionary(d, force)

	return C.CString(d.ID)
}

//export LookupEntry
func LookupEntry(term *C.char, file *C.struct_DictionaryFile, splitting C.int) *C.char {
	buf := C.GoBytes(unsafe.Pointer(file.bytes), C.int(file.length))
	dict := odict.ReadDictionaryFromBuffer(uint16(file.version), buf)
	query := C.GoString(term)

	return C.CString(odict.Lookup(dict, query, int(splitting)))
}

func main() {}
