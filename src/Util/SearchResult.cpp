#include "SearchResult.h"

odict::SearchResult::SearchResult() : results(vector<Entry*>()) { }

void odict::SearchResult::add_result(Entry* entry) {
    this->results.push_back(entry);
}

vector<Entry*> odict::SearchResult::get_results() {
    return this->results;
}

string odict::SearchResult::get_query() {
    return this->query;
}

void odict::SearchResult::set_query(string query) {
    this->query = query;
}
