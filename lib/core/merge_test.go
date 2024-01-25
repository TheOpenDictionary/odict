package core

import (
	"testing"

	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/stretchr/testify/assert"
)

func TestMerge(t *testing.T) {
	dict1 := types.Dictionary{
		Entries: types.KVMap[string, types.Entry]{
			"dog": types.Entry{
				Term: "dog",
				Etymologies: []types.Etymology{
					{Senses: types.KVMap[types.PartOfSpeech, types.Sense]{
						types.Noun: types.Sense{
							POS: types.Noun,
							Definitions: []types.Definition{
								{Value: "some definition"},
							},
						},
					},
					},
				},
			},
		},
	}

	dict2 := types.Dictionary{
		Entries: types.KVMap[string, types.Entry]{
			"cat": types.Entry{
				Term: "cat",
				Etymologies: []types.Etymology{
					{Senses: types.KVMap[types.PartOfSpeech, types.Sense]{
						types.Noun: types.Sense{
							POS: types.Noun,
							Definitions: []types.Definition{
								{Value: "some other definition"},
							},
						},
					},
					},
				},
			},
		},
	}

	expected := types.Dictionary{
		Entries: types.KVMap[string, types.Entry]{
			"cat": types.Entry{
				Term: "cat",
				Etymologies: []types.Etymology{
					{Senses: types.KVMap[types.PartOfSpeech, types.Sense]{
						types.Noun: types.Sense{
							POS: types.Noun,
							Definitions: []types.Definition{
								{
									Value:    "some other definition",
									Examples: []string{},
									Notes:    []types.Note{},
								},
							},
							Groups: []types.Group{},
						},
					},
					},
				},
			},
			"dog": types.Entry{
				Term: "dog",
				Etymologies: []types.Etymology{
					{Senses: types.KVMap[types.PartOfSpeech, types.Sense]{
						types.Noun: types.Sense{
							POS: types.Noun,
							Definitions: []types.Definition{
								{
									Value:    "some definition",
									Examples: []string{},
									Notes:    []types.Note{},
								},
							},
							Groups: []types.Group{},
						},
					},
					},
				},
			},
		},
	}

	dict1_b := (&dict1).Buffer()
	dict2_b := (&dict2).Buffer()

	merged, err := MergeDictionaries(dict1_b, dict2_b)

	assert.Equal(t, nil, err)
	assert.Equal(t, expected, *merged)
}
