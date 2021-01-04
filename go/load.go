package odict

import (
	"encoding/binary"
	"fmt"
	"os"

	"github.com/odict/odict/go/models"
	"github.com/odict/odict/go/schema"

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

func getODDefinitionGroups(usage schema.Usage) []models.Group {
	var definitionGroup schema.Group

	definitionGroups := []models.Group{}

	for d := 0; d < usage.GroupsLength(); d++ {
		usage.Groups(&definitionGroup, d)

		odGroup := models.Group{
			Description: string(definitionGroup.Description()),
			Definitions: getODDefinitionsFromGroup(definitionGroup),
		}

		definitionGroups = append(definitionGroups, odGroup)
	}

	return definitionGroups
}

func getODUsages(etymology schema.Etymology) models.UsageMap {
	var usage schema.Usage

	usages := models.UsageMap{make(map[models.PartOfSpeech]models.Usage)}

	for c := 0; c < etymology.UsagesLength(); c++ {
		etymology.Usages(&usage, c)

		odUsage := models.Usage{
			POS:         resolvePOS(usage.Pos()),
			Groups:      getODDefinitionGroups(usage),
			Definitions: getODDefinitionsFromUsage(usage),
		}

		usages.Set(odUsage.POS, odUsage)
	}

	return usages
}

func getODEtymologies(entry schema.Entry) []models.Etymology {
	var ety schema.Etymology
	var etymologies []models.Etymology

	for b := 0; b < entry.EtymologiesLength(); b++ {
		entry.Etymologies(&ety, b)

		odEty := models.Etymology{
			ID:     string(ety.Id()),
			Usages: getODUsages(ety),
		}

		etymologies = append(etymologies, odEty)
	}

	return etymologies
}

func getODEntries(dictionary *schema.Dictionary) models.EntryMap {
	var entry schema.Entry

	entries := models.EntryMap{make(map[string]models.Entry)}

	for a := 0; a < dictionary.EntriesLength(); a++ {
		dictionary.Entries(&entry, a)

		odEntry := models.Entry{
			Term:        string(entry.Term()),
			Etymologies: getODEtymologies(entry),
		}

		entries.Set(odEntry.Term, odEntry)
	}

	return entries
}

// LoadDictionary loads a compiled ODict dictionary from the provided
// path and returns a Dictionary model, with the ability to forcibly re-index
// the dictionary when it loads
func LoadDictionary(inputPath string, newIndex bool) models.Dictionary {
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
	dictionary := models.Dictionary{
		ID:      string(buffer.Id()),
		Name:    string(buffer.Name()),
		Version: version,
		Entries: getODEntries(buffer),
	}

	createIndex(dictionary, newIndex)

	return dictionary
}

func resolvePOS(pos schema.POS) models.PartOfSpeech {
	posMap := map[schema.POS]models.PartOfSpeech{
		schema.POSadj:      models.Adjective,
		schema.POSadv:      models.Adverb,
		schema.POSverb:     models.Verb,
		schema.POSnoun:     models.Noun,
		schema.POSpronoun:  models.Pronoun,
		schema.POSprep:     models.Preposition,
		schema.POSconj:     models.Conjugation,
		schema.POSintj:     models.Interjection,
		schema.POSprefix:   models.Prefix,
		schema.POSsuffix:   models.Suffix,
		schema.POSparticle: models.Particle,
		schema.POSarticle:  models.Article,
		schema.POSunknown:  models.Unknown,
	}

	if val, ok := posMap[pos]; ok {
		return val
	} else {
		panic(fmt.Sprintf("Compilation error: invalid part-of-speech used: %s", pos))
	}
}
