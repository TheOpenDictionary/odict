#ifndef ODICT_DICTIONARYSEARCH_H
#define ODICT_DICTIONARYSEARCH_H

#include <iostream>
#include <string>

#include "../schema_generated.h"

#include "../Converters/JSONConverter.h"
#include "../Converters/ConverterResolver.h"
#include "../Util/Constants.h"
#include "../Util/CacheLocationManager.h"
#include "../Util/SearchResult.h"

#include "DictionaryReader.h"

using namespace std;
using namespace schema;
using namespace flatbuffers;
using namespace odc;

class DictionarySearch {
private:
    const Dictionary* dict;
    const char *format;
    FlatBufferBuilder builder;

public:
    DictionarySearch(const char *, const char *);
    DictionarySearch(const uint8_t *, const char *);
    DictionarySearch(const uint8_t *);
    DictionarySearch(const char *);

    const char * search_by_entry(const char *);
    const char * search_by_contents(const char *);
};


#endif //ODICT_DICTIONARYSEARCH_H
