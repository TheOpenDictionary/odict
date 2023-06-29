package core

import (
	"os"
	"testing"

	"github.com/TheOpenDictionary/odict/lib/config"
	"github.com/TheOpenDictionary/odict/lib/test"
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/suite"
)

type TestReadSuite struct {
	suite.Suite
	path string
	dict *types.DictionaryRepresentable
}

func (suite *TestReadSuite) SetupTest() {
	os.Setenv("ODICT_CONFIG_DIR", ".odict")

	suite.dict = GetDictionaryFromXML("<dictionary><entry term=\"run\" /></dictionary>")
	suite.path = "../../examples/read.odict"

	WriteDictionaryFromExisting(suite.path, suite.dict)
}

func (suite *TestReadSuite) TearDownSuite() {
	test.CleanupTest()
}

func (suite *TestReadSuite) TestFromPath() {
	dict, err := ReadDictionary(suite.path)

	assert.Equal(suite.T(), nil, err)
	assert.Equal(suite.T(), suite.dict.ID, string(dict.Id()))
}

func (suite *TestReadSuite) TestFromAlias() {
	config.AddDictionaryAlias("test", suite.path)

	dict, err := ReadDictionary("test")

	assert.Equal(suite.T(), nil, err)
	assert.Equal(suite.T(), suite.dict.ID, string(dict.Id()))
}

func TestRead(t *testing.T) {
	suite.Run(t, new(TestReadSuite))
}
