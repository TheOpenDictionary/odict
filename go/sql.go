package odict

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

type USAGES struct {
	sq.TableStruct
	ID           sq.NumberField `ddl:"type=BIGINT primarykey auto_increment autoincrement identity"`
	ETYMOLOGY_ID sq.StringField `ddl:"notnull references=etymologies.id"`
}

type GROUPS struct {
	sq.TableStruct
	ID          sq.NumberField `ddl:"type=BIGINT primarykey auto_increment autoincrement identity"`
	DESCRIPTION sq.StringField
	USAGE_ID    sq.NumberField `ddl:"type=BIGINT notnull references=usages.id"`
}

type DEFINITIONS struct {
	sq.TableStruct
	ID       sq.NumberField `ddl:"type=BIGINT primarykey auto_increment autoincrement identity"`
	TEXT     sq.StringField `ddl:"notnull"`
	USAGE_ID sq.NumberField `ddl:"type=BIGINT references=usages.id"`
	GROUP_ID sq.NumberField `ddl:"type=BIGINT references=groups.id"`
}

type EXAMPLES struct {
	sq.TableStruct
	ID            sq.NumberField `ddl:"type=BIGINT primarykey auto_increment autoincrement identity"`
	TEXT          sq.StringField `ddl:"notnull"`
	DEFINITION_ID sq.NumberField `ddl:"type=BIGINT notnull references=definitions.id"`
}
