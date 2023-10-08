package dump

import (
	"bytes"
	"embed"
	"fmt"
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/bokwoon95/sq"
	"github.com/bokwoon95/sqddl/ddl"
	"github.com/huandu/go-sqlbuilder"
	"reflect"
)

//go:embed sql.go
var fsys embed.FS

var sqlDictionaryId int = 1
var sqlEntryId int = 1
var sqlEtymologyId int = 1
var sqlSenseId int = 1
var sqlNoteId int = 1
var sqlGroupId int = 1
var sqlDefinitionId int = 1
var sqlExampleId int = 1

func printSchema(obj interface{}) {
	printSchemaRecursive(reflect.ValueOf(obj), 0)
}

func printSchemaRecursive(val reflect.Value, depth int) {
	if val.Kind() != reflect.Struct {
		return
	}

	typ := val.Type()
	indent := ""
	for i := 0; i < depth; i++ {
		indent += "  "
	}

	for i := 0; i < val.NumField(); i++ {
		field := val.Field(i)
		fieldType := typ.Field(i)

		print(fieldType.Tag.Get("db"))
		fmt.Printf("%sField Name: %s\n", indent, fieldType.Name)
		fmt.Printf("%sField Type: %s\n", indent, fieldType.Type)

		if field.Kind() == reflect.Map {

			fmt.Printf("%sSub-Struct:\n", indent)
			printSchemaRecursive(field, depth+1)
		}
	}
}

func sql(dict types.DictionaryRepresentable, sqlDialect SqlDialect) (string, error) {
	printSchema(dict)
	var sqlCmds string
	// v := sqlbuilder.InsertBuilder()
	s := sqlbuilder.NewStruct(new(types.DictionaryRepresentable))
	v := s.Flavor.NewCreateTableBuilder().CreateTable("dictionary")
	s4, _ := v.Build()
	print(s4)
	db := s.InsertInto("dictionaries", dict)

	sql, args := db.BuildWithFlavor(sqlbuilder.PostgreSQL)

	s2, _ := sqlbuilder.PostgreSQL.Interpolate(sql, args)
	print(s2)
	create, createErr := sqlCreate(sqlDialect)
	insert, insertErr := sqlInsert(sqlDialect, dict)

	if createErr != nil {
		return "", createErr
	} else {
		sqlCmds += create
	}

	if insertErr != nil {
		return "", insertErr
	}

	sqlCmds += insert

	return sqlCmds, nil
}

func sqlCreate(sqlDialect SqlDialect) (string, error) {
	var buf bytes.Buffer

	// Generate SQL create statements and constraints
	generateCmd := &ddl.GenerateCmd{
		Dialect:   sqlDialect,
		DirFS:     fsys,
		Filenames: []string{"sql.go"},
		DryRun:    true,
		Stdout:    &buf,
	}

	err := generateCmd.Run()

	if err != nil {
		return "", err
	}

	return buf.String(), nil
}

func sqlInsert(sqlDialect SqlDialect, dict types.DictionaryRepresentable) (string, error) {
	var sqlCmds string

	// Entry point that will capture every insert statement
	// Dictionary, entries, etymologies, senses, groups, definitions, examples
	insert, err := sqlInsertDictionary(sqlDialect, dict)

	if err != nil {
		return "", err
	}

	sqlCmds += insert

	return sqlCmds, nil
}

func sqlInsertDictionary(sqlDialect SqlDialect, dict types.DictionaryRepresentable) (string, error) {
	var sqlCmds string

	// Insert dictionary w/ PK of 1
	d := sq.New[DICTIONARIES]("")

	insertQuery := sq.
		InsertInto(d).
		Columns(d.NAME, d.ID).
		Values(sq.Literal(dict.Name), sq.Literal(sqlDictionaryId))

	dict_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)

	if err != nil {
		return "", err
	}

	sqlCmds += dict_query + ";\n"

	insertEntries, insertErr := sqlInsertEntries(sqlDialect, dict.Entries)

	if insertErr != nil {
		return "", insertErr
	}

	sqlCmds += insertEntries

	return sqlCmds, nil
}

func sqlInsertEntries(sqlDialect SqlDialect, entries types.KVMap[string, types.EntryRepresentable]) (string, error) {
	var sqlCmds string

	// Insert entries w/ relation to dictionary with PK of 1
	for _, entry := range entries {
		e := sq.New[ENTRIES]("")

		insertQuery := sq.
			InsertInto(e).
			Columns(e.ID, e.TERM, e.DICTIONARY_ID).
			Values(sq.Literal(sqlEntryId), sq.Literal(entry.Term), sq.Literal(1))

		e_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)

		if err != nil {
			return "", err
		}

		sqlCmds += e_query + ";\n"

		insertEty, err := sqlInsertEtymologies(sqlDialect, entry.Etymologies)

		if err != nil {
			return "", err
		}

		sqlCmds += insertEty
		sqlEntryId++
	}

	return sqlCmds, nil
}

func sqlInsertEtymologies(sqlDialect SqlDialect, etymologies []types.EtymologyRepresentable) (string, error) {
	var sqlCmds string

	// Insert etymologies w/ relation to current entry
	for _, etymology := range etymologies {
		ety := sq.New[ETYMOLOGIES]("")

		insertQuery := sq.
			InsertInto(ety).
			Columns(ety.ID, ety.DESCRIPTION, ety.ENTRY_ID).
			Values(sq.Literal(sqlEtymologyId), sq.Literal(etymology.Description.String()), sq.Literal(sqlEntryId))

		ety_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)

		if err != nil {
			return "", err
		}

		sqlCmds += ety_query + ";\n"

		insertSenses, insertErr := sqlInsertSenses(sqlDialect, etymology.Senses)

		if insertErr != nil {
			return "", insertErr
		}

		sqlCmds += insertSenses
		sqlEtymologyId++
	}

	return sqlCmds, nil
}

func sqlInsertSenses(sqlDialect SqlDialect, senses types.KVMap[types.PartOfSpeech, types.SenseRepresentable]) (string, error) {
	var sqlCmds string

	// Insert senses w/ relation to current ety
	for _, sense := range senses {
		usg := sq.New[SENSES]("")

		insertQuery := sq.
			InsertInto(usg).
			Columns(usg.ID, usg.ETYMOLOGY_ID).
			Values(sq.Literal(sqlSenseId), sq.Literal(sqlEtymologyId))

		usg_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)

		if err != nil {
			return "", err
		}

		sqlCmds += usg_query + ";\n"

		insertGroups, insertErr := sqlInsertGroups(sqlDialect, sense.Groups)

		if insertErr != nil {
			return "", insertErr
		}

		sqlCmds += insertGroups
		sqlSenseId++
	}

	return sqlCmds, nil
}

func sqlInsertGroups(sqlDialect SqlDialect, groups []types.GroupRepresentable) (string, error) {
	var sqlCmds string

	// Insert groups w/ relation to current sense
	for _, group := range groups {
		grp := sq.New[GROUPS]("")

		insertQuery := sq.
			InsertInto(grp).
			Columns(grp.ID, grp.DESCRIPTION, grp.SENSE_ID).
			Values(sq.Literal(sqlGroupId), sq.Literal(group.Description.String()), sq.Literal(sqlSenseId))

		grp_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)

		if err != nil {
			return "", err
		}

		insertDefinitions, insertErr := sqlInsertDefinitions(sqlDialect, group.Definitions)

		if insertErr != nil {
			return "", insertErr
		}

		sqlCmds += grp_query + ";\n"
		sqlCmds += insertDefinitions

		sqlGroupId++
	}

	return sqlCmds, nil
}

func sqlInsertDefinitions(sqlDialect SqlDialect, definitions []types.DefinitionRepresentable) (string, error) {
	var sqlCmds string

	// Insert definitions w/ relation to current sense/group
	for _, definition := range definitions {
		def := sq.New[DEFINITIONS]("")

		insertQuery := sq.
			InsertInto(def).
			Columns(def.ID, def.TEXT, def.SENSE_ID, def.GROUP_ID).
			Values(sq.Literal(sqlDefinitionId), sq.Literal(definition.Value.String()), sq.Literal(sqlSenseId), sq.Literal(sqlGroupId))

		def_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)

		if err != nil {
			return "", err
		}

		insertExamples, insertErr := sqlInsertDefinitionExamples(sqlDialect, definition.Examples)

		if insertErr != nil {
			return "", insertErr
		}

		sqlCmds += def_query + ";\n"
		sqlCmds += insertExamples

		sqlDefinitionId++
	}

	return sqlCmds, nil
}

func sqlInsertNotes(sqlDialect SqlDialect, notes []types.NoteRepresentable) (string, error) {
	var sqlCmds string

	// Insert definitions w/ relation to current sense/group
	for _, note := range notes {
		def := sq.New[NOTES]("")

		insertQuery := sq.
			InsertInto(def).
			Columns(def.ID, def.TEXT, def.DEFINITION_ID).
			Values(sq.Literal(sqlNoteId), sq.Literal(note.Value.String()), sq.Literal(sqlDefinitionId))

		note_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)

		if err != nil {
			return "", err
		}

		insertExamples, insertErr := sqlInsertDefinitionExamples(sqlDialect, note.Examples)

		if insertErr != nil {
			return "", insertErr
		}

		sqlCmds += note_query + ";\n"
		sqlCmds += insertExamples

		sqlNoteId++
	}

	return sqlCmds, nil
}

func sqlInsertDefinitionExamples(sqlDialect SqlDialect, examples []string) (string, error) {
	var sqlCmds string

	// Insert examples w/ relation to current definition
	for _, example := range examples {
		ex := sq.New[DEFINITION_EXAMPLES]("")

		insertQuery := sq.
			InsertInto(ex).
			Columns(ex.ID, ex.TEXT, ex.DEFINITION_ID).
			Values(sq.Literal(sqlExampleId), sq.Literal(example), sq.Literal(sqlDefinitionId))

		ex_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)

		if err != nil {
			return "", err
		}

		sqlCmds += ex_query + ";\n"

		sqlExampleId++
	}

	return sqlCmds, nil
}
