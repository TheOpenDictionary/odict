#ifndef ODICT_JSONCONVERTER_H
#define ODICT_JSONCONVERTER_H

#include <string>
#include <vector>
#include <string>
#include <iostream>

#include "schema_generated.h"

#include "IConverter.h"

using namespace std;
using namespace schema;
using namespace flatbuffers;

class JSONConverter : public IConverter {
private:
    string add_definitions(const Vector<Offset<String>> *definitions);
    string add_groups(const Vector<Offset<Group>> *groups);
    string add_usages(const Vector<Offset<Usage>> *usages);
    string add_etymologies(const Vector<Offset<Etymology>> *etymologies);
public:
    JSONConverter();
    string convert(const Entry *entry);
};

#endif //ODICT_JSONCONVERTER_H
