package utils

import (
	"io"

	"github.com/gomarkdown/markdown"
	"github.com/gomarkdown/markdown/ast"
	"github.com/gomarkdown/markdown/html"
	"github.com/gomarkdown/markdown/parser"
)

var markdownExtensions = parser.NoIntraEmphasis |
	parser.Autolink |
	parser.Strikethrough |
	parser.MathJax |
	parser.SuperSubscript |
	parser.Includes

var htmlFlags = html.CommonFlags | html.HrefTargetBlank

var htmlRenderOptions = html.RendererOptions{Flags: htmlFlags, RenderNodeHook: mdRenderHook}

func mdRenderHook(w io.Writer, node ast.Node, entering bool) (ast.WalkStatus, bool) {
	skipPara := false

	if _, ok := node.(*ast.Paragraph); ok {
		// Skip rendering paragraph tags
		skipPara = true
	}

	return ast.GoToNext, skipPara
}

func MarkdownToHTML(md []byte) []byte {
	markdownParser := parser.NewWithExtensions(markdownExtensions)
	htmlRenderer := html.NewRenderer(htmlRenderOptions)
	return markdown.Render(markdownParser.Parse(md), htmlRenderer)
}
