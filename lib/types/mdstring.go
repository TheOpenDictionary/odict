package types

import (
	"github.com/TheOpenDictionary/odict/lib/utils"
)

type MDString string

var markdownEnabled = true

func IsMarkdownProcessingEnabled() bool {
	return markdownEnabled
}

func SetMarkdownProcessingEnabled(enabled bool) {
	markdownEnabled = enabled
}

func (mds MDString) MarshalText() ([]byte, error) {
	output := []byte(mds)

	if IsMarkdownProcessingEnabled() {
		output = utils.MarkdownToHTML([]byte(mds))
	}

	return output, nil
}

func (mds MDString) String() string {
	return string(mds)
}
