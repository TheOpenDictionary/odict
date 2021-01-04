package models

type Etymology struct {
	ID          string   `json:"id" xml:"id,attr"`
	Description string   `json:"description" xml:"description,attr,omitempty"`
	Usages      UsageMap `json:"usages" xml:"usage"`
}
