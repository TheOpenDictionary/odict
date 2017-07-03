#ifndef ODICT_JSONCONVERTER_H
#define ODICT_JSONCONVERTER_H

#include <string>
#include <vector>
#include <string>
#include "schema_generated.h"

using namespace std;
using namespace schema;
using namespace flatbuffers;

class EntryJSONConverter {
public:
    EntryJSONConverter();
    string add_definitions(const Vector<Offset<String>> *definitions);
    string add_usages(const Vector<Offset<Usage>> *usages);
    string convert(const Entry *entry);
};

#endif //ODICT_JSONCONVERTER_H
