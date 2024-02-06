package types

import (
	"github.com/TheOpenDictionary/odict/lib/utils"
)

type MDString string

var markdownStrategy = MarkdownStrategyHTML

type MarkdownStrategy string

const (
	MarkdownStrategyHTML    MarkdownStrategy = "html"
	MarkdownStrategyText    MarkdownStrategy = "text"
	MarkdownStrategyDisable MarkdownStrategy = "disable"
)

func GetMarkdownStrategy() MarkdownStrategy {
	return markdownStrategy
}

func SetMarkdownProcessingStrategy(strategy MarkdownStrategy) {
	markdownStrategy = strategy
}

func (mds MDString) MarshalText() ([]byte, error) {
	return []byte(mds.String()), nil
}

func (mds MDString) String() string {
	output := []byte(mds)

	switch markdownStrategy {
	case MarkdownStrategyHTML:
		output = utils.MarkdownToHTML(output)
	case MarkdownStrategyText:
		output = utils.MarkdownToText(output)
	}

	return string(output)
}
