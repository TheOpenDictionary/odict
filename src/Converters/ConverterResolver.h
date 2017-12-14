#ifndef ODICT_CONVERTERRESOLVER_H
#define ODICT_CONVERTERRESOLVER_H

#include "IConverter.h"
#include "Constants.h"
#include "JSONConverter.h"

using namespace odc;

class ConverterResolver {
public:
    static IConverter *resolve(const char*);
};


#endif //ODICT_CONVERTERRESOLVER_H
