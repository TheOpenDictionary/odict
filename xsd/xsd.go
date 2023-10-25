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

	if t.Kind() == reflect.Struct {
		xsd := fmt.Sprintf("%s<xs:element name=\"%s\">\n", strings.Repeat(" ", indent), tagName)
		xsd += fmt.Sprintf("%s<xs:complexType>\n", strings.Repeat(" ", indent+2))

		var sequences []string = []string{}
		var attributes []string = []string{}

		for i := 0; i < t.NumField(); i++ {
			field := t.Field(i)
			fieldType := field.Type
			tag := field.Tag.Get("xml")
			tagName := strings.Split(tag, ",")[0]
			attr := strings.Contains(tag, "attr")

			if fieldType.Kind() == reflect.Map || fieldType.Kind() == reflect.Slice {
				sequences = append(sequences, generateXSD(tagName, attr, fieldType.Elem(), indent+6))
			} else if attr {
				attributes = append(attributes, generateXSD(tagName, attr, fieldType, indent+4))
			} else {
				xsd += generateXSD(tagName, attr, fieldType, indent+4)
			}
		}

		if len(sequences) > 0 {
			xsd += fmt.Sprintf("%s<xs:sequence>\n", strings.Repeat(" ", indent+4))
			xsd += strings.Join(sequences, "")
			xsd += fmt.Sprintf("%s</xs:sequence>\n", strings.Repeat(" ", indent+4))
		}

		if len(attributes) > 0 {
			xsd += strings.Join(attributes, "")
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
	xsd := "<xs:schema attributeFormDefault=\"unqualified\" elementFormDefault=\"qualified\" xmlns:xs=\"http://www.w3.org/2001/XMLSchema\">\n"
	xsd += generateXSD("dictionary", false, reflect.TypeOf(types.DictionaryRepresentable{}), 2)
	xsd += "</xs:schema>"

	print(xsd)
}
