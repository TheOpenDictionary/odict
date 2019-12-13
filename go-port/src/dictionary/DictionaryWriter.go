package dictionary

import (
	"bufio"
	"encoding/binary"
	"encoding/xml"
	"fmt"
	"io/ioutil"
	"odict/schema"
	"os"
	"strconv"
	"time"

	"github.com/golang/snappy"
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

	return file
}

func xmlToDictionary(file *os.File) Dictionary {
	var dictionary Dictionary

	byteValue, _ := ioutil.ReadAll(file)
	xml.Unmarshal(byteValue, &dictionary)

	return dictionary
}

func getDefinitionsVectorFromUsage(builder *flatbuffers.Builder, usage Usage) flatbuffers.UOffsetT {
	definitions := usage.Definitions

	var defBuffer []flatbuffers.UOffsetT

	for idx := range definitions {
		defBuffer = append(defBuffer, builder.CreateString(definitions[idx]))
	}

	defCount := len(defBuffer)

	schema.GroupStartDefinitionsVector(builder, defCount)

	for i := defCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(defBuffer[i])
	}

	return builder.EndVector(defCount)
}

func getDefinitionsVectorFromGroup(builder *flatbuffers.Builder, group DefinitionGroup) flatbuffers.UOffsetT {
	definitions := group.Definitions

	var defBuffer []flatbuffers.UOffsetT

	for idx := range definitions {
		defBuffer = append(defBuffer, builder.CreateString(definitions[idx]))
	}

	defCount := len(defBuffer)

	schema.GroupStartDefinitionsVector(builder, defCount)

	for i := defCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(defBuffer[i])
	}

	return builder.EndVector(defCount)
}

func getGroupsVector(builder *flatbuffers.Builder, usage Usage) flatbuffers.UOffsetT {
	groups := usage.DefinitionGroups

	var groupBuffer []flatbuffers.UOffsetT

	for idx := range groups {
		group := groups[idx]
		groupID := builder.CreateString(strconv.Itoa(idx))
		groupDescription := builder.CreateString(group.Description)
		groupDefinitions := getDefinitionsVectorFromGroup(builder, group)

		schema.GroupStart(builder)
		schema.GroupAddId(builder, groupID)
		schema.GroupAddDescription(builder, groupDescription)
		schema.EtymologyAddUsages(builder, groupDefinitions)

		groupBuffer = append(groupBuffer, schema.EtymologyEnd(builder))
	}

	groupCount := len(groupBuffer)

	schema.UsageStartGroupsVector(builder, groupCount)

	for i := groupCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(groupBuffer[i])
	}

	return builder.EndVector(groupCount)
}

func getUsagesVector(builder *flatbuffers.Builder, ety Etymology) flatbuffers.UOffsetT {
	usages := ety.Usages

	var usageBuffer []flatbuffers.UOffsetT

	for idx := range usages {
		usage := usages[idx]
		usageID := builder.CreateString(strconv.Itoa(idx))
		usagePOS := builder.CreateString(usage.POS)
		usageDefinitionGroups := getGroupsVector(builder, usage)
		usageDefinitions := getDefinitionsVectorFromUsage(builder, usage)

		schema.UsageStart(builder)
		schema.UsageAddId(builder, usageID)
		schema.UsageAddPos(builder, usagePOS)
		schema.UsageAddGroups(builder, usageDefinitionGroups)
		schema.UsageAddDefinitions(builder, usageDefinitions)

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
	etymologies := entry.Etymologies

	var etyBuffer []flatbuffers.UOffsetT

	for idx := range etymologies {
		ety := etymologies[idx]
		etyID := builder.CreateString(strconv.Itoa(idx))
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
	entries := dictionary.Entries

	var entryBuffer []flatbuffers.UOffsetT

	for idx := range entries {
		entry := entries[idx]
		entryID := builder.CreateString(strconv.Itoa(idx)) // TODO: add prefix
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

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func assert(condition bool, errorMessage string) {
	if !condition {
		panic("Assertion failed: " + errorMessage)
	}
}

func uint16ToBytes(n uint16) []byte {
	bytes := make([]byte, 2)

	// TODO: normalize
	binary.LittleEndian.PutUint16(bytes, uint16(n))

	return bytes
}

func uint32ToBytes(n uint32) []byte {
	bytes := make([]byte, 4)

	// TODO: normalize
	binary.LittleEndian.PutUint32(bytes, uint32(n))

	return bytes
}

func createODictFile(outputPath string, dictionary Dictionary) {
	dictionaryBytes := dictionaryToBytes(dictionary)
	compressed := snappy.Encode(nil, dictionaryBytes)
	file, err := os.Create(outputPath)

	check(err)

	defer file.Close()

	signature := []byte("ODICT")
	version := uint16ToBytes(2)
	compressedSize := uint32(len(compressed))
	compressedBytes := uint32ToBytes(compressedSize)

	writer := bufio.NewWriter(file)

	sigBytes, sigErr := writer.Write(signature)
	versionBytes, versionErr := writer.Write(version)
	contentCountBytes, contentCountErr := writer.Write(compressedBytes)
	contentBytes, contentErr := writer.Write(compressed)
	total := sigBytes + versionBytes + contentCountBytes + contentBytes

	check(sigErr)
	check(versionErr)
	check(contentCountErr)
	check(contentErr)

	assert(sigBytes == 5, "Signature bytes do not equal 5")
	assert(versionBytes == 2, "Version bytes do not equal 2")
	assert(contentCountBytes == 4 || contentCountBytes == 8, "Content byte count does not equal 4 or 8")
	assert(contentBytes == int(compressedSize), "Content does not equal the computed byte count")

	writer.Flush()

	fmt.Printf("Wrote %d bytes to path: %s\n", total, outputPath)
}

// WriteDictionary generates an ODict binary file given
// a ODXML input file path
func WriteDictionary(inputPath, outputPath string) {
	start := time.Now()
	xmlFile := readFile(inputPath)

	defer xmlFile.Close()

	createODictFile(outputPath, xmlToDictionary(xmlFile))

	elapsed := time.Since(start)

	fmt.Printf("Completed in %f seconds\n", elapsed.Seconds())
}
