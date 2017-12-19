#include "SearchResult.h"

odict::SearchResult::SearchResult() : results(vector<Entry*>()) { }

void odict::SearchResult::addResult(Entry* entry) {
    this->results.push_back(entry);
}

vector<Entry*> odict::SearchResult::getResults() {
    return this->results;
}

string odict::SearchResult::getQuery() {
    return this->query;
}

void odict::SearchResult::setQuery(string query) {
    this->query = query;
}
