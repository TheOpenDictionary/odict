package config

import (
	"os"
	"testing"

	"github.com/TheOpenDictionary/odict/lib/test"
	"github.com/stretchr/testify/assert"
)

func TestAddDictionaryAlias(t *testing.T) {
	os.Setenv("ODICT_CONFIG_PATH", ".odict")

	err1 := AddDictionaryAlias("eng", "mydict1.eng")
	path1, err2 := GetDictionaryPathFromAlias("eng")

	assert.Equal(t, path1, "mydict1.eng")
	assert.Nil(t, err1)
	assert.Nil(t, err2)

	err3 := AddDictionaryAlias("eng", "mydict2.eng")
	path2, err4 := GetDictionaryPathFromAlias("eng")

	assert.NotNil(t, err3)
	assert.Nil(t, err4)
	assert.Equal(t, path2, "mydict1.eng")

	test.CleanupTest()
}

func TestRemoveDictionaryAlias(t *testing.T) {
	os.Setenv("ODICT_CONFIG_PATH", ".odict")

	err1 := AddDictionaryAlias("eng", "mydict.eng")
	path1, err2 := GetDictionaryPathFromAlias("eng")

	assert.Equal(t, path1, "mydict.eng")
	assert.Nil(t, err1)
	assert.Nil(t, err2)

	err3 := RemoveDictionaryAlias("eng")
	path2, err4 := GetDictionaryPathFromAlias("eng")

	assert.Equal(t, path2, "")
	assert.Nil(t, err3)
	assert.Nil(t, err4)

	err5 := RemoveDictionaryAlias("eng")

	assert.NotNil(t, err5)

	test.CleanupTest()
}

func TestSetDictionaryAlias(t *testing.T) {
	os.Setenv("ODICT_CONFIG_PATH", ".odict")

	err1 := SetDictionaryAlias("eng", "mydict1.eng")
	path1, err2 := GetDictionaryPathFromAlias("eng")

	assert.Equal(t, path1, "mydict1.eng")
	assert.Nil(t, err1)
	assert.Nil(t, err2)

	err3 := SetDictionaryAlias("eng", "mydict2.eng")
	path2, err4 := GetDictionaryPathFromAlias("eng")

	assert.Equal(t, path2, "mydict2.eng")
	assert.Nil(t, err3)
	assert.Nil(t, err4)

	test.CleanupTest()
}

func TestListDictionaryAlias(t *testing.T) {
	os.Setenv("ODICT_CONFIG_PATH", ".odict")

	AddDictionaryAlias("eng", "mydict.eng")

	dicts, err := ListDictionaries()

	assert.Nil(t, err)
	assert.Equal(t, len(dicts), 1)
	assert.Equal(t, dicts[0].Name, "eng")
	assert.Equal(t, dicts[0].Path, "mydict.eng")

	test.CleanupTest()
}
