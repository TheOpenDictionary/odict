package server

import (
	"net/http"
	"net/http/httptest"
	"os"
	"strings"
	"testing"

	"github.com/TheOpenDictionary/lib/config"
	"github.com/TheOpenDictionary/odict/lib/core"
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	"github.com/stretchr/testify/suite"
)

type TestLookupSuite struct {
	suite.Suite
	path string
	dict *types.Dictionary
}

func (suite *TestLookupSuite) SetupTest() {
	os.Setenv("ODICT_CONFIG_DIR", ".odict")

	core.CompileDictionary("../../examples/example1.xml", "../../examples/example1.odict")

	suite.path = "../../examples/example1.odict"

	dict, err := core.ReadDictionary(suite.path)

	suite.Equal(nil, err)

	suite.dict = dict
}

func (suite *TestLookupSuite) TestLookupSingleDictionary() {
	request, _ := http.NewRequest(http.MethodGet, "/lookup?query=hot(dog)&query=run&query=catdog&query=ran&split=3&follow=1", nil)

	response := httptest.NewRecorder()

	handler := handleLookup(suite.path)

	entries := core.Lookup(
		core.LookupRequest{
			Dictionary: suite.dict,
			Split:      3,
			Follow:     true,
			Queries:    []string{"hot(dog)", "run", "catdog", "ran"},
		},
	)

	representable := types.NestedEntriesToRepresentables(entries)

	json, err := utils.SerializeToJSON(representable, false)

	suite.Equal(nil, err)

	handler(response, request)

	suite.Equal(http.StatusOK, response.Code)
	suite.Equal("application/json", response.Header().Get("Content-Type"))
	suite.Equal(strings.TrimSpace(json), strings.TrimSpace(response.Body.String()))
}

func (suite *TestLookupSuite) TestLookupMultiDictionary() {
	request, _ := http.NewRequest(http.MethodGet, "/lookup?dictionary=lookup&split=3&follow=1&query=hot(dog)&query=run&query=catdog&query=ran", nil)

	response := httptest.NewRecorder()

	config.AddDictionaryAlias("lookup", suite.path)

	handler := handleLookup("")

	entries := core.Lookup(
		core.LookupRequest{
			Dictionary: suite.dict,
			Split:      3,
			Follow:     true,
			Queries:    []string{"hot(dog)", "run", "catdog", "ran"},
		},
	)

	representable := types.NestedEntriesToRepresentables(entries)

	json, err := utils.SerializeToJSON(representable, false)

	suite.Equal(nil, err)

	handler(response, request)

	suite.Equal(http.StatusOK, response.Code)
	suite.Equal("application/json", response.Header().Get("Content-Type"))
	suite.Equal(strings.TrimSpace(json), strings.TrimSpace(response.Body.String()))
}

func TestServerLookup(t *testing.T) {
	suite.Run(t, new(TestLookupSuite))
}
