#ifndef ODICT_ICONVERTER_H
#define ODICT_ICONVERTER_H

#include <string>

#include "../schema_generated.h"

using namespace schema;
using namespace std;

class IConverter {
public:
    virtual const char *convert(const Entry *entry) = 0;
};

#endif //ODICT_ICONVERTER_H
