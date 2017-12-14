#include "DictionarySearch.h"

DictionarySearch::DictionarySearch(const uint8_t * buffer, const char * format2) : buf(buffer), format(format2) {
//    if (buffer == 0) {
//        cout << "Data is corrupted. Cannot read dictionary.\n" << endl;
//        exit(0);
//    }
}

DictionarySearch::DictionarySearch(const char *filename, const char *format) {
    DictionaryReader *reader = new DictionaryReader();
    const uint8_t* buf = reader->ReadAsBuffer(filename);
    DictionarySearch(buf, format);
}

DictionarySearch::DictionarySearch(const char * filename) {
    DictionarySearch(filename, FORMAT_JSON);
}

DictionarySearch::DictionarySearch(const uint8_t * buffer) {
    DictionarySearch(buffer, FORMAT_JSON);
}

const char* DictionarySearch::searchByEntry(const char *word) {
    auto buf = this->buf;
    auto entries = GetDictionary(this->buf)->entries();
    auto result = entries->LookupByKey(word);

    return ConverterResolver::resolve(format)->convert(result);
}