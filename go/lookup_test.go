package odict

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestLookup(t *testing.T) {
	CompileDictionary("../examples/example1.xml", "../examples/example1.odict")

	dict := ReadDictionaryFromPath("../examples/example1.odict")
	entries := Map(dict.Lookup([]string{"dog", "cat"}, 0, false), func(entryList []Entry) []EntryRepresentable {
		return Map(entryList, func(entry Entry) EntryRepresentable {
			return entry.AsRepresentable()
		})
	})

	assert.EqualValues(t, EntryRepresentable{
		Term: "dog",
		Etymologies: []EtymologyRepresentable{
			{
				Description: "Latin root",
				Usages: KVMap[PartOfSpeech, UsageRepresentable]{
					Unknown: UsageRepresentable{
						POS:    Unknown,
						Groups: []GroupRepresentable{},
						Definitions: []DefinitionRepresentable{
							{
								Value:    "a dog",
								Examples: []string{},
							},
						},
					},
				},
			},
		},
	}, entries[0][0])

	assert.EqualValues(t, EntryRepresentable{
		Term: "cat",
		Etymologies: []EtymologyRepresentable{
			{
				Description: "Latin root",
				Usages: KVMap[PartOfSpeech, UsageRepresentable]{
					Noun: UsageRepresentable{
						POS:    Noun,
						Groups: []GroupRepresentable{},
						Definitions: []DefinitionRepresentable{
							{
								Value:    "a cat",
								Examples: []string{"There goes a cat!"},
							},
						},
					},
				},
			},
		},
	}, entries[1][0])

	CleanupTest()
}

func TestLookupSplitting(t *testing.T) {
	CompileDictionary("../examples/example1.xml", "../examples/example1.odict")

	dict := ReadDictionaryFromPath("../examples/example1.odict")
	entries := dict.Lookup([]string{"catdog"}, 2, false)

	assert.Equal(t, 1, len(entries))
	assert.Equal(t, 2, len(entries[0]))
	assert.Equal(t, "cat", string(entries[0][0].Term()))
	assert.Equal(t, "dog", string(entries[0][1].Term()))

	CleanupTest()
}

func TestFallbacks(t *testing.T) {
	CompileDictionary("../examples/example1.xml", "../examples/example1.odict")

	dict := ReadDictionaryFromPath("../examples/example1.odict")
	entries := dict.Lookup([]string{"catdog(run)"}, 2, false)

	assert.Equal(t, 1, len(entries))
	assert.Equal(t, "run", string(entries[0][0].Term()))

	entries = dict.Lookup([]string{"(run)"}, 2, false)

	assert.Equal(t, 1, len(entries))
	assert.Equal(t, "run", string(entries[0][0].Term()))

	CleanupTest()
}

func TestFollow(t *testing.T) {
	CompileDictionary("../examples/example2.xml", "../examples/example2.odict")

	dict := ReadDictionaryFromPath("../examples/example2.odict")

	entries1 := dict.Lookup([]string{"runners"}, 2, false)

	assert.Equal(t, len(entries1), 1)
	assert.Equal(t, "runners", string(entries1[0][0].Term()))

	entries2 := dict.Lookup([]string{"runner"}, 2, true)

	assert.Equal(t, 1, len(entries2))
	assert.Equal(t, "runner", string(entries2[0][0].Term()))

	CleanupTest()
}
