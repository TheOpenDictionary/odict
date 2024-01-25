package sql

import (
	"math/rand"
	"testing"

	"github.com/TheOpenDictionary/odict/lib/core"
	"github.com/TheOpenDictionary/odict/lib/test"
	"github.com/gkampitakis/go-snaps/snaps"
	"github.com/google/uuid"
	"github.com/stretchr/testify/assert"
)

func TestDictionaryToSQL(t *testing.T) {
	_, err := core.CompilePath("../../examples/example1.xml", "../../examples/example1.odict")

	assert.Equal(t, nil, err)

	dict, err := core.ReadDictionary("../../examples/example1.odict")

	assert.Equal(t, nil, err)

	dict_r := dict.Struct()

	assert.Equal(t, nil, err)

	uuid.SetRand(rand.New(rand.NewSource(1)))

	sql, err := DictionaryToSQL(dict_r, "postgres", true)

	assert.Equal(t, nil, err)

	snaps.MatchSnapshot(t, sql)

	test.CleanupTest()
}
