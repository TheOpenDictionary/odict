#include "DictionarySearch.h"

DictionarySearch::DictionarySearch(const uint8_t *buffer, const char *format) : dict(GetDictionary(buffer)),
                                                                                 format(format) {
    if (buffer == 0) {
        cout << "Data is corrupted. Cannot read dictionary.\n" << endl;
        exit(0);
    }
}

DictionarySearch::DictionarySearch(const char *filename) : DictionarySearch(filename, FORMAT_JSON) {}

DictionarySearch::DictionarySearch(const uint8_t *buffer) : DictionarySearch(buffer, FORMAT_JSON) {}

DictionarySearch::DictionarySearch(const char *filename, const char *format) : DictionarySearch(
        (new DictionaryReader())->ReadAsBuffer(filename),
        format
) {}

const char *DictionarySearch::searchByEntry(const char *word) {
    auto entries = this->dict->entries();
    auto result = entries->LookupByKey(word);
    auto converter = ConverterResolver::resolve(this->format);

    if (converter == NULL) {
        printf("\nError: could not find resolver for format '%s'\n", this->format);
        exit(0);
    } else return converter->convert((Entry*)result);
}

const char *DictionarySearch::searchByContents(const char *str) {
    lucy_bootstrap_parcel();

    const char *id = this->dict->id()->c_str();
    const char *base = CacheLocationManager::getInstance()->getLocation();
    cfish_String *folder = cfish_Str_newf("%s/%s", base, id);
    lucy_IndexSearcher *searcher = lucy_IxSearcher_new((cfish_Obj *) folder);

    cfish_String *query_str = cfish_Str_newf("%s", str);
    lucy_Hits *hits = LUCY_IxSearcher_Hits(searcher, (cfish_Obj *) query_str, 0, 10, NULL);

    cfish_String *title_str = cfish_Str_newf("title");
    cfish_String *tokens_str = cfish_Str_newf("tokens");
    lucy_HitDoc *hit;
    int i = 1;

    vector<Offset<Entry>> results = vector<Offset<Entry>>();
    odict::SearchResult *sr = new odict::SearchResult();

    sr->setQuery(str);

    // Loop over search results.
    while (NULL != (hit = LUCY_Hits_Next(hits))) {
        cfish_String *title = (cfish_String *) LUCY_HitDoc_Extract(hit, title_str);
        char *title_c = CFISH_Str_To_Utf8(title);

        cfish_String *tokens = (cfish_String *) LUCY_HitDoc_Extract(hit, tokens_str);
        char *tokens_c = CFISH_Str_To_Utf8(tokens);

        auto entry = this->dict->entries()->LookupByKey(title_c);

        sr->addResult((Entry*)entry);

        free(tokens_c);
        free(title_c);
        CFISH_DECREF(tokens);
        CFISH_DECREF(title);
        CFISH_DECREF(hit);
        i++;
    }

    CFISH_DECREF(tokens_str);
    CFISH_DECREF(title_str);
    CFISH_DECREF(hits);
    CFISH_DECREF(query_str);
    CFISH_DECREF(searcher);
    CFISH_DECREF(folder);

    return ConverterResolver::resolve(this->format)->convert(sr);
}