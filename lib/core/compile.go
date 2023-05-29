package core

import (
	"io/ioutil"
	"os"

	"github.com/TheOpenDictionary/odict/lib/utils"
)

// CompileDictionary compiles an XML file into an ODict binary
func CompileDictionary(xmlPath string, outputPath string) error {
	xmlFile, err := os.Open(xmlPath)

	utils.Check(err)

	defer xmlFile.Close()

	xmlStr, err := ioutil.ReadAll(xmlFile)

	utils.Check(err)

	return WriteDictionaryFromXML(string(xmlStr), outputPath)
}
