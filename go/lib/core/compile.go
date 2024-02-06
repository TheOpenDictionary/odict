package core

import (
	"io"
	"os"
)

// CompileDictionary compiles an XML file into an ODict binary
func CompileDictionary(xmlPath string, outputPath string) (int, error) {
	xmlFile, err := os.Open(xmlPath)

	if err != nil {
		return 0, err
	}

	defer xmlFile.Close()

	xmlStr, err := io.ReadAll(xmlFile)

	if err != nil {
		return 0, err
	}

	return WriteDictionaryFromXML(string(xmlStr), outputPath)
}
