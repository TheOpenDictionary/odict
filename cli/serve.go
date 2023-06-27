package cli

import (
	"fmt"
	"html"
	"net/http"
)

// Spin up a local web server
func serve(c *cli.Context) error {
	pathOrAlias := c.Args().First()
	dict := core.ReadDictionary(pathOrAlias)
	
	http.HandleFunc("/lookup", func(w http.ResponseWriter, r *http.Request) {
		dictionary := r.URL.Query().Get("dictionary")

		if dictionary == "" {
			http.Error(w, "Missing 'dictionary' parameter", http.StatusBadRequest)
			return
		}
	
		// Assume you have a lookup function that retrieves the dictionary
		// based on the provided parameter
		result := lookup(dictionary)
	
		// Return the result as JSON
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(result)
	}

	http.ListenAndServe(":8080", nil)
}
