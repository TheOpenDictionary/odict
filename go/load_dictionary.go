package odict

import (
	"encoding/binary"
	"os"

	schema "github.com/odict/odict/go/schema"

	"github.com/golang/snappy"
)

func getODDefinitionsFromUsage(usage schema.Usage) []string {
	definitions := []string{}

	for f := 0; f < usage.DefinitionsLength(); f++ {
		definitions = append(definitions, string(usage.Definitions(f)))
	}

	return definitions
}

func getODDefinitionsFromGroup(group schema.Group) []string {
	definitions := []string{}

	for e := 0; e < group.DefinitionsLength(); e++ {
		definitions = append(definitions, string(group.Definitions(e)))
	}

	return definitions
}

func getODDefinitionGroups(usage schema.Usage) []OpenDictionaryDefinitionGroup {
	var definitionGroup schema.Group

	definitionGroups := []OpenDictionaryDefinitionGroup{}

	for d := 0; d < usage.GroupsLength(); d++ {
		usage.Groups(&definitionGroup, d)

		odGroup := OpenDictionaryDefinitionGroup{
			Description: string(definitionGroup.Description()),
			Definitions: getODDefinitionsFromGroup(definitionGroup),
		}

		definitionGroups = append(definitionGroups, odGroup)
	}

	return definitionGroups
}

func getODUsages(etymology schema.Etymology) []OpenDictionaryUsage {
	var usage schema.Usage
	var usages []OpenDictionaryUsage

	for c := 0; c < etymology.UsagesLength(); c++ {
		etymology.Usages(&usage, c)

		odUsage := OpenDictionaryUsage{
			ID:               string(usage.Id()),
			POS:              string(usage.Pos()),
			DefinitionGroups: getODDefinitionGroups(usage),
			Definitions:      getODDefinitionsFromUsage(usage),
		}

		usages = append(usages, odUsage)
	}

	return usages
}

func getODEtymologies(entry schema.Entry) []OpenDictionaryEtymology {
	var ety schema.Etymology
	var etymologies []OpenDictionaryEtymology

	for b := 0; b < entry.EtymologiesLength(); b++ {
		entry.Etymologies(&ety, b)

		odEty := OpenDictionaryEtymology{
			ID:     string(ety.Id()),
			Usages: getODUsages(ety),
		}

		etymologies = append(etymologies, odEty)
	}

	return etymologies
}

func getODEntries(dictionary *schema.Dictionary) []OpenDictionaryEntry {
	var entry schema.Entry
	var entries []OpenDictionaryEntry

	for a := 0; a < dictionary.EntriesLength(); a++ {
		dictionary.Entries(&entry, a)

		odEntry := OpenDictionaryEntry{
			ID:          string(entry.Id()),
			Term:        string(entry.Term()),
			Etymologies: getODEtymologies(entry),
		}

		entries = append(entries, odEntry)
	}

	return entries
}

// LoadDictionary can go fuck itself
func LoadDictionary(inputPath string, newIndex bool) OpenDictionary {
	// Read input file
	file, err := os.Open(inputPath)

	Check(err)

	defer file.Close()

	// Read file signature as bytes
	sigBytes := make([]byte, 5)
	_, sigErr := file.Read(sigBytes)

	Check(sigErr)

	// Read ODict version as bytes
	file.Seek(5, 0)

	versionBytes := make([]byte, 2)
	_, versionError := file.Read(versionBytes)

	Check(versionError)

	// Read the compressed content size in bytes
	file.Seek(7, 0)

	contentSizeBytes := make([]byte, 4)
	_, contentSizeError := file.Read(contentSizeBytes)

	Check(contentSizeError)
	file.Seek(11, 0)

	// Decode bytes for signature, version, and contentSize
	signature := string(sigBytes)
	version := binary.LittleEndian.Uint16(versionBytes)
	contentSize := binary.LittleEndian.Uint32(contentSizeBytes)

	// Assert signature
	Assert(signature == "ODICT", "Invalid file signature")

	// Read compressed buffer content as bytes
	contentBytes := make([]byte, contentSize)

	_, contentError := file.Read(contentBytes)

	Check(contentError)

	decoded, decodedError := snappy.Decode(nil, contentBytes)

	Check(decodedError)

	buffer := schema.GetRootAsDictionary(decoded, 0)
	dictionary := OpenDictionary{
		ID:      string(buffer.Id()),
		Name:    string(buffer.Name()),
		Version: version,
		Entries: getODEntries(buffer),
	}

	createIndex(dictionary, newIndex)

	return dictionary
}
