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

func getUsagesVector(builder *flatbuffers.Builder, ety Etymology) flatbuffers.UOffsetT {
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

	usageCount := len(usageBuffer)

	schema.EtymologyStartUsagesVector(builder, usageCount)

	for i := usageCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(usageBuffer[i])
	}

	return builder.EndVector(usageCount)
}

func getEtymologiesVector(builder *flatbuffers.Builder, entry Entry) flatbuffers.UOffsetT {
	var etyBuffer []flatbuffers.UOffsetT

	for etyIdx, ety := range entry.Etymologies {
		etyID := builder.CreateString(strconv.Itoa(etyIdx))
		etyDescription := builder.CreateString(ety.Description)
		etyUsages := getUsagesVector(builder, ety)

		schema.EtymologyStart(builder)
		schema.EtymologyAddId(builder, etyID)
		schema.EtymologyAddDescription(builder, etyDescription)
		schema.EtymologyAddUsages(builder, etyUsages)

		etyBuffer = append(etyBuffer, schema.EtymologyEnd(builder))
	}

	etyCount := len(etyBuffer)

	schema.EntryStartEtymologiesVector(builder, etyCount)

	for i := etyCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(etyBuffer[i])
	}

	return builder.EndVector(etyCount)
}

func getEntriesVector(builder *flatbuffers.Builder, dictionary Dictionary) flatbuffers.UOffsetT {
	var entryBuffer []flatbuffers.UOffsetT

	for entryIdx, entry := range dictionary.Entries {
		entryID := builder.CreateString(strconv.Itoa(entryIdx)) // TODO: add prefix
		entryTerm := builder.CreateString(entry.Term)
		entryEtymologies := getEtymologiesVector(builder, entry)

		schema.EntryStart(builder)
		schema.EntryAddId(builder, entryID)
		schema.EntryAddTerm(builder, entryTerm)
		schema.EntryAddEtymologies(builder, entryEtymologies)

		entryBuffer = append(entryBuffer, schema.EntryEnd(builder))
	}

	entryCount := len(entryBuffer)

	schema.DictionaryStartEntriesVector(builder, entryCount)

	for i := entryCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(entryBuffer[i])
	}

	return builder.EndVector(entryCount)
}

func dictionaryToBytes(dictionary Dictionary) []byte {
	builder := flatbuffers.NewBuilder(1024)

	id := builder.CreateString("id") // TODO: replace
	name := builder.CreateString(dictionary.Name)
	entries := getEntriesVector(builder, dictionary)

	schema.DictionaryStart(builder)
	schema.DictionaryAddId(builder, id)
	schema.DictionaryAddName(builder, name)
	schema.DictionaryAddEntries(builder, entries)

	builder.Finish(schema.DictionaryEnd(builder))

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
