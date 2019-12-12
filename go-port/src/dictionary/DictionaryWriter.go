package dictionary

import (
	"encoding/xml"
	"fmt"
	"io/ioutil"
	"odict/schema"
	"os"
	"time"

	flatbuffers "github.com/google/flatbuffers/go"
)

type DefinitionGroup struct {
	XMLName     xml.Name `xml:"group"`
	Definitions []string `xml:"definition"`
	Description string   `xml:"description,attr"`
}

type Usage struct {
	XMLName          xml.Name          `xml:"usage"`
	POS              string            `xml:"pos,attr"`
	DefinitionGroups []DefinitionGroup `xml:"group"`
	Definitions      []string          `xml:"definition"`
}

type Etymology struct {
	XMLName     xml.Name `xml:"ety"`
	Description string   `xml:"description,attr"`
	Usages      []Usage  `xml:"usage"`
}

type Entry struct {
	XMLName     xml.Name    `xml:"entry"`
	Term        string      `xml:"term,attr"`
	Etymologies []Etymology `xml:"ety"`
}

type Dictionary struct {
	XMLName xml.Name `xml:"dictionary"`
	Name    string   `xml:"name,attr"`
	Entries []Entry  `xml:"entry"`
}

func readFile(path string) *os.File {
	file, err := os.Open(path)

	if err != nil {
		fmt.Println(err)
	}

	defer file.Close()

	return file
}

func xmlToDictionary(file *os.File) Dictionary {
	var dictionary Dictionary

	byteValue, _ := ioutil.ReadAll(file)
	xml.Unmarshal(byteValue, &dictionary)

	return dictionary
}

func dictionaryToBuffer(dictionary Dictionary) {
	builder := flatbuffers.NewBuilder(1024)

}

// GenerateDictionary generates an ODict binary file given
// a ODXML input file path
func GenerateDictionary(inputPath, outputPath string) {
	start := time.Now()

	xmlFile := readFile(inputPath)
	dictionary := xmlToDictionary(xmlFile)
	schema.DictionaryStart()
	elapsed := time.Since(start)

	fmt.Printf("Completed in %f seconds\n", elapsed.Seconds())
}
