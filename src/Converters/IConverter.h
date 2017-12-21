#ifndef ODICT_ICONVERTER_H
#define ODICT_ICONVERTER_H

#include <string>

#include "../schema_generated.h"

using namespace schema;
using namespace std;

class IConverter {
    virtual string convert(const Entry *entry) = 0;
};

#endif //ODICT_ICONVERTER_H
