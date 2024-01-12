package core

import (
	"testing"

	"github.com/TheOpenDictionary/odict/lib/test"
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/samber/lo"
	"github.com/stretchr/testify/assert"
)

func TestLookup(t *testing.T) {
	CompileDictionary("../../examples/example1.xml", "../../examples/example1.odict")

	dict, err := ReadDictionary("../../examples/example1.odict")

	assert.Equal(t, nil, err)

	entries := lo.Map(Lookup(
		LookupRequest{
			Dictionary: dict,
			Queries:    []string{"dog", "cat"},
			Split:      0,
			Follow:     false,
		},
	), func(entryList []types.Entry, _ int) []types.EntryRepresentable {
		return types.EntriesToRepresentables(entryList)
	})

	assert.EqualValues(t, types.EntryRepresentable{
		Term: "dog",
		Etymologies: []types.EtymologyRepresentable{
			{
				Description:   "Latin root",
				Pronunciation: "dooooog",
				Senses: types.KVMap[types.PartOfSpeech, types.SenseRepresentable]{
					types.Unknown: types.SenseRepresentable{
						POS:    types.Unknown,
						Groups: []types.GroupRepresentable{},
						Definitions: []types.DefinitionRepresentable{
							{
								Value:    "a dog",
								Examples: []string{},
								Notes:    []types.NoteRepresentable{},
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
				Senses: types.KVMap[types.PartOfSpeech, types.SenseRepresentable]{
					types.Noun: types.SenseRepresentable{
						POS:    types.Noun,
						Groups: []types.GroupRepresentable{},
						Definitions: []types.DefinitionRepresentable{
							{
								Value:    "a cat",
								Examples: []string{"There goes a cat!"},
								Notes: []types.NoteRepresentable{
									{
										Value:    "Some definition note",
										Examples: []string{"Some example"},
									},
								},
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

	dict, err := ReadDictionary("../../examples/example1.odict")

	assert.Equal(t, nil, err)

	entries := Lookup(
		LookupRequest{
			Dictionary: dict,
			Queries:    []string{"catdog"},
			Split:      2,
			Follow:     false,
		},
	)

	assert.Equal(t, 1, len(entries))
	assert.Equal(t, 2, len(entries[0]))
	assert.Equal(t, "cat", string(entries[0][0].Term()))
	assert.Equal(t, "dog", string(entries[0][1].Term()))

	test.CleanupTest()
}

func TestFallbacks(t *testing.T) {
	CompileDictionary("../../examples/example1.xml", "../../examples/example1.odict")

	dict, err := ReadDictionary("../../examples/example1.odict")

	assert.Equal(t, nil, err)

	entries := Lookup(
		LookupRequest{
			Dictionary: dict,
			Queries:    []string{"catdog(run)"},
			Split:      2,
			Follow:     false,
		},
	)

	assert.Equal(t, 1, len(entries))
	assert.Equal(t, "run", string(entries[0][0].Term()))

	entries = Lookup(
		LookupRequest{
			Dictionary: dict,
			Queries:    []string{"(run)"},
			Split:      2,
			Follow:     false,
		},
	)

	assert.Equal(t, 1, len(entries))
	assert.Equal(t, "run", string(entries[0][0].Term()))

	test.CleanupTest()
}

func TestFollow(t *testing.T) {
	CompileDictionary("../../examples/example2.xml", "../../examples/example2.odict")

	dict, err := ReadDictionary("../../examples/example2.odict")

	assert.Equal(t, nil, err)

	control := Lookup(
		LookupRequest{
			Dictionary: dict,
			Queries:    []string{"runners"},
			Split:      2,
			Follow:     false,
		},
	)

	assert.Equal(t, len(control), 1)
	assert.Equal(t, "runners", string(control[0][0].Term()))

	basic := Lookup(
		LookupRequest{
			Dictionary: dict,
			Queries:    []string{"runners"},
			Split:      2,
			Follow:     true,
		},
	)

	assert.Equal(t, 1, len(basic))
	assert.Equal(t, "runner", string(basic[0][0].Term()))

	fallback := Lookup(
		LookupRequest{
			Dictionary: dict,
			Queries:    []string{"unfindable (runners)"},
			Split:      2,
			Follow:     true,
		},
	)

	assert.Equal(t, 1, len(fallback))
	assert.Equal(t, "runner", string(fallback[0][0].Term()))

	test.CleanupTest()
}
