#include "JSONConverter.h"

JSONConverter::JSONConverter() {}

void JSONConverter::add_definitions(pt::ptree *root, const Vector<Offset<String>> *definitions) {
    int length = definitions->Length();

    pt::ptree list;

    for (int i = 0; i < length; i++) {
        pt::ptree def_node;
        def_node.put("", definitions->Get(i)->str());
        list.push_back(make_pair("", def_node));
    }

    root->add_child("definitions", list);
}

void JSONConverter::add_groups(pt::ptree *root, const Vector<Offset<Group>> *groups) {
    int length = groups->Length();

    pt::ptree list;

    for (int i = 0; i < length; i++) {
        const Group *group = groups->Get(i);
        const Vector<Offset<String>> *definitions = group->definitions();
        vector<string> properties = vector<string>();

        pt::ptree group_node;

        group_node.put("id", group->id()->str());

        if (group->description()->str().length() > 0)
            group_node.put("description", group->description()->str());

        if (definitions->Length() > 0)
            this->add_definitions(&group_node, definitions);

        list.push_back(make_pair("", group_node));
    }

    root->add_child("groups", list);
}

void JSONConverter::add_usages(pt::ptree *root, const Vector<Offset<Usage>> *usages) {
    int length = usages->Length();

    pt::ptree list;

    for (int i = 0; i < length; i++) {
        const Usage *usage = usages->Get(i);
        const Vector<Offset<Group>> *groups = usage->groups();
        const Vector<Offset<String>> *definitions = usage->definitions();
        vector<string> properties = vector<string>();

        pt::ptree usage_node;

        usage_node.put("id", usage->id()->str());

        if (usage->pos()->str().length() > 0)
            usage_node.put("pos", usage->pos()->str());

        if (groups->Length() > 0)
            this->add_groups(&usage_node, groups);

        if (definitions->Length() > 0)
            this->add_definitions(&usage_node, definitions);

        list.push_back(make_pair("", usage_node));
    }

    root->add_child("usages", list);
}

void JSONConverter::add_etymologies(pt::ptree *root, const Vector<Offset<Etymology>> *etymologies) {
    int length = etymologies->Length();

    pt::ptree list;

    for (int i = 0; i < length; i++) {
        const Etymology *etymology = etymologies->Get(i);
        const Vector<Offset<Usage>> *usages = etymology->usages();

        pt::ptree ety_node;

        ety_node.put("id", etymology->id()->str());

        if (etymology->description()->str().length() > 0)
            ety_node.put("description", etymology->description()->str());

        if (usages->Length() > 0)
            this->add_usages(&ety_node, usages);

        // TODO: require at least one usage before writing
        list.push_back(std::make_pair("", ety_node));
    }

    root->add_child("etymologies", list);
}

const char* JSONConverter::convert(odict::SearchResult *searchResult) {
    pt::ptree root;
    stringstream ss;

    root.put("query", searchResult->getQuery());

    auto results = searchResult->getResults();
    auto result = results.begin();

    pt::ptree list;

    while (result != results.end()) {
        pt::ptree entry_node;

        entry_node.put("id", (*result)->id()->str());
        entry_node.put("term", (*result)->term()->str());

        this->add_etymologies(&entry_node, (*result)->etymologies());

        list.push_back(std::make_pair("", entry_node));

        result++;
    }

    root.add_child("results", list);

    pt::write_json(ss, root);

    // We have to copy the string out of local memory so we don't lose it
    string output = ss.str();
    char * str = new char[output.length() + 1];
    strcpy(str, output.c_str());

    return str;
}


const char* JSONConverter::convert(Entry *entry) {
    pt::ptree root;
    stringstream ss;

    this->add_etymologies(&root, entry->etymologies());

    pt::write_json(ss, root);

    // We have to copy the string out of local memory so we don't lose it
    string output = ss.str();
    char * str = new char[output.length() + 1];
    strcpy(str, output.c_str());

    return str;
}
