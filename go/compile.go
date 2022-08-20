package odict

import (
	"io/ioutil"
	"os"
)

// CompileDictionary compiles an XML file into an ODict binary
func CompileDictionary(xmlPath string, outputPath string) {
	xmlFile, err := os.Open(xmlPath)

	defer xmlFile.Close()

	Check(err)

	xmlStr, err := ioutil.ReadAll(xmlFile)

	Check(err)

	WriteDictionary(string(xmlStr), outputPath)
}
