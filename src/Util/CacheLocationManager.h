#ifndef ODICT_CACHELOCATIONMANAGER_H
#define ODICT_CACHELOCATIONMANAGER_H

#include <cstring>
#include <iostream>
#include <stdio.h>
#include <string>
#include <assert.h>

using namespace std;

class CacheLocationManager {
private:
    static CacheLocationManager* instance;
    const char* location;
    CacheLocationManager();
public:
    static CacheLocationManager* get_instance();
    void set_location(const char*);
    const char* get_location();
};


#endif //ODICT_CACHELOCATIONMANAGER_H
