package server

import (
	"fmt"
	"net/http"
)

// Spin up a local web server
func StartServer(pathOrAlias string, port int) error {
	handleLookup(pathOrAlias)
	handleSearch(pathOrAlias)

	fmt.Printf("Listening on port %d\n", port)

	http.ListenAndServe(fmt.Sprintf(":%d", port), nil)

	return nil
}
