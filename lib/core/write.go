package core

import (
	"bufio"
	"encoding/xml"
	"fmt"
	"html"
	"os"
	"regexp"
	"strconv"

	"github.com/golang/snappy"
)

func xmlToDictionaryRepresentable(xmlStr string) DictionaryRepresentable {
	var dictionary DictionaryRepresentable

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

func WriteDictionaryFromExisting(outputPath string, dictionary DictionaryRepresentable) int {
	dictionaryBytes := serialize(&dictionary)
	compressed := snappy.Encode(nil, dictionaryBytes)
	file, err := os.Create(outputPath)
	versionInt, parseErr := strconv.Atoi(version)

	Check(err)
	Check(parseErr)

	defer file.Close()

	signature := []byte("ODICT")
	version := Uint16ToBytes(uint16(versionInt))
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

	return total
}

// WriteDictionary generates an ODict binary file given
// a ODXML input file path
func WriteDictionaryFromXML(xmlStr, outputPath string) int {
	return WriteDictionaryFromExisting(outputPath, xmlToDictionaryRepresentable(xmlStr))
}
