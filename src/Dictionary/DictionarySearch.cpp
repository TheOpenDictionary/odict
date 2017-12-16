#include "DictionarySearch.h"

DictionarySearch::DictionarySearch(const uint8_t *buffer, const char *format2) : buf(buffer), format(format2) {
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
    auto entries = GetDictionary(this->buf)->entries();
    auto result = entries->LookupByKey(word);
    auto converter = ConverterResolver::resolve(this->format);

    if (converter == nullptr) {
        printf("\nError: could not find resolver for format '%s'\n", this->format);
        exit(0);
    }
    else return converter->convert(result);
}