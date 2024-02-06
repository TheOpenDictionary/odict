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
	parser.SuperSubscript

var htmlFlags = html.CommonFlags | html.HrefTargetBlank

var htmlRenderOptions = html.RendererOptions{Flags: htmlFlags, RenderNodeHook: mdRenderHook}

var plainRenderOptions = html.RendererOptions{RenderNodeHook: plainRenderHook}

func mdRenderHook(w io.Writer, node ast.Node, entering bool) (ast.WalkStatus, bool) {
	skipPara := false

	if _, ok := node.(*ast.Paragraph); ok {
		// Skip rendering paragraph tags
		skipPara = true
	}

	return ast.GoToNext, skipPara
}

func plainRenderHook(w io.Writer, node ast.Node, entering bool) (ast.WalkStatus, bool) {
	if entering {
		switch node.(type) {
		case *ast.Text:
			text := node.(*ast.Text).Literal
			w.Write(text)
		case *ast.Superscript:
			text := node.(*ast.Superscript).Literal
			w.Write(text)
		case *ast.Subscript:
			text := node.(*ast.Subscript).Literal
			w.Write(text)
		}
	}
	return ast.GoToNext, true
}

func MarkdownToHTML(md []byte) []byte {
	markdownParser := parser.NewWithExtensions(markdownExtensions)
	htmlRenderer := html.NewRenderer(htmlRenderOptions)
	return markdown.Render(markdownParser.Parse(md), htmlRenderer)
}

func MarkdownToText(md []byte) []byte {
	markdownParser := parser.NewWithExtensions(markdownExtensions)
	htmlRenderer := html.NewRenderer(plainRenderOptions)
	return markdown.Render(markdownParser.Parse(md), htmlRenderer)
}
