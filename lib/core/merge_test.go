package core

import (
	"testing"

	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/stretchr/testify/assert"
)

func TestMerge(t *testing.T) {
	dict1 := types.DictionaryRepresentable{
		Entries: types.KVMap[string, types.EntryRepresentable]{
			"dog": types.EntryRepresentable{
				Term: "dog",
				Etymologies: []types.EtymologyRepresentable{
					{Senses: types.KVMap[types.PartOfSpeech, types.SenseRepresentable]{
						types.Noun: types.SenseRepresentable{
							POS: types.Noun,
							Definitions: []types.DefinitionRepresentable{
								{Value: "some definition"},
							},
						},
					},
					},
				},
			},
		},
	}

	dict2 := types.DictionaryRepresentable{
		Entries: types.KVMap[string, types.EntryRepresentable]{
			"cat": types.EntryRepresentable{
				Term: "cat",
				Etymologies: []types.EtymologyRepresentable{
					{Senses: types.KVMap[types.PartOfSpeech, types.SenseRepresentable]{
						types.Noun: types.SenseRepresentable{
							POS: types.Noun,
							Definitions: []types.DefinitionRepresentable{
								{Value: "some other definition"},
							},
						},
					},
					},
				},
			},
		},
	}

	expected := types.DictionaryRepresentable{
		Entries: types.KVMap[string, types.EntryRepresentable]{
			"cat": types.EntryRepresentable{
				Term: "cat",
				Etymologies: []types.EtymologyRepresentable{
					{Senses: types.KVMap[types.PartOfSpeech, types.SenseRepresentable]{
						types.Noun: types.SenseRepresentable{
							POS: types.Noun,
							Definitions: []types.DefinitionRepresentable{
								{
									Value:    "some other definition",
									Examples: []string{},
									Notes:    []types.NoteRepresentable{},
								},
							},
							Groups: []types.GroupRepresentable{},
						},
					},
					},
				},
			},
			"dog": types.EntryRepresentable{
				Term: "dog",
				Etymologies: []types.EtymologyRepresentable{
					{Senses: types.KVMap[types.PartOfSpeech, types.SenseRepresentable]{
						types.Noun: types.SenseRepresentable{
							POS: types.Noun,
							Definitions: []types.DefinitionRepresentable{
								{
									Value:    "some definition",
									Examples: []string{},
									Notes:    []types.NoteRepresentable{},
								},
							},
							Groups: []types.GroupRepresentable{},
						},
					},
					},
				},
			},
		},
	}

	dict1_b := types.GetRootAsDictionary(types.Serialize(&dict1), 0)
	dict2_b := types.GetRootAsDictionary(types.Serialize(&dict2), 0)

	merged, err := MergeDictionaries(dict1_b, dict2_b)

	assert.Equal(t, nil, err)
	assert.Equal(t, expected, *merged)
}
