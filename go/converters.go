package odict

import (
	"bytes"
	"embed"
	"encoding/json"
	"encoding/xml"
	"fmt"

	"github.com/bokwoon95/sq"
	"github.com/bokwoon95/sqddl/ddl"
	flatbuffers "github.com/google/flatbuffers/go"
)

//go:embed sql.go
var fsys embed.FS

func strToPartOfSpeech(str string) PartOfSpeech {
	if val, ok := posTagPartOfSpeechMap[str]; ok {
		return val
	}

	return Unknown
}

func serialize(b Serializable) []byte {
	builder := flatbuffers.NewBuilder(0)
	buffer := b.AsBuffer(builder)

	builder.Finish(buffer)

	return builder.FinishedBytes()
}

func JSON(any interface{}) string {
	b, err := json.MarshalIndent(&any, "", " ")

	Check(err)

	return string(b)
}

func XML(any interface{}) string {
	str, err := xml.MarshalIndent(&any, "", " ")

	Check(err)

	return string(str)
}

func SQL(dict DictionaryRepresentable, sqlDialect string) string {
	var buf bytes.Buffer
	var sqlCmds string
	dictId := 1
	entryId := 1
	etyId := 1

	generateCmd := &ddl.GenerateCmd{
		Dialect:   sqlDialect,
		DirFS:     fsys,
		Filenames: []string{"sql.go"},
		DryRun:    true,
		Stdout:    &buf,
	}
	err := generateCmd.Run()
	if err != nil {
		fmt.Println(err)
	}

	sqlCmds += buf.String()

	// Insert Dictionary with PK of 1
	d := sq.New[DICTIONARY]("")
	insertQuery := sq.
		InsertInto(d).
		Columns(d.NAME, d.ID).
		Values(sq.Literal(dict.Name), sq.Literal(dictId))
	dict_query, _, err := sq.ToSQL(sq.DialectPostgres, insertQuery, nil)
	if err != nil {
		fmt.Println(err)
	}
	sqlCmds += dict_query + ";\n"

	// Insert entries with relation to dictionary with PK of 1
	for _, entry := range dict.Entries {
		e := sq.New[ENTRY]("")
		insertQuery := sq.
			InsertInto(e).
			Columns(e.ID, e.TERM, e.DICTIONARY_ID).
			Values(sq.Literal(entryId), sq.Literal(entry.Term), sq.Literal(1))
		e_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)
		if err != nil {
			fmt.Println(err)
			continue
		}
		sqlCmds += e_query + ";\n"
		// Insert ety with relation to current entry
		for _, etymology := range entry.Etymologies {
			ety := sq.New[ETYMOLOGY]("")
			insertQuery := sq.
				InsertInto(ety).
				Columns(ety.ID, ety.DESCRIPTION, ety.ENTRY_ID).
				Values(sq.Literal(etyId), sq.Literal(etymology.Description), sq.Literal(entryId))
			ety_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)
			if err != nil {
				fmt.Println(err)
				continue
			}
			sqlCmds += ety_query + ";\n"
			etyId++
		}
		entryId++
	}

	return sqlCmds
}
