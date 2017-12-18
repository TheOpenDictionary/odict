#ifndef ODICT_INDEXER_H
#define ODICT_INDEXER_H

#include <string>
#include <sys/stat.h>
#include <iostream>

#include <boost/filesystem.hpp>

#include "lucy.h"
#include "IndexSchema.h"
#include "CacheLocationManager.h"

using namespace std;

class IndexBuilder {
private:
    lucy_Schema* schema;
    cfish_String* folder;
    lucy_Indexer* indexer;

public:
    IndexBuilder(const char*);
    IndexBuilder* addDocument(const char*, const char*, const uint8_t*);
    void build();
};


#endif //ODICT_INDEXER_H
