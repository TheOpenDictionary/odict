#include "EntryJSONConverter.h"

EntryJSONConverter::EntryJSONConverter() {}

string EntryJSONConverter::add_definitions(const Vector<Offset<String>> *definitions) {
    string output = "\"definitions\":[";
    int length = definitions->Length();

    for (int i = 0;  i < length; i++) {
        const String *definition = definitions->Get(i);
        output += "\"" + definition->str() + "\"";
        if (i != length - 1) output += ",";
    }

    output += "]";

    return output;
}

string EntryJSONConverter::add_usages(const Vector<Offset<Usage>> *usages) {
    string output = "\"usages\":[";
    int length = usages->Length();

    for (int i = 0;  i < length; i++) {
        const Usage *usage = usages->Get(i);
        const Vector<Offset<String>> *definitions = usage->definitions();

        output += "{\"pos\":\"" + usage->pos()->str() + "\"";

        if (definitions->Length() > 0) {
            output += ",";
            output += this->add_definitions(definitions);
        }

        output += "}";

        if (i != length - 1) output += ",";
    }

    output += "]";

    return output;
}

string EntryJSONConverter::convert(const Entry *entry) {
    string output = "{";

    output += this->add_usages(entry->usages());
    output += "}";

    return output;
}
