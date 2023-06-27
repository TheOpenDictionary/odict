package core

import (
	"bufio"
	"encoding/xml"
	"errors"
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

func xmlToDictionaryRepresentable(xmlStr string) *types.DictionaryRepresentable {
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

	return &dictionary
}

func WriteDictionaryFromExisting(outputPath string, dictionary *types.DictionaryRepresentable) (int, error) {
	dictionaryBytes := types.Serialize(dictionary)
	compressed := snappy.Encode(nil, dictionaryBytes)
	file, err := os.Create(outputPath)

	if err != nil {
		return 0, err
	}

	versionInt, parseErr := strconv.Atoi(version)

	if parseErr != nil {
		return 0, parseErr
	}

	defer file.Close()

	signature := []byte("ODICT")

	version := utils.Uint16ToBytes(uint16(versionInt))

	compressedSize := uint64(len(compressed))

	compressedSizeBytes := utils.Uint64ToBytes(compressedSize)

	writer := bufio.NewWriter(file)

	sigBytes, sigErr := writer.Write(signature)

	if sigErr != nil {
		return 0, sigErr
	}

	versionBytes, versionErr := writer.Write(version)

	if versionErr != nil {
		return 0, versionErr
	}

	contentSizeBytes, contentCountErr := writer.Write(compressedSizeBytes)

	if contentCountErr != nil {
		return 0, contentCountErr
	}

	contentBytes, contentErr := writer.Write(compressed)

	if contentErr != nil {
		return 0, contentErr
	}

	total := sigBytes + versionBytes + contentSizeBytes + contentBytes

	if sigBytes != 5 {
		return 0, errors.New("signature bytes do not equal 5")
	}

	if versionBytes != 2 {
		return 0, errors.New("version bytes do not equal 2")
	}

	if contentSizeBytes != 8 {
		return 0, errors.New("content byte count does not equal 8")
	}

	if contentBytes != int(compressedSize) {
		return 0, errors.New("content does not equal the computed byte count")
	}

	writer.Flush()

	return total, nil
}

// WriteDictionary generates an ODict binary file given
// a ODXML input file path
func WriteDictionaryFromXML(xmlStr, outputPath string) (int, error) {
	return WriteDictionaryFromExisting(outputPath, xmlToDictionaryRepresentable(xmlStr))
}
