package core

import (
	"bufio"
	"encoding/xml"
	"fmt"
	"html"
	"os"
	"regexp"
	"strconv"

	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	"github.com/golang/snappy"
	"github.com/google/uuid"
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

	if len(dictionary.ID) == 0 {
		dictionary.ID = uuid.New().String()
	}

	return dictionary
}

func WriteDictionaryFromExisting(outputPath string, dictionary types.DictionaryRepresentable) int {
	dictionaryBytes := types.Serialize(&dictionary)
	compressed := snappy.Encode(nil, dictionaryBytes)
	file, err := os.Create(outputPath)
	versionInt, parseErr := strconv.Atoi(version)

	utils.Check(err)
	utils.Check(parseErr)

	defer file.Close()

	signature := []byte("ODICT")
	version := utils.Uint16ToBytes(uint16(versionInt))
	compressedSize := uint64(len(compressed))
	compressedSizeBytes := utils.Uint64ToBytes(compressedSize)

	writer := bufio.NewWriter(file)

	sigBytes, sigErr := writer.Write(signature)
	versionBytes, versionErr := writer.Write(version)
	contentSizeBytes, contentCountErr := writer.Write(compressedSizeBytes)
	contentBytes, contentErr := writer.Write(compressed)
	total := sigBytes + versionBytes + contentSizeBytes + contentBytes

	utils.Check(sigErr)
	utils.Check(versionErr)
	utils.Check(contentCountErr)
	utils.Check(contentErr)

	utils.Assert(sigBytes == 5, "Signature bytes do not equal 5")
	utils.Assert(versionBytes == 2, "Version bytes do not equal 2")
	utils.Assert(contentSizeBytes == 8, "Content byte count does not equal 8")
	utils.Assert(contentBytes == int(compressedSize), "Content does not equal the computed byte count")

	writer.Flush()

	return total
}

// WriteDictionary generates an ODict binary file given
// a ODXML input file path
func WriteDictionaryFromXML(xmlStr, outputPath string) int {
	return WriteDictionaryFromExisting(outputPath, xmlToDictionaryRepresentable(xmlStr))
}
