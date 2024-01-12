package sql

import (
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	"github.com/bokwoon95/sq"
)

type ENTRIES struct {
	sq.TableStruct
	ID            sq.UUIDField `ddl:"primarykey"`
	CREATED_AT    sq.TimeField `ddl:"default=CURRENT_TIMESTAMP"`
	UPDATED_AT    sq.TimeField
	TERM          sq.StringField `ddl:"notnull"`
	DICTIONARY_ID sq.UUIDField   `ddl:"references=dictionaries.id"`
}

// Insert entries w/ relation to dictionary
func insertEntries(builder *SQLBuilder, dictionaryID string, entries types.KVMap[string, types.EntryRepresentable]) {
	for _, entry := range entries {
		e := sq.New[ENTRIES]("")
		id := utils.CreateUUID()

		builder.AddCommand(
			sq.
				InsertInto(e).
				Columns(e.ID, e.TERM, e.DICTIONARY_ID).
				Values(sq.Literal(id), sq.Literal(entry.Term), sq.Literal(dictionaryID)),
		)

		insertEtymologies(builder, id, entry.Etymologies)
	}
}
