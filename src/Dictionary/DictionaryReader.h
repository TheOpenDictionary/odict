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
    const uint8_t *GetBuffer(const char*);
    const void generateIndex(const Dictionary*);
    const void generateIndex(const uint8_t*);
public:
    const uint8_t *ReadAsBuffer(const char*);
    const uint8_t *ReadAsBuffer(const char*, bool);
    const Dictionary *ReadAsDictionary(const char*);
};


#endif //ODICT_DICTIONARYREADER_H
