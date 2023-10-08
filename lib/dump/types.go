package dump

import "github.com/bokwoon95/sq"

type DICTIONARIES struct {
	sq.TableStruct
	ID   sq.NumberField `ddl:"type=BIGINT primarykey auto_increment autoincrement identity"`
	NAME sq.StringField `ddl:"notnull"`
}

type ENTRIES struct {
	sq.TableStruct
	ID            sq.NumberField `ddl:"type=BIGINT primarykey auto_increment autoincrement identity"`
	CREATED_AT    sq.TimeField   `ddl:"default=CURRENT_TIMESTAMP"`
	UPDATED_AT    sq.TimeField
	TERM          sq.StringField `ddl:"notnull"`
	DICTIONARY_ID sq.NumberField `ddl:"type=BIGINT references=dictionaries.id"`
}

type ETYMOLOGIES struct {
	sq.TableStruct
	ID          sq.StringField `ddl:"primarykey"` // cuid() generation is handled by prisma, not the database.
	DESCRIPTION sq.StringField
	ENTRY_ID    sq.NumberField `ddl:"type=BIGINT notnull references=entries.id"`
}

type SENSES struct {
	sq.TableStruct
	ID           sq.NumberField `ddl:"type=BIGINT primarykey auto_increment autoincrement identity"`
	ETYMOLOGY_ID sq.StringField `ddl:"notnull references=etymologies.id"`
}

type GROUPS struct {
	sq.TableStruct
	ID          sq.NumberField `ddl:"type=BIGINT primarykey auto_increment autoincrement identity"`
	DESCRIPTION sq.StringField
	SENSE_ID    sq.NumberField `ddl:"type=BIGINT notnull references=senses.id"`
}

type DEFINITIONS struct {
	sq.TableStruct
	ID       sq.NumberField `ddl:"type=BIGINT primarykey auto_increment autoincrement identity"`
	TEXT     sq.StringField `ddl:"notnull"`
	SENSE_ID sq.NumberField `ddl:"type=BIGINT references=senses.id"`
	GROUP_ID sq.NumberField `ddl:"type=BIGINT references=groups.id"`
}

type NOTES struct {
	sq.TableStruct
	ID            sq.NumberField `ddl:"type=BIGINT primarykey auto_increment autoincrement identity"`
	TEXT          sq.StringField `ddl:"notnull"`
	DEFINITION_ID sq.NumberField `ddl:"type=BIGINT references=definitions.id"`
}

type DEFINITION_EXAMPLES struct {
	sq.TableStruct
	ID            sq.NumberField `ddl:"type=BIGINT primarykey auto_increment autoincrement identity"`
	TEXT          sq.StringField `ddl:"notnull"`
	DEFINITION_ID sq.NumberField `ddl:"type=BIGINT notnull references=definitions.id"`
}

type NOTE_EXAMPLES struct {
	sq.TableStruct
	ID      sq.NumberField `ddl:"type=BIGINT primarykey auto_increment autoincrement identity"`
	TEXT    sq.StringField `ddl:"notnull"`
	NOTE_ID sq.NumberField `ddl:"type=BIGINT notnull references=notes.id"`
}
