#ifndef ODICT_ODINDEXSCHEMA_H
#define ODICT_ODINDEXSCHEMA_H

#include "lucy.h"

class ODIndexSchema {
private:
    static lucy_Schema *instance;
    ODIndexSchema();

public:
    static lucy_Schema* getInstance();
};


#endif //ODICT_ODINDEXSCHEMA_H
