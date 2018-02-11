#ifndef ODICT_ICONVERTER_H
#define ODICT_ICONVERTER_H

#include <string>

#include "../schema_generated.h"

#include "../Util/SearchResult.h"

using namespace schema;
using namespace std;

class IConverter {
public:
    virtual const char *convert(Entry *entry) = 0;
    virtual const char *convert(odict::SearchResult *searchResult) = 0;
};

#endif //ODICT_ICONVERTER_H
