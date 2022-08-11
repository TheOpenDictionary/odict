package odict

import (
	"bufio"
	"encoding/xml"
	"fmt"
	"html"
	"io/ioutil"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strconv"
	"strings"

	"github.com/golang/snappy"

	"github.com/TheOpenDictionary/odict/schema"
	flatbuffers "github.com/google/flatbuffers/go"
	"github.com/google/uuid"
)

func xmlToDictionary(xmlStr string) Dictionary {
	var dictionary Dictionary

	xml.Unmarshal([]byte(xmlStr), &dictionary)

	r := regexp.MustCompile("<entry.*?term=\"(.*?)\">")
	entries := r.FindAllStringSubmatch(xmlStr, -1)
	expectedEntries := len(entries)
	actualEntries := dictionary.Entries.Size()

	if expectedEntries != actualEntries {

		fmt.Printf("WARNING: The dictionary that was read into memory from XML is missing entries. %d entries were read when there should be %d total. Are you sure your XML is 100%% valid and there are no duplicate entries?\n", actualEntries, expectedEntries)

		for _, entry := range entries {
			v := html.UnescapeString(entry[1])

			if !dictionary.Entries.Has(v) {
				fmt.Printf("- %s\n", v)
			}
		}
	}

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

func getDefinitionsVectorFromGroup(builder *flatbuffers.Builder, group Group) flatbuffers.UOffsetT {
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
	groups := usage.Groups

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

func resolveSchemaPOS(pos PartOfSpeech) schema.POS {
	posMap := map[PartOfSpeech]schema.POS{
		"adjective":    schema.POSadj,
		"adj":          schema.POSadj,
		"adverb":       schema.POSadv,
		"adv":          schema.POSadv,
		"verb":         schema.POSv,
		"v":            schema.POSv,
		"n":            schema.POSn,
		"noun":         schema.POSn,
		"pronoun":      schema.POSpro,
		"pn":           schema.POSpro,
		"prep":         schema.POSprep,
		"preposition":  schema.POSprep,
		"conj":         schema.POSconj,
		"conjugation":  schema.POSconj,
		"intj":         schema.POSintj,
		"interjection": schema.POSintj,
		"prefix":       schema.POSpref,
		"pre":          schema.POSpref,
		"suffix":       schema.POSsuff,
		"suf":          schema.POSsuff,
		"particle":     schema.POSpart,
		"part":         schema.POSpart,
		"article":      schema.POSart,
		"art":          schema.POSart,
		"un":           schema.POSun,
		"unknown":      schema.POSun,
	}

	if val, ok := posMap[pos]; ok {
		return val
	} else if len(strings.TrimSpace(string(pos))) == 0 {
		return schema.POSun
	} else {
		panic(fmt.Sprintf("Compilation error: invalid part-of-speech used: %s", pos))
	}
}

func getUsagesVector(builder *flatbuffers.Builder, ety Etymology) flatbuffers.UOffsetT {
	usages := ety.Usages

	var usageBuffer []flatbuffers.UOffsetT

	for key := range usages.Iterable {
		usage := usages.Get(key)
		usagePOS := resolveSchemaPOS(usage.POS)
		usageDefinitionGroups := getGroupsVector(builder, usage)
		usageDefinitions := getDefinitionsVectorFromUsage(builder, usage)

		schema.UsageStart(builder)
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

	var entriesBuffer []flatbuffers.UOffsetT

	keys := entries.Keys()

	// EXTREMELY IMPORTANT!!
	// Because FlatBuffers performs key lookups via binary search, if the keys are not sorted
	// in the vector there may be a number of false negatives when searching
	sort.Strings(keys)

	for _, key := range keys {
		entry := entries.Get(key)
		entryKey := builder.CreateString(key)
		entryTerm := builder.CreateString(entry.Term)
		entryEtymologies := getEtymologiesVector(builder, entry)

		schema.EntryStart(builder)
		schema.EntryAddKey(builder, entryKey)
		schema.EntryAddTerm(builder, entryTerm)
		schema.EntryAddEtymologies(builder, entryEtymologies)

		entryBuffer := schema.EntryEnd(builder)
		entriesBuffer = append(entriesBuffer, entryBuffer)
	}

	entryCount := len(entriesBuffer)

	schema.DictionaryStartEntriesVector(builder, entryCount)

	for i := entryCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(entriesBuffer[i])
	}

	return builder.EndVector(entryCount)
}

func dictionaryToBytes(dictionary Dictionary) []byte {
	builder := flatbuffers.NewBuilder(0)

	id := builder.CreateString(uuid.New().String())
	name := builder.CreateString(dictionary.Name)
	entries := getEntriesVector(builder, dictionary)

	schema.DictionaryStart(builder)
	schema.DictionaryAddId(builder, id)
	schema.DictionaryAddName(builder, name)
	schema.DictionaryAddEntries(builder, entries)

	builder.Finish(schema.DictionaryEnd(builder))

	return builder.FinishedBytes()
}

// CreateODictFile writes a new .odict binary from a
// Dictionary struct
func CreateODictFile(outputPath string, dictionary Dictionary) {
	dictionaryBytes := dictionaryToBytes(dictionary)
	compressed := snappy.Encode(nil, dictionaryBytes)
	file, err := os.Create(outputPath)

	Check(err)

	defer file.Close()

	signature := []byte("ODICT")
	version := Uint16ToBytes(2)
	compressedSize := uint64(len(compressed))
	compressedSizeBytes := Uint64ToBytes(compressedSize)

	writer := bufio.NewWriter(file)

	sigBytes, sigErr := writer.Write(signature)
	versionBytes, versionErr := writer.Write(version)
	contentSizeBytes, contentCountErr := writer.Write(compressedSizeBytes)
	contentBytes, contentErr := writer.Write(compressed)
	total := sigBytes + versionBytes + contentSizeBytes + contentBytes

	Check(sigErr)
	Check(versionErr)
	Check(contentCountErr)
	Check(contentErr)

	Assert(sigBytes == 5, "Signature bytes do not equal 5")
	Assert(versionBytes == 2, "Version bytes do not equal 2")
	Assert(contentSizeBytes == 8, "Content byte count does not equal 8")
	Assert(contentBytes == int(compressedSize), "Content does not equal the computed byte count")

	writer.Flush()

	fmt.Printf("Wrote %d bytes to path: %s\n", total, outputPath)
}

// WriteDictionary generates an ODict binary file given
// a ODXML input file path
func WriteDictionary(xmlStr, outputPath string) {
	CreateODictFile(outputPath, xmlToDictionary(xmlStr))
}

// CompileDictionary compiles an XML file into an ODict binary
func CompileDictionary(xmlPath string) {
	base := filepath.Base(xmlPath)
	name := strings.TrimSuffix(base, filepath.Ext(base))
	outputPath := fmt.Sprintf("%s/%s.odict", filepath.Dir(xmlPath), name)
	xmlFile, err := os.Open(xmlPath)

	defer xmlFile.Close()

	Check(err)

	xmlStr, err := ioutil.ReadAll(xmlFile)

	Check(err)

	WriteDictionary(string(xmlStr), outputPath)
}
