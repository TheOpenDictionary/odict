package core

import (
	"testing"

	"github.com/TheOpenDictionary/odict/lib/test"
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	"github.com/stretchr/testify/assert"
)

func TestLookup(t *testing.T) {
	CompileDictionary("../../examples/example1.xml", "../../examples/example1.odict")

	dict := ReadDictionaryFromPath("../../examples/example1.odict")
	entries := utils.Map(Lookup(dict, []string{"dog", "cat"}, 0, false), func(entryList []types.Entry) []types.EntryRepresentable {
		return utils.Map(entryList, func(entry types.Entry) types.EntryRepresentable {
			return entry.AsRepresentable()
		})
	})

	assert.EqualValues(t, types.EntryRepresentable{
		Term:          "dog",
		Pronunciation: "dooooog",
		Etymologies: []types.EtymologyRepresentable{
			{
				Description: "Latin root",
				Usages: types.KVMap[types.PartOfSpeech, types.UsageRepresentable]{
					types.Unknown: types.UsageRepresentable{
						POS:    types.Unknown,
						Groups: []types.GroupRepresentable{},
						Definitions: []types.DefinitionRepresentable{
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

	assert.EqualValues(t, types.EntryRepresentable{
		Term: "cat",
		Etymologies: []types.EtymologyRepresentable{
			{
				Description: "Latin root",
				Usages: types.KVMap[types.PartOfSpeech, types.UsageRepresentable]{
					types.Noun: types.UsageRepresentable{
						POS:    types.Noun,
						Groups: []types.GroupRepresentable{},
						Definitions: []types.DefinitionRepresentable{
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

	test.CleanupTest()
}

func TestLookupSplitting(t *testing.T) {
	CompileDictionary("../../examples/example1.xml", "../../examples/example1.odict")

	dict := ReadDictionaryFromPath("../../examples/example1.odict")
	entries := Lookup(dict, []string{"catdog"}, 2, false)

	assert.Equal(t, 1, len(entries))
	assert.Equal(t, 2, len(entries[0]))
	assert.Equal(t, "cat", string(entries[0][0].Term()))
	assert.Equal(t, "dog", string(entries[0][1].Term()))

	test.CleanupTest()
}

func TestFallbacks(t *testing.T) {
	CompileDictionary("../../examples/example1.xml", "../../examples/example1.odict")

	dict := ReadDictionaryFromPath("../../examples/example1.odict")
	entries := Lookup(dict, []string{"catdog(run)"}, 2, false)

	assert.Equal(t, 1, len(entries))
	assert.Equal(t, "run", string(entries[0][0].Term()))

	entries = Lookup(dict, []string{"(run)"}, 2, false)

	assert.Equal(t, 1, len(entries))
	assert.Equal(t, "run", string(entries[0][0].Term()))

	test.CleanupTest()
}

func TestFollow(t *testing.T) {
	CompileDictionary("../../examples/example2.xml", "../../examples/example2.odict")

	dict := ReadDictionaryFromPath("../../examples/example2.odict")

	control := Lookup(dict, []string{"runners"}, 2, false)

	assert.Equal(t, len(control), 1)
	assert.Equal(t, "runners", string(control[0][0].Term()))

	basic := Lookup(dict, []string{"runners"}, 2, true)

	assert.Equal(t, 1, len(basic))
	assert.Equal(t, "runner", string(basic[0][0].Term()))

	fallback := Lookup(dict, []string{"unfindable (runners)"}, 2, true)

	assert.Equal(t, 1, len(fallback))
	assert.Equal(t, "runner", string(fallback[0][0].Term()))

	test.CleanupTest()
}
