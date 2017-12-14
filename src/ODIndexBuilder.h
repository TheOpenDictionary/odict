#ifndef ODICT_INDEXER_H
#define ODICT_INDEXER_H

#include <string>
#include <sys/stat.h>

#include "lucy.h"
#include "ODIndexSchema.h"

using namespace std;

class ODIndexBuilder {
private:
    lucy_Indexer* indexer;
    lucy_Schema *schema;
    cfish_String *folder;

public:
    ODIndexBuilder(string);
    ODIndexBuilder* addDocument(string, string);
    void build();
};


#endif //ODICT_INDEXER_H
