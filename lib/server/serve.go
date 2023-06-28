package server

import (
	"fmt"
	"net/http"
)

// Spin up a local web server
func StartServer(pathOrAlias string, port int) error {
	http.HandleFunc("/lookup", handleLookup(pathOrAlias))
	http.HandleFunc("/search", handleSearch(pathOrAlias))

	fmt.Printf("Listening on port %d\n", port)

	http.ListenAndServe(fmt.Sprintf(":%d", port), nil)

	return nil
}
