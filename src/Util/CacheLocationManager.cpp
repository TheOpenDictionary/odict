#include "CacheLocationManager.h"

CacheLocationManager *CacheLocationManager::instance = 0;

/**
 * Expands a path that includes ~ to include the full user home directory
 * @param path
 * @return
 */
const char* expand_user(std::string path) {
    if (not path.empty() and path[0] == '~') {
        assert(path.size() == 1 or path[1] == '/');  // or other error handling
        char const *home = getenv("HOME");
        if (home || (home = getenv("USERPROFILE"))) {
            path.replace(0, 1, home);
        } else {
            char const *hdrive = getenv("HOMEDRIVE"),
                    *hpath = getenv("HOMEPATH");
            assert(hdrive);  // or other error handling
            assert(hpath);
            path.replace(0, 1, std::string(hdrive) + hpath);
        }
    }

    char * str = new char[path.length() + 1];
    strcpy(str, path.c_str());

    return str;
}

CacheLocationManager::CacheLocationManager() : location(expand_user("~/.odict")) { }

/**
 * Returns the singleton instance
 * @return
 */
CacheLocationManager *CacheLocationManager::getInstance() {
    if (instance == 0)
        instance = new CacheLocationManager();
    return instance;
}

/**
 * Sets the cache location path
 * @param location
 */
void CacheLocationManager::setLocation(const char* path) {
    this->location = expand_user(string(path) + "/.odict");
}

/**
 * Gets the cache location path
 * @return
 */
const char* CacheLocationManager::getLocation() {
    return this->location;
}