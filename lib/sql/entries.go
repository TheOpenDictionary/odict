package sql

import (
	"sort"

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
func insertEntries(builder *SQLBuilder, dictionaryID string, entryMap types.KVMap[string, types.Entry]) {
	entries := entryMap.Values()

	sort.Slice(entries, func(i, j int) bool {
		return entries[i].Term < entries[j].Term
	})

	for _, entry := range entries {
		e := sq.New[ENTRIES]("")
		id := utils.CreateUUID()
		term := entry.Term

		builder.AddCommand(
			sq.
				InsertInto(e).
				Columns(e.ID, e.TERM, e.DICTIONARY_ID).
				Values(sq.Literal(id), sq.Literal(term), sq.Literal(dictionaryID)),
		)

		insertEtymologies(builder, id, entry.Etymologies)
	}
}
