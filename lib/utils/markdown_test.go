package utils

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMarkdownToHTML(t *testing.T) {
	str := "**This** is a ^test^ ~of~ the _parser_"
	expected := "<strong>This</strong> is a <sup>test</sup> <sub>of</sub> the <em>parser</em>"
	output := MarkdownToHTML([]byte(str))
	assert.Equal(t, expected, string(output))
}

func TestMarkdownToText(t *testing.T) {
	str := "**This** [is](google.com) a ^test^ ~of~ the _parser_"
	expected := "This is a test of the parser"
	output := MarkdownToText([]byte(str))
	assert.Equal(t, expected, string(output))
}
