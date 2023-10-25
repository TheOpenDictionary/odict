package main

import (
	"fmt"
	"reflect"
	"strings"

	"github.com/TheOpenDictionary/odict/lib/types"
)

func generateXSD(indent int, t reflect.Type) string {
	xsd := fmt.Sprintf("%s<xs:complexType>\n", strings.Repeat(" ", indent))

	for i := 0; i < t.NumField(); i++ {
		field := t.Field(i)
		fieldType := field.Type
		fieldName := fieldType.Name()
		tag := field.Tag.Get("xml")
		tagName := strings.Split(tag, ",")[0]

		if fieldName == "Name" {
			continue
		}

		if fieldType.Kind() == reflect.Map {
			xsd += generateXSD(indent+2, fieldType.Elem())
		} else if fieldType.Kind() == reflect.Slice {
			xsd += fmt.Sprintf("%s<xs:sequence>\n", strings.Repeat(" ", indent))
			println(fieldType.Elem().Name())
			xsd += generateXSD(indent+4, fieldType.Elem())
			xsd += fmt.Sprintf("%s</xs:sequence>\n", strings.Repeat(" ", indent))
		} else {
			xsd += fmt.Sprintf("%s<xs:element name=\"%s\" type=\"xs:%s\" />\n", strings.Repeat(" ", indent+2), tagName, fieldName)
		}
	}

	xsd += fmt.Sprintf("%s</xs:complexType>\n", strings.Repeat(" ", indent))

	return xsd
}

func main() {
	xsd := generateXSD(0, reflect.TypeOf(types.DictionaryRepresentable{}))

	print(xsd)
}
