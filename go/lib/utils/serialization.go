package utils

import (
	"encoding/json"
	"encoding/xml"
)

// Serializes any interface to JSON if possible
func SerializeToJSON(any interface{}, prettyPrint bool) (string, error) {
	if prettyPrint {
		b, err := json.MarshalIndent(&any, "", " ")

		if err != nil {
			return "", err
		}

		return string(b), nil
	} else {
		b, err := json.Marshal(&any)

		if err != nil {
			return "", err
		}

		return string(b), nil
	}
}

// Serializes any interface to XML if possible
func SerializeToXML(any interface{}, prettyPrint bool) (string, error) {
	if prettyPrint {
		str, err := xml.MarshalIndent(&any, "", " ")

		if err != nil {
			return "", err
		}

		return string(str), nil
	} else {
		str, err := xml.Marshal(&any)

		if err != nil {
			return "", err
		}

		return string(str), err
	}
}
