#include "ODIndexSchema.h"

lucy_Schema *create_schema() {
    // Create a new schema.
    lucy_Schema *schema = lucy_Schema_new();

    // Create an analyzer.
    cfish_String *language = cfish_Str_newf("en");
    lucy_EasyAnalyzer *analyzer = lucy_EasyAnalyzer_new(language);

    // Specify fields.
    lucy_FullTextType *type = lucy_FullTextType_new((lucy_Analyzer *) analyzer);

    {
        cfish_String *field_str = cfish_Str_newf("title");
        LUCY_Schema_Spec_Field(schema, field_str, (lucy_FieldType *) type);
        CFISH_DECREF(field_str);
    }

    {
        cfish_String *field_str = cfish_Str_newf("content");
        LUCY_Schema_Spec_Field(schema, field_str, (lucy_FieldType *) type);
        CFISH_DECREF(field_str);
    }

    CFISH_DECREF(type);
    CFISH_DECREF(analyzer);
    CFISH_DECREF(language);

    return schema;
}

lucy_Schema* ODIndexSchema::instance = 0;

lucy_Schema* ODIndexSchema::getInstance() {
    if (instance == 0)
        instance = create_schema();
    return instance;
}