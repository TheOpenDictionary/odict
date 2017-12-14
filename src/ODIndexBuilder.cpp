#include "ODIndexBuilder.h"

ODIndexBuilder::ODIndexBuilder(string name) {
    lucy_bootstrap_parcel();

    string base = ".odict";
    string path = base + "/" + name;

    mkdir(base.c_str(), S_IRWXU | S_IRWXG | S_IROTH | S_IXOTH);

    this->schema = ODIndexSchema::getInstance();
    this->folder = cfish_Str_newf("%s", path.c_str());
    this->indexer = lucy_Indexer_new(schema, (cfish_Obj *) folder, NULL, lucy_Indexer_CREATE | lucy_Indexer_TRUNCATE);
}

ODIndexBuilder* ODIndexBuilder::addDocument(string title, string tokens) {
    lucy_Doc *doc = lucy_Doc_new(NULL, 0);

    const char* title_cstr = title.c_str();
    const char* tokens_cstr = tokens.c_str();

    {
        // Store 'title' field
        cfish_String *field = cfish_Str_newf("title");
        cfish_String *value = cfish_Str_new_from_utf8(title_cstr, strlen(title_cstr));
        LUCY_Doc_Store(doc, field, (cfish_Obj*)value);
        CFISH_DECREF(field);
        CFISH_DECREF(value);
    }

    {
        // Store 'content' field
        cfish_String *field = cfish_Str_newf("content");
        cfish_String *value = cfish_Str_new_from_utf8(tokens_cstr, strlen(tokens_cstr));
        LUCY_Doc_Store(doc, field, (cfish_Obj*)value);
        CFISH_DECREF(field);
        CFISH_DECREF(value);
    }

    LUCY_Indexer_Add_Doc(this->indexer, doc, 1.0);

    CFISH_DECREF(doc);

    return this;
}

void ODIndexBuilder::build() {
    LUCY_Indexer_Commit(this->indexer);

    CFISH_DECREF(this->indexer);
    CFISH_DECREF(this->folder);
    CFISH_DECREF(this->schema);
}