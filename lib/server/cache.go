package server

import (
	"errors"
	"net/http"
	"sync"

	"github.com/TheOpenDictionary/odict/lib/core"
	"github.com/TheOpenDictionary/odict/lib/types"
)

type dictionaryCache struct {
	data map[string]*types.Dictionary
	mu   sync.Mutex
}

var cache = dictionaryCache{
	data: make(map[string]*types.Dictionary),
}

func (c *dictionaryCache) getDictionary(pathOrAlias string) (*types.Dictionary, error) {
	c.mu.Lock()

	defer c.mu.Unlock()

	if _, ok := c.data[pathOrAlias]; !ok {
		dict, err := core.ReadDictionary(pathOrAlias, nil)

		if err != nil {
			return nil, err
		}

		c.data[pathOrAlias] = dict
	}

	return c.data[pathOrAlias], nil
}

func getDictionary(pathOrAlias string, r *http.Request) (*types.Dictionary, error) {
	dictionary := r.URL.Query().Get("dictionary")

	var dict *types.Dictionary
	var err error

	if len(pathOrAlias) > 0 && len(dictionary) > 0 {
		return nil, errors.New("'dictionary' parameter cannot be used when a single dictionary is being served directly")
	} else if len(pathOrAlias) > 0 {
		dict, err = cache.getDictionary(pathOrAlias)
	} else if len(dictionary) > 0 {
		dict, err = cache.getDictionary(dictionary)
	} else {
		return nil, errors.New("'dictionary' parameter must be used when a single dictionary is not being served")
	}

	if err != nil {
		return nil, err
	}

	return dict, nil
}
