#include "IndexBuilder.h"

void create_output_dir(const char* name) {
    string path = string(CacheLocationManager::getInstance()->getLocation()) + "/" + string(name);

    if (!boost::filesystem::exists(path)) {
        if (!boost::filesystem::create_directories(path)) {
            cout << "An error occurred creating the index directory" << endl;
            exit(0);
        }
    }
}

IndexBuilder::IndexBuilder(const char *name) :
        schema(IndexSchema::getInstance()),
        folder((create_output_dir(name), cfish_Str_newf("%s/%s", CacheLocationManager::getInstance()->getLocation(),
                                                    name))),
        indexer((lucy_bootstrap_parcel(), lucy_Indexer_new(schema, (cfish_Obj *) folder, NULL,
                                                           lucy_Indexer_CREATE | lucy_Indexer_TRUNCATE))) {
}

IndexBuilder *IndexBuilder::addDocument(const char *title, const char *tokens) {
    lucy_Doc *doc = lucy_Doc_new(NULL, 0);

    {
        // Store 'title' field
        cfish_String *field = cfish_Str_newf("title");
        cfish_String *value = cfish_Str_new_from_utf8(title, strlen(title));
        LUCY_Doc_Store(doc, field, (cfish_Obj *) value);
        CFISH_DECREF(field);
        CFISH_DECREF(value);
    }

    {
        // Store 'content' field
        cfish_String *field = cfish_Str_newf("tokens");
        cfish_String *value = cfish_Str_new_from_utf8(tokens, strlen(tokens));
        LUCY_Doc_Store(doc, field, (cfish_Obj *) value);
        CFISH_DECREF(field);
        CFISH_DECREF(value);
    }

    LUCY_Indexer_Add_Doc(this->indexer, doc, 1.0);

    CFISH_DECREF(doc);

    return this;
}

void IndexBuilder::build() {
    LUCY_Indexer_Commit(this->indexer);

    CFISH_DECREF(this->indexer);
    CFISH_DECREF(this->folder);
    CFISH_DECREF(this->schema);
}