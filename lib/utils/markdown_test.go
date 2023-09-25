package utils

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMarkdownToHTML(t *testing.T) {
	str := "**This** is a ^test^ of the _parser_"
	expected := "<strong>This</strong> is a <sup>test</sup> of the <em>parser</em>"
	output := MarkdownToHTML([]byte(str))
	assert.Equal(t, expected, string(output))
}
