#ifndef ODICT_SEARCHRESULT_H
#define ODICT_SEARCHRESULT_H

#include <string>

#include "../schema_generated.h"

using namespace std;
using namespace flatbuffers;
using namespace schema;

namespace odict {
    class SearchResult {
    private:
        string query;
        vector<Entry*> results;
    public:
        SearchResult();
        void add_result(Entry*);
        vector<Entry*> get_results();
        string get_query();
        void set_query(string query);
    };

}

#endif //ODICT_SEARCHRESULT_H
