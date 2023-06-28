package search

// func TestSearch(t *testing.T) {
// 	os.Setenv("ODICT_CONFIG_DIR", ".odict")

// 	core.CompileDictionary("../../examples/example1.xml", "../../examples/example1.odict")

// 	dict, err := core.ReadDictionary("../../examples/example1.odict")

// 	assert.Equal(t, nil, err)

// 	Index(
// 		IndexRequest{
// 			Dictionary: dict,
// 			Overwrite:  true,
// 			Quiet:      true,
// 		},
// 	)

// 	entries, err := SearchDictionary(SearchDictionaryRequest{Dictionary: dict, Query: "run", Exact: false})

// 	assert.Equal(t, nil, err)
// 	assert.Equal(t, string(entries[0].Term()), "run")

// 	test.CleanupTest()
// }
