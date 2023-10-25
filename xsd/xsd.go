package main

import (
	"fmt"
	"reflect"
	"strings"

	"github.com/TheOpenDictionary/odict/lib/types"
)

func generateXSD(tagName string, attribute bool, t reflect.Type, indent int) string {
	name := t.Name()

	if name == "Name" {
		return ""
	}

	if name == "PartOfSpeech" || name == "MDString" {
		return generateXSD(tagName, attribute, reflect.TypeOf(""), indent)
	}

	if t.Kind() == reflect.Map {
		return generateXSD(tagName, attribute, t.Elem(), indent+2)
	}

	if t.Kind() == reflect.Slice {
		xsd := fmt.Sprintf("%s<xs:sequence>\n", strings.Repeat(" ", indent))
		xsd += generateXSD(tagName, attribute, t.Elem(), indent+2)
		xsd += fmt.Sprintf("%s</xs:sequence>\n", strings.Repeat(" ", indent))
		return xsd
	}

	if t.Kind() == reflect.Struct {
		xsd := fmt.Sprintf("%s<xs:element name=\"%s\">\n", strings.Repeat(" ", indent), tagName)
		xsd += fmt.Sprintf("%s<xs:complexType>\n", strings.Repeat(" ", indent+2))

		for i := 0; i < t.NumField(); i++ {
			field := t.Field(i)
			fieldType := field.Type
			tag := field.Tag.Get("xml")
			tagName := strings.Split(tag, ",")[0]
			attr := strings.Contains(tag, "attr")

			xsd += generateXSD(tagName, attr, fieldType, indent+4)
		}

		xsd += fmt.Sprintf("%s</xs:complexType>\n", strings.Repeat(" ", indent+2))
		xsd += fmt.Sprintf("%s</xs:element>\n", strings.Repeat(" ", indent))

		return xsd
	}

	tag := "element"

	if attribute {
		tag = "attribute"
	}

	return fmt.Sprintf("%s<xs:%s name=\"%s\" type=\"xs:%s\" />\n", strings.Repeat(" ", indent), tag, tagName, name)
}

func main() {
	xsd := generateXSD("dictionary", false, reflect.TypeOf(types.DictionaryRepresentable{}), 0)

	print(xsd)
}
