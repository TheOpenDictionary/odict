#ifndef ODICT_DICTIONARYBUILDER_H
#define ODICT_DICTIONARYBUILDER_H

#include <fstream>
#include <ios>
#include <iostream>
#include <string>

#include "rapidxml.hpp"
#include "rapidxml_utils.hpp"
#include "snappy.h"

#include "constants.h"
#include "schema_generated.h"
#include "timer.h"

using namespace std;
using namespace rapidxml;
using namespace flatbuffers;
using namespace schema;
using namespace snappy;

#import "constants.h"

class DictionaryWriter {
public:
    DictionaryWriter();
    void generate(const char*, const char*);
private:
    FlatBufferBuilder builder;
    Offset<Vector<Offset<Entry>>> get_entries(xml_node<>*);
    Offset<Vector<Offset<Etymology>>> get_etymologies(xml_node<>*);
    Offset<Vector<Offset<Usage>>> get_usages(xml_node<>*);
    Offset<Vector<Offset<Group>>> get_groups(xml_node<>*);
    Offset<Vector<Offset<String>>> get_definitions(xml_node<>*);
    bool output_compressed_buffer(uint8_t*, int, const char*);
};

#endif //ODICT_DICTIONARYBUILDER_H
