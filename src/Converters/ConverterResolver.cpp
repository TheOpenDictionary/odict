#include "ConverterResolver.h"

IConverter *ConverterResolver::resolve(const char *format) {
    if (!strcmp(format, FORMAT_JSON))
        return new JSONConverter();
    return nullptr;
}