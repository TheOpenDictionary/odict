package main

import "C"

import odict "github.com/odict/odict/go"

// //export CreateDictionaryFromPath
// func CreateDictionaryFromPath(inputPath *C.char) {
// 	createDictionaryFromPath(C.GoString(inputPath))
// }

//export CreateDictionaryFromXML
func CreateDictionaryFromXML(xmlStr, outputPath *C.char) {
	odict.WriteDictionary(C.GoString(xmlStr), C.GoString(outputPath))
}

func main() {}
