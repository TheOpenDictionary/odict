package core

import (
	"encoding/xml"
	"fmt"
	"html"
	"io/ioutil"
	"regexp"
	"strconv"

	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	"github.com/golang/snappy"
)

func xmlToDictionaryRepresentable(xmlStr string) types.DictionaryRepresentable {
	var dictionary types.DictionaryRepresentable

	xml.Unmarshal([]byte(xmlStr), &dictionary)

	r := regexp.MustCompile("<entry.*?term=\"(.*?)\".*?>")
	entries := r.FindAllStringSubmatch(xmlStr, -1)
	expectedEntries := len(entries)
	actualEntries := len(dictionary.Entries)

	if expectedEntries != actualEntries {
		fmt.Printf("WARNING: The dictionary that was read into memory from XML is missing entries. %d entries were read when there should be %d total. Are you sure your XML is 100%% valid and there are no duplicate entries?\n", actualEntries, expectedEntries)

		for _, entry := range entries {
			v := html.UnescapeString(entry[1])

			if _, ok := dictionary.Entries[v]; !ok {
				fmt.Printf("- %s\n", v)
			}
		}
	}

	return dictionary
}

func writeBytesToFile(outputPath string, data []byte) error {
	err := ioutil.WriteFile(outputPath, data, 0644)

	if err != nil {
		return err
	}

	return nil
}

func serializeDictionary(dictionary types.DictionaryRepresentable) []byte {
	dictionaryBytes := types.Serialize(&dictionary)
	compressed := snappy.Encode(nil, dictionaryBytes)
	versionInt, parseErr := strconv.Atoi(version)

	utils.Check(parseErr)

	signature := []byte("ODICT")
	version := utils.Uint16ToBytes(uint16(versionInt))
	compressedSize := uint64(len(compressed))
	compressedSizeBytes := utils.Uint64ToBytes(compressedSize)
	totalSize := len(signature) + len(version) + len(compressedSizeBytes) + len(compressed)

	output := make([]byte, 0, totalSize)

	output = append(output, signature...)
	output = append(output, version...)
	output = append(output, compressedSizeBytes...)
	output = append(output, compressed...)

	utils.Assert(len(signature) == 5, "Signature bytes do not equal 5")
	utils.Assert(len(version) == 2, "Version bytes do not equal 2")
	utils.Assert(len(compressedSizeBytes) == 8, "Content byte count does not equal 8")
	utils.Assert(len(compressed) == int(compressedSize), "Content does not equal the computed byte count")

	return output
}

func WriteDictionaryToDisk(outputPath string, dictionary types.DictionaryRepresentable) error {
	return writeBytesToFile(outputPath, serializeDictionary(dictionary))
}

// WriteDictionary generates an ODict binary file given
// a ODXML input file path
func WriteDictionaryFromXML(xmlStr, outputPath string) error {
	return WriteDictionaryToDisk(outputPath, xmlToDictionaryRepresentable(xmlStr))
}

func GetDictionaryBytesFromXML(xmlStr string) []byte {
	return serializeDictionary(xmlToDictionaryRepresentable(xmlStr))
}
