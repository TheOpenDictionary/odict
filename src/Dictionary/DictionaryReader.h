#ifndef ODICT_DICTIONARYREADER_H
#define ODICT_DICTIONARYREADER_H

#include <fstream>
#include <ios>
#include <string>

#include "snappy.h"
#include "flatbuffers/flatbuffers.h"

#include "../schema_generated.h"
#include "../Util/Timer.h"
#include "../Util/Constants.h"
#include "../Indexer/IndexBuilder.h"

using namespace std;
using namespace schema;
using namespace snappy;
using namespace flatbuffers;
using namespace odc;

class DictionaryReader {
private:
    FlatBufferBuilder builder;
    const uint8_t *get_buffer(const char*);
    // const void generate_index(const Dictionary*);
    // const void generate_index(const uint8_t*);
public:
    const uint8_t *read_as_buffer(const char*);
    const uint8_t *read_as_buffer(const char*, bool);
    const Dictionary *read_as_dictionary(const char*);
};


#endif //ODICT_DICTIONARYREADER_H
