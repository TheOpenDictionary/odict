package main

import (
	"fmt"
	"os"
	"reflect"
	"strings"

	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/go-xmlfmt/xmlfmt"
)

type XSDOptions struct {
	TagName     string
	IsItem      bool
	IsAttribute bool
	isRequired  bool
}

func generateStruct(t reflect.Type, opts XSDOptions) string {
	tagName := opts.TagName
	isItem := opts.IsItem
	isRequired := opts.isRequired
	minMax := ""

	if isItem {
		minOccurs := 0

		if isRequired {
			minOccurs = 1
		}

		minMax = fmt.Sprintf(" minOccurs=\"%d\" maxOccurs=\"unbounded\"", minOccurs)
	} else if isRequired {
		minMax = " use=\"required\""
	}

	var sequences []string = []string{}
	var attributes []string = []string{}
	var children []string = []string{}

	for i := 0; i < t.NumField(); i++ {
		field := t.Field(i)
		fieldType := field.Type
		tag := field.Tag.Get("xml")
		isAttribute := strings.Contains(tag, "attr")
		isCollection := fieldType.Kind() == reflect.Map || fieldType.Kind() == reflect.Slice
		isItem := false
		isRequired := !strings.Contains(tag, "omitempty")
		name := strings.Split(tag, ",")[0]

		if fieldType.Name() == "Name" {
			tagName = name
			continue
		}

		if isCollection {
			fieldType = fieldType.Elem()
			isItem = true
		}

		value := generateXSD(fieldType, XSDOptions{
			TagName:     name,
			IsItem:      isItem,
			IsAttribute: isAttribute,
			isRequired:  isRequired,
		})

		if isCollection {
			sequences = append(sequences, value)
			continue
		}

		if isAttribute {
			attributes = append(attributes, value)
			continue
		}

		children = append(children, value)
	}

	xsd := fmt.Sprintf("<xs:element%s name=\"%s\">\n", minMax, tagName)
	xsd += "<xs:complexType>\n"

	if len(sequences) > 0 {
		xsd += "<xs:sequence>\n"
		xsd += strings.Join(sequences, "")
		xsd += "</xs:sequence>\n"
	}

	if len(children) > 0 {
		xsd += strings.Join(children, "")
	}

	if len(attributes) > 0 {
		xsd += strings.Join(attributes, "")
	}

	xsd += "</xs:complexType>\n"
	xsd += "</xs:element>\n"

	return xsd
}

func generateXSD(t reflect.Type, opts XSDOptions) string {
	tagName := opts.TagName
	isAttribute := opts.IsAttribute
	isItem := opts.IsItem
	isRequired := opts.isRequired
	name := t.Name()
	minOccurs := 0

	if name == "Name" {
		return ""
	}

	if name == "PartOfSpeech" || name == "MDString" {
		return generateXSD(reflect.TypeOf(""), opts)
	}

	if t.Kind() == reflect.Struct {
		return generateStruct(t, opts)
	}

	tag := "element"
	minMax := ""

	if isAttribute {
		tag = "attribute"
	}

	if isRequired {
		minOccurs = 1
	}

	if isItem {
		minMax = fmt.Sprintf(" minOccurs=\"%d\" maxOccurs=\"unbounded\"", minOccurs)
	} else if isRequired {
		minMax = " use=\"required\""
	}

	return fmt.Sprintf("<xs:%s%s name=\"%s\" type=\"xs:%s\" />\n", tag, minMax, tagName, name)
}

func generateSchema() string {
	xsd := "<xs:schema attributeFormDefault=\"unqualified\" elementFormDefault=\"qualified\" xmlns:xs=\"http://www.w3.org/2001/XMLSchema\">\n"
	xsd += generateXSD(reflect.TypeOf(types.DictionaryRepresentable{}), XSDOptions{TagName: "dictionary"})
	xsd += "</xs:schema>"
	return xsd
}

func main() {
	xsd := xmlfmt.FormatXML(generateSchema(), "\t", "  ")
	os.WriteFile("odict.xsd", []byte(xsd), 0644)
}
