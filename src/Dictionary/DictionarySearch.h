#ifndef ODICT_DICTIONARYSEARCH_H
#define ODICT_DICTIONARYSEARCH_H

#import <iostream>
#import <string>

#include "schema_generated.h"

#include "JSONConverter.h"
#include "Constants.h"
#include "ConverterResolver.h"
#include "DictionaryReader.h"
#include "CacheLocationManager.h"
#include "SearchResult.h"

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

    const char * searchByEntry(const char *);
    const char * searchByContents(const char *);
};


#endif //ODICT_DICTIONARYSEARCH_H
