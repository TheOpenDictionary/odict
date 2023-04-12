package core

import (
	"io/ioutil"
	"os"
)

// CompileDictionary compiles an XML file into an ODict binary
func CompileDictionary(xmlPath string, outputPath string) int {
	xmlFile, err := os.Open(xmlPath)

	Check(err)

	defer xmlFile.Close()

	xmlStr, err := ioutil.ReadAll(xmlFile)

	Check(err)

	return WriteDictionaryFromXML(string(xmlStr), outputPath)
}
