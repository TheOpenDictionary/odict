package core

import (
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

func writeBytesToFile(outputPath string, data []byte) (int, error) {
	err := os.WriteFile(outputPath, data, 0644)

	if err != nil {
		return 0, err
	}

	return len(data), nil
}

func serializeDictionary(dictionary *types.DictionaryRepresentable) ([]byte, error) {
	dictionaryBytes := types.Serialize(dictionary)
	compressed := snappy.Encode(nil, dictionaryBytes)
	versionInt, parseErr := strconv.Atoi(version)

	if parseErr != nil {
		return nil, parseErr
	}

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

	if len(signature) != 5 {
		return nil, errors.New("signature bytes do not equal 5")
	}

	if len(version) != 2 {
		return nil, errors.New("version bytes do not equal 2")
	}

	if len(compressedSizeBytes) != 8 {
		return nil, errors.New("content byte count does not equal 8")
	}

	if len(compressed) != int(compressedSize) {
		return nil, errors.New("content does not equal the computed byte count")
	}

	return output, nil
}

func GetDictionaryFromXML(xmlStr string) *types.DictionaryRepresentable {
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

func WriteDictionaryToDisk(outputPath string, dictionary *types.DictionaryRepresentable) (int, error) {
	serialized, err := serializeDictionary(dictionary)

	if err != nil {
		return 0, err
	}

	return writeBytesToFile(outputPath, serialized)
}

// WriteDictionary generates an ODict binary file given
// a ODXML input file path
func WriteDictionaryFromXML(xmlStr, outputPath string) (int, error) {
	return WriteDictionaryToDisk(outputPath, GetDictionaryFromXML(xmlStr))
}

func GetDictionaryBytesFromXML(xmlStr string) ([]byte, error) {
	return serializeDictionary(GetDictionaryFromXML(xmlStr))
}
