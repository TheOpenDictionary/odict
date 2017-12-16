#include "JSONConverter.h"

JSONConverter::JSONConverter() {}

string propertiesToJSON(vector<string> properties) {
    string output = "";

    for (int i = 0; i < properties.size(); i++) {
        output += properties.at(i);

        if (i != properties.size() - 1) output += ",";
    }

    return output;
}

string JSONConverter::add_definitions(const Vector<Offset<String>> *definitions) {
    string output = "\"definitions\":[";
    int length = definitions->Length();

    for (int i = 0; i < length; i++) {
        const String *definition = definitions->Get(i);
        output += "\"" + definition->str() + "\"";
        if (i != length - 1) output += ",";
    }

    output += "]";

    return output;
}

string JSONConverter::add_groups(const Vector<Offset<Group>> *groups) {
    string output = "\"groups\":[";
    int length = groups->Length();

    for (int i = 0; i < length; i++) {
        const Group *group = groups->Get(i);
        const Vector<Offset<String>> *definitions = group->definitions();
        vector<string> properties = vector<string>();

        output += "{";

        if (group->id()->str().length() > 0)
            properties.push_back("id:\"" + group->id()->str() + "\",");

        if (group->description()->str().length() > 0)
            properties.push_back("\"description\":\"" + group->description()->str() + "\",");

        if (definitions->Length() > 0) {
            properties.push_back(this->add_definitions(definitions));
        }

        output += propertiesToJSON(properties);
        output += "}";

        if (i != length - 1) output += ",";
    }

    output += "]";

    return output;
}

string JSONConverter::add_usages(const Vector<Offset<Usage>> *usages) {
    string output = "\"usages\":[";
    int length = usages->Length();

    for (int i = 0; i < length; i++) {
        const Usage *usage = usages->Get(i);
        const Vector<Offset<Group>> *groups = usage->groups();
        const Vector<Offset<String>> *definitions = usage->definitions();
        vector<string> properties = vector<string>();

        output += "{";

        if (usage->pos()->str().length() > 0)
            properties.push_back("\"pos\":\"" + usage->pos()->str() + "\"");

        if (groups->Length() > 0)
            properties.push_back(this->add_groups(groups));

        if (definitions->Length() > 0)
            properties.push_back(this->add_definitions(definitions));

        output += propertiesToJSON(properties);
        output += "}";

        if (i != length - 1) output += ",";
    }

    output += "]";

    return output;
}

string JSONConverter::add_etymologies(const Vector<Offset<Etymology>> *etymologies) {
    string output = "\"etymologies\":[";
    int length = etymologies->Length();

    for (int i = 0; i < length; i++) {
        const Etymology *etymology = etymologies->Get(i);
        const Vector<Offset<Usage>> *usages = etymology->usages();

        output += "{";

        if (etymology->description()->str().length() > 0)
            output += "\"description\":\"" + etymology->description()->str() + "\",";

        if (usages->Length() > 0)
            output += this->add_usages(usages);

        // TODO: require at least one usage before writing

        output += "}";

        if (i != length - 1) output += ",";
    }

    output += "]";

    return output;
}

string JSONConverter::convert(const Entry *entry) {
    string output = "{";

    output += this->add_etymologies(entry->etymologies());
    output += "}";

    return output;
}
