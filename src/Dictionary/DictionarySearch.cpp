#include "DictionarySearch.h"

DictionarySearch::DictionarySearch(const uint8_t *buffer, const char *format2) : dict(GetDictionary(buffer)),
                                                                                 format(format2) {
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

    if (converter == nullptr) {
        printf("\nError: could not find resolver for format '%s'\n", this->format);
        exit(0);
    } else return converter->convert(result);
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
    cfish_String *content_str = cfish_Str_newf("content");
    lucy_HitDoc *hit;
    int i = 1;

    // Loop over search results.
    while (NULL != (hit = LUCY_Hits_Next(hits))) {
        cfish_String *title = (cfish_String *) LUCY_HitDoc_Extract(hit, title_str);
        char *title_c = CFISH_Str_To_Utf8(title);

        cfish_String *tokens = (cfish_String *) LUCY_HitDoc_Extract(hit, tokens_str);
        char *tokens_c = CFISH_Str_To_Utf8(tokens);

        cfish_Blob *content = (cfish_Blob *) LUCY_HitDoc_Extract(hit, content_str);
        uint8_t *content_b = (uint8_t *) CFISH_Blob_Get_Buf(content);
        printf("Result %d: %s %d (%s)\n", i, title_c, sizeof(content_b), tokens_c);

        free(tokens_c);
        free(title_c);
        CFISH_DECREF(tokens);
        CFISH_DECREF(title);
        CFISH_DECREF(hit);
        i++;
    }

    CFISH_DECREF(tokens_str);
    CFISH_DECREF(content_str);
    CFISH_DECREF(title_str);
    CFISH_DECREF(hits);
    CFISH_DECREF(query_str);
    CFISH_DECREF(searcher);
    CFISH_DECREF(folder);

    return "";
}