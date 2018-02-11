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
    static CacheLocationManager* getInstance();
    void setLocation(const char*);
    const char* getLocation();
};


#endif //ODICT_CACHELOCATIONMANAGER_H
