#ifndef ODICT_DICTIONARYREADER_H
#define ODICT_DICTIONARYREADER_H

#include <fstream>
#include <ios>
#include <string>
#include <snappy.h>

#include "../schema_generated.h"
#include "../Util/Timer.h"
#include "../Util/Constants.h"

using namespace std;
using namespace schema;
using namespace snappy;
using namespace flatbuffers;

class DictionaryReader {
private:
    FlatBufferBuilder builder;
    const uint8_t *read_buffer(const char* path);
public:
    string lookup(const char* word, const char* dictionary_path);
};


#endif //ODICT_DICTIONARYREADER_H
