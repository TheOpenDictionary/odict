package sql

import (
	"github.com/TheOpenDictionary/odict/lib/utils"
	"github.com/bokwoon95/sq"
)

type EXAMPLES struct {
	sq.TableStruct
	ID            sq.StringField `ddl:"primarykey"`
	TEXT          sq.StringField `ddl:"notnull"`
	DEFINITION_ID sq.UUIDField   `ddl:"references=definitions.id"`
	NOTE_ID       sq.UUIDField   `ddl:"references=notes.id"`
}

// Insert examples w/ relation to current definition
func insertExamples(builder *SQLBuilder, definitionID string, noteID string, examples []string) {
	for _, example := range examples {
		ex := sq.New[EXAMPLES]("")
		id := utils.CreateUUID()

		builder.AddCommand(
			sq.
				InsertInto(ex).
				ColumnValues(func(col *sq.Column) {
					col.Set(ex.ID, sq.Literal(id))
					col.Set(ex.TEXT, sq.Literal(example))

					if len(definitionID) > 0 {
						col.Set(ex.DEFINITION_ID, sq.Literal(definitionID))
					}

					if len(noteID) > 0 {
						col.Set(ex.NOTE_ID, sq.Literal(noteID))
					}
				}),
		)
	}
}
