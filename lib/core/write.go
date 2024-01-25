package core

import (
	"os"

	"github.com/TheOpenDictionary/odict/lib/types"
)

func writeToFile(data []byte, outputPath string) (int, error) {
	err := os.WriteFile(outputPath, data, 0644)

	if err != nil {
		return 0, err
	}

	return len(data), nil
}

func WriteDictionary(dictionary *types.Dictionary, outputPath string) (int, error) {
	serialized, err := createFileBytes(dictionary)

	if err != nil {
		return 0, err
	}

	return writeToFile(serialized, outputPath)
}

func WriteXML(xmlStr string, outputPath string) (int, error) {
	return WriteDictionary(types.NewDictionary(xmlStr), outputPath)
}
