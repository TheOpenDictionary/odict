#ifndef ODICT_JSONCONVERTER_H
#define ODICT_JSONCONVERTER_H

#include <string>
#include <vector>
#include <string>
#include <iostream>

#include <boost/property_tree/ptree.hpp>
#include <boost/property_tree/json_parser.hpp>

#include "../schema_generated.h"
#include "../Util/SearchResult.h"

#include "IConverter.h"

using namespace std;
using namespace schema;
using namespace flatbuffers;

namespace pt = boost::property_tree;

class JSONConverter : public IConverter {
private:
    void add_definitions(pt::ptree *, const Vector<Offset<String>> *definitions);
    void add_groups(pt::ptree *, const Vector<Offset<Group>> *groups);
    void add_usages(pt::ptree *, const Vector<Offset<Usage>> *usages);
    void add_etymologies(pt::ptree *, const Vector<Offset<Etymology>> *etymologies);
public:
    JSONConverter();
    const char* convert(odict::SearchResult *searchResult);
    const char* convert(Entry *entry);
};

#endif //ODICT_JSONCONVERTER_H
