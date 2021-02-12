package odict

import (
	"encoding/binary"
	"os"

	"github.com/golang/snappy"
	"github.com/TheOpenDictionary/odict/schema/go"
)

func getDefinitionsFromUsage(usage schema.Usage) []string {
	definitions := []string{}

	for f := 0; f < usage.DefinitionsLength(); f++ {
		definitions = append(definitions, string(usage.Definitions(f)))
	}

	return definitions
}

func getDefinitionsFromGroup(group schema.Group) []string {
	definitions := []string{}

	for e := 0; e < group.DefinitionsLength(); e++ {
		definitions = append(definitions, string(group.Definitions(e)))
	}

	return definitions
}

func getDefinitionGroupModels(usage schema.Usage) []Group {
	var definitionGroup schema.Group

	definitionGroups := []Group{}

	for d := 0; d < usage.GroupsLength(); d++ {
		usage.Groups(&definitionGroup, d)

		odGroup := Group{
			Description: string(definitionGroup.Description()),
			Definitions: getDefinitionsFromGroup(definitionGroup),
		}

		definitionGroups = append(definitionGroups, odGroup)
	}

	return definitionGroups
}

func getUsageMap(etymology schema.Etymology) UsageMap {
	var usage schema.Usage

	usages := UsageMap{make(map[PartOfSpeech]Usage)}

	for c := 0; c < etymology.UsagesLength(); c++ {
		etymology.Usages(&usage, c)

		odUsage := Usage{
			POS:         PartOfSpeech(usage.Pos().String()),
			Groups:      getDefinitionGroupModels(usage),
			Definitions: getDefinitionsFromUsage(usage),
		}

		usages.Set(odUsage.POS, odUsage)
	}

	return usages
}

func getEtymologyModels(entry schema.Entry) []Etymology {
	var ety schema.Etymology
	var etymologies []Etymology

	for b := 0; b < entry.EtymologiesLength(); b++ {
		entry.Etymologies(&ety, b)

		odEty := Etymology{
			ID:     string(ety.Id()),
			Usages: getUsageMap(ety),
		}

		etymologies = append(etymologies, odEty)
	}

	return etymologies
}

func getEntryModels(dictionary *schema.Dictionary) EntryMap {
	var entry schema.Entry

	entries := EntryMap{make(map[string]Entry)}

	for a := 0; a < dictionary.EntriesLength(); a++ {
		dictionary.Entries(&entry, a)

		odEntry := Entry{
			Term:        string(entry.Term()),
			Etymologies: getEtymologyModels(entry),
		}

		entries.Set(odEntry.Term, odEntry)
	}

	return entries
}

// ReadDictionaryBuffer reads the Flatbuffers buffer
// from a filepath. This is heavily used by non-Go
// languages because Go can't export its structs, so
// we need to use language-specific Flatbuffer libraries
// to decode the buffer.
func ReadODictFile(path string) (string, uint16, []byte) {
	// Read input file
	file, err := os.Open(path)

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

	contentSizeBytes := make([]byte, 8)
	_, contentSizeError := file.Read(contentSizeBytes)

	Check(contentSizeError)

	file.Seek(15, 0)

	// Decode bytes for signature, version, and contentSize
	signature := string(sigBytes)
	version := binary.LittleEndian.Uint16(versionBytes)
	contentSize := binary.LittleEndian.Uint64(contentSizeBytes)

	// Assert signature
	Assert(signature == "ODICT", "Invalid file signature")

	// Read compressed buffer content as bytes
	contentBytes := make([]byte, contentSize)

	_, contentError := file.Read(contentBytes)

	Check(contentError)

	decoded, decodedError := snappy.Decode(nil, contentBytes)

	Check(decodedError)

	return signature, version, decoded
}

// ReadDictionary loads a compiled ODict dictionary from the provided
// path and returns a Dictionary model, with the ability to forcibly re-index
// the dictionary when it loads
func ReadDictionary(path string) Dictionary {
	_, version, bytes := ReadODictFile(path)

	buffer := schema.GetRootAsDictionary(bytes, 0)

	dictionary := Dictionary{
		ID:      string(buffer.Id()),
		Name:    string(buffer.Name()),
		Version: version,
		Entries: getEntryModels(buffer),
	}

	return dictionary
}
