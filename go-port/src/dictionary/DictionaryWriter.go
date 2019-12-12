package dictionary

import (
	"encoding/xml"
	"fmt"
	"io/ioutil"
	"odict/schema"
	"os"
	"strconv"
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

	id := builder.CreateString("id") // TODO: replace
	name := builder.CreateString(dictionary.Name)

	weaponTwo := builder.CreateString("Axe")

	schema.DictionaryStart(builder)
	schema.DictionaryAddId(builder, id)
	schema.DictionaryAddName(builder, name)

	var entryBuffer []flatbuffers.UOffsetT

	for entryIdx, entry := range dictionary.Entries {
		entryId := schema.CreateString(entryIdx) // TODO: add prefix
		entryTerm := schema.CreateString(entry.Term)

		schema.EntryStart(builder)
		schema.EntryAddId(builder, entryId)
		schema.EntryAddTerm(builder, entryTerm)

		var etyBuffer []flatbuffers.UOffsetT

		for etyIdx, ety := range entry.Etymologies {
			etyID := builder.CreateString(strconv.Itoa(etyIdx))
			etyDescription := builder.CreateString(ety.Description)

			schema.EtymologyStart(builder)
			schema.EtymologyAddId(builder, etyID)
			schema.EtymologyAddDescription(builder, etyDescription)

			var usageBuffer []flatbuffers.UOffsetT

			for usageIdx, usage := range ety.Usages {
				schema.UsageStart(builder)
				schema.UsageAddId(builder, usageIdx)
				schema.UsageAddPos(builder, usage.POS)

				// var groupBuffer []byte

				// for groupIdx, group := range group.Usage {
				// 	schema.GroupStart(builder)
				// 	schema.GroupAddId(builder, groupIdx)
				// 	schema.GroupAddDescription(builder, group.Description)
				// }

				append(usageBuffer, schema.UsageEnd(builder))
			}

			append(etyBuffer, schema.EtymologyEnd(builder))
		}

		append(entryBuffer, schema.EntryEnd(builder))
	}

	println(entryBuffer)

	// schema.DictionaryStartEntriesVector(builder)

	// schema.DictionaryAddEntries(
	// 	builder,
	// 	schema.EntryStart
	// )

	return builder.Finish()
}

// WriteDictionary generates an ODict binary file given
// a ODXML input file path
func WrtieDictionary(inputPath, outputPath string) {
	start := time.Now()

	xmlFile := readFile(inputPath)
	dictionary := xmlToDictionary(xmlFile)

	elapsed := time.Since(start)

	fmt.Printf("Completed in %f seconds\n", elapsed.Seconds())
}
