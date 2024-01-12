package sql

import (
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	"github.com/bokwoon95/sq"
)

type DICTIONARIES struct {
	sq.TableStruct
	ID   sq.UUIDField   `ddl:"primarykey"`
	NAME sq.StringField `ddl:"notnull"`
}

// Insert dictionary
func insertDictionary(builder *SQLBuilder, dict types.DictionaryRepresentable) {
	d := sq.New[DICTIONARIES]("")
	id := utils.CreateUUID()

	builder.AddCommand(
		sq.
			InsertInto(d).
			Columns(d.NAME, d.ID).
			Values(sq.Literal(dict.Name), sq.Literal(id)),
	)

	insertEntries(builder, id, dict.Entries)
}
