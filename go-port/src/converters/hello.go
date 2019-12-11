package converters

import (
	"io/ioutil"
	"odict/schema"
)

func main() {
	buf, _ := ioutil.ReadFile("monster.dat")
	// handle err
	schema.GetRootAsDictionary(buf, 0)
}
