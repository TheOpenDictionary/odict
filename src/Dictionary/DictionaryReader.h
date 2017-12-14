#ifndef ODICT_DICTIONARYREADER_H
#define ODICT_DICTIONARYREADER_H

#include <fstream>
#include <ios>
#include <string>
#include <snappy.h>

<<<<<<< HEAD
#include "../schema_generated.h"
#include "../Util/Timer.h"
#include "../Util/Constants.h"
=======
#include "ODIndexBuilder.h"
#include "schema_generated.h"
#include "Util/Timer.h"
#include "Util/Constants.h"
>>>>>>> Began implementation of Apache Lucy

using namespace std;
using namespace schema;
using namespace snappy;
using namespace flatbuffers;
using namespace odc;

class DictionaryReader {
private:
    FlatBufferBuilder builder;
    const uint8_t *GetBuffer(const char* path);
public:
    const uint8_t *ReadAsBuffer(const char* dictionary_path);
    const Dictionary *ReadAsDictionary(const char* dictionary_path);
};


#endif //ODICT_DICTIONARYREADER_H
