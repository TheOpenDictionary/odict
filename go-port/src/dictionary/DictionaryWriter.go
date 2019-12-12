package dictionary

import (
	"encoding/xml"
	"fmt"
	flatbuffers "github.com/google/flatbuffers/go"
	"io/ioutil"
	"odict/schema"
	"os"
	"strconv"
	"time"
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

	return file
}

func xmlToDictionary(file *os.File) Dictionary {
	var dictionary Dictionary

	byteValue, _ := ioutil.ReadAll(file)
	xml.Unmarshal(byteValue, &dictionary)

	return dictionary
}

func dictionaryToBytes(dictionary Dictionary) []byte {
	builder := flatbuffers.NewBuilder(1024)

	id := builder.CreateString("id") // TODO: replace
	name := builder.CreateString(dictionary.Name)

	var entryBuffer []flatbuffers.UOffsetT

	for entryIdx, entry := range dictionary.Entries {
		entryID := builder.CreateString(strconv.Itoa(entryIdx)) // TODO: add prefix
		entryTerm := builder.CreateString(entry.Term)

		var etyBuffer []flatbuffers.UOffsetT

		for etyIdx, ety := range entry.Etymologies {
			etyID := builder.CreateString(strconv.Itoa(etyIdx))
			etyDescription := builder.CreateString(ety.Description)

			var usageBuffer []flatbuffers.UOffsetT

			for usageIdx, usage := range ety.Usages {
				usageID := builder.CreateString(strconv.Itoa(usageIdx))
				usagePOS := builder.CreateString(usage.POS)

				schema.UsageStart(builder)
				schema.UsageAddId(builder, usageID)
				schema.UsageAddPos(builder, usagePOS)

				// var groupBuffer []byte

				// for groupIdx, group := range group.Usage {
				// 	schema.GroupStart(builder)
				// 	schema.GroupAddId(builder, groupIdx)
				// 	schema.GroupAddDescription(builder, group.Description)
				// }

				usageBuffer = append(usageBuffer, schema.UsageEnd(builder))
			}

			schema.EtymologyStart(builder)
			schema.EtymologyAddId(builder, etyID)
			schema.EtymologyAddDescription(builder, etyDescription)
			schema.pre
			schema.EtymologyAddUsages(builder, usageBuffer)
			etyBuffer = append(etyBuffer, schema.EtymologyEnd(builder))
		}

		schema.EntryStart(builder)
		schema.EntryAddId(builder, entryID)
		schema.EntryAddTerm(builder, entryTerm)

		entryBuffer = append(entryBuffer, schema.EntryEnd(builder))
	}

	schema.DictionaryStart(builder)
	schema.DictionaryAddId(builder, id)
	schema.DictionaryAddName(builder, name)

	finishedDictionary := schema.DictionaryEnd(builder)

	// schema.DictionaryStartEntriesVector(builder)

	// schema.DictionaryAddEntries(
	// 	builder,
	// 	schema.EntryStart
	// )

	builder.Finish(finishedDictionary)

	return builder.FinishedBytes()
}

// WriteDictionary generates an ODict binary file given
// a ODXML input file path
func WriteDictionary(inputPath, outputPath string) {
	start := time.Now()

	xmlFile := readFile(inputPath)

	dictionary := xmlToDictionary(xmlFile)
	dictionaryBytes := dictionaryToBytes(dictionary)

	defer xmlFile.Close()

	println(dictionaryBytes)

	// println(dictionaryBytes)
	elapsed := time.Since(start)

	fmt.Printf("Completed in %f seconds\n", elapsed.Seconds())
}
