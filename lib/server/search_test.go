package server

import (
	"net/http"
	"net/http/httptest"
	"os"
	"strings"
	"testing"

	"github.com/TheOpenDictionary/odict/lib/core"
	ods "github.com/TheOpenDictionary/odict/lib/search"
	"github.com/TheOpenDictionary/odict/lib/test"
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	"github.com/stretchr/testify/suite"
)

type TestSearchSuite struct {
	suite.Suite
	path string
	dict *types.Dictionary
}

func (suite *TestSearchSuite) SetupTest() {
	os.Setenv("ODICT_CONFIG_DIR", ".odict")

	core.CompileDictionary("../../examples/example2.xml", "../../examples/example2.odict", nil)

	suite.path = "../../examples/example2.odict"

	dict, err := core.ReadDictionary(suite.path, nil)

	suite.Equal(nil, err)

	suite.dict = dict

	_, err = ods.Index(ods.IndexRequest{Dictionary: dict, Quiet: true})

	suite.Equal(nil, err)
}

func (suite *TestSearchSuite) TearDownTest() {
	test.CleanupTest()
}

func (suite *TestSearchSuite) TestSearchSingleDictionary() {
	request, _ := http.NewRequest(http.MethodGet, "/search?query=To%20sail%20before%20the%20wind", nil)

	response := httptest.NewRecorder()

	handler := handleSearch(suite.path)

	entries, err := ods.SearchDictionary(
		ods.SearchDictionaryRequest{
			Dictionary: suite.dict,
			Query:      "To sail before the wind",
		},
	)

	suite.Equal(nil, err)

	representable := types.EntriesToRepresentables(entries)

	json, err := utils.SerializeToJSON(representable, false)

	suite.Equal(nil, err)

	handler(response, request)

	suite.Equal(http.StatusOK, response.Code)
	suite.Equal("application/json", response.Header().Get("Content-Type"))
	suite.Equal(strings.TrimSpace(json), strings.TrimSpace(response.Body.String()))
}

func TestServerSearch(t *testing.T) {
	suite.Run(t, new(TestSearchSuite))
}
