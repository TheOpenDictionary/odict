package models

import (
	"encoding/json"
	"encoding/xml"
	"io"
)

type Usage struct {
	POS         PartOfSpeech `json:"pos" xml:"pos,attr"`
	Definitions []string     `json:"definitions" xml:"definition"`
	Groups      []Group      `json:"groups" xml:"group"`
	XMLName     xml.Name     `json:"-" xml:"usage"`
}

type UsageMap struct {
	Iterable map[PartOfSpeech]Usage
}

func (m *UsageMap) Set(key PartOfSpeech, value Usage) {
	m.Iterable[key] = value
}

func (m *UsageMap) Get(key PartOfSpeech) Usage {
	return m.Iterable[key]
}

func (m UsageMap) MarshalJSON() ([]byte, error) {
	return json.Marshal(m.Iterable)
}

func (m UsageMap) MarshalXML(e *xml.Encoder, start xml.StartElement) error {
	for key := range m.Iterable {
		e.Encode(m.Get(key))
	}
	return nil
}

func (m *UsageMap) UnmarshalXML(d *xml.Decoder, start xml.StartElement) error {
	var usage Usage

	d.DecodeElement(&usage, &start)

	if m.Iterable == nil {
		m.Iterable = make(map[PartOfSpeech]Usage)
	}

	m.Set(usage.POS, usage)

	for {
		_, err := d.Token()

		if err != nil {
			if err == io.EOF {
				return nil
			}
			return err
		}
	}
}
