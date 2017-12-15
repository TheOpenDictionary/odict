#ifndef ODICT_DICTIONARYBUILDER_H
#define ODICT_DICTIONARYBUILDER_H

#include <fstream>
#include <ios>
#include <iostream>
#include <string>

#include <boost/lexical_cast.hpp>
#include <boost/uuid/uuid_io.hpp>
#include <boost/uuid/uuid.hpp>
#include <boost/uuid/random_generator.hpp>

#include "snappy.h"
#include "rapidxml.hpp"
#include "rapidxml_utils.hpp"

#include "schema_generated.h"

#include "Util/Constants.h"
#include "Util/Timer.h"

using namespace std;
using namespace rapidxml;
using namespace flatbuffers;
using namespace schema;
using namespace snappy;

class DictionaryWriter {
public:
    DictionaryWriter();
    void generate(const char*, const char*);
private:
    FlatBufferBuilder builder;
    Offset<String> get_uuid_string();
    Offset<Vector<Offset<Entry>>> get_entries(xml_node<>*);
    Offset<Vector<Offset<Etymology>>> get_etymologies(xml_node<>*);
    Offset<Vector<Offset<Usage>>> get_usages(xml_node<>*);
    Offset<Vector<Offset<Group>>> get_groups(xml_node<>*);
    Offset<Vector<Offset<String>>> get_definitions(xml_node<>*);
    bool output_compressed_buffer(uint8_t*, int, const char*);
};

#endif //ODICT_DICTIONARYBUILDER_H
