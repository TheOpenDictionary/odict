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
	usageId := 1
	groupId := 1
	defId := 1
	exId := 1

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
		fmt.Println(err)
	}
	sqlCmds += buf.String()

	// Insert dictionary w/ PK of 1
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

	// Insert entries w/ relation to dictionary with PK of 1
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
		// Insert etymologies w/ relation to current entry
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
			// Insert usages w/ relation to current ety
			for _, usage := range etymology.Usages {
				usg := sq.New[USAGE]("")
				insertQuery := sq.
					InsertInto(usg).
					Columns(usg.ID, usg.ETYMOLOGY_ID).
					Values(sq.Literal(usageId), sq.Literal(etyId))
				usg_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)
				if err != nil {
					fmt.Println(err)
					continue
				}
				sqlCmds += usg_query + ";\n"
				// Insert groups w/ relation to current usage
				for _, group := range usage.Groups {
					grp := sq.New[GROUP]("")
					insertQuery := sq.
						InsertInto(grp).
						Columns(grp.ID, grp.DESCRIPTION, grp.USAGE_ID).
						Values(sq.Literal(groupId), sq.Literal(group.Description), sq.Literal(usageId))
					grp_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)
					if err != nil {
						fmt.Println(err)
						continue
					}
					sqlCmds += grp_query + ";\n"
					// Insert definitions w/ relation to current usage/group
					for _, definition := range group.Definitions {
						def := sq.New[DEFINITION]("")
						insertQuery := sq.
							InsertInto(def).
							Columns(def.ID, def.TEXT, def.USAGE_ID, def.GROUP_ID).
							Values(sq.Literal(defId), sq.Literal(definition.Value), sq.Literal(usageId), sq.Literal(groupId))
						def_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)
						if err != nil {
							fmt.Println(err)
							continue
						}
						sqlCmds += def_query + ";\n"
						// Insert examples w/ relation to current definition
						for _, example := range definition.Examples {
							ex := sq.New[EXAMPLE]("")
							insertQuery := sq.
								InsertInto(ex).
								Columns(ex.ID, ex.TEXT, ex.DEFINITION_ID).
								Values(sq.Literal(exId), sq.Literal(example), sq.Literal(defId))
							ex_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)
							if err != nil {
								fmt.Println(err)
								continue
							}
							sqlCmds += ex_query + ";\n"
							exId++
						}
						defId++
					}
					groupId++
				}
				usageId++
			}
			etyId++
		}
		entryId++
	}

	return sqlCmds
}
