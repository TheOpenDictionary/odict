#include "DictionaryWriter.h"

string getAttributeIfExists(xml_node<> *node, const char *attribute) {
    auto first_attr = node->first_attribute(attribute);

    if (first_attr == 0) return "";
    else {
        string attr(first_attr->value());
        return attr;
    }
}

DictionaryWriter::DictionaryWriter() {}

/**
 * Gets all of the definitions in a usage node and returns them as a FlatBuffer object
 * @param usage_node
 * @return
 */
Offset<Vector<Offset<String>>> DictionaryWriter::get_definitions(xml_node<> *node) {
    xml_node<> *current_def = node->first_node(NODE_DEFINITION);
    vector<Offset<String>> definitions = vector<Offset<String>>();

    while (current_def != 0) {
        string text(current_def->value());
        definitions.push_back(builder.CreateString(text));
        current_def = current_def->next_sibling(NODE_DEFINITION);
    }

    return builder.CreateVector(definitions);
}

/**
 * Gets all of the definitions in a group node and returns them as a FlatBuffer object
 * @param usage_node
 * @return
 */
Offset<Vector<Offset<Group>>> DictionaryWriter::get_groups(xml_node<> *usage_node) {
    xml_node<> *current_group = usage_node->first_node(NODE_GROUP);
    vector<Offset<Group>> groups = vector<Offset<Group>>();

    while (current_group != 0) {
        string id = getAttributeIfExists(current_group, ATTR_ID);
        string description = getAttributeIfExists(current_group, ATTR_DESCRIPTION);

        groups.push_back(CreateGroup(
                builder,
                builder.CreateString(id),
                builder.CreateString(description),
                this->get_definitions(current_group)
        ));

        current_group = current_group->next_sibling(NODE_DEFINITION);
    }

    return builder.CreateVector(groups);
}

Offset<Vector<Offset<Etymology>>> DictionaryWriter::get_etymologies(xml_node<>* entry_node) {
    xml_node<> *current_ety = entry_node->first_node(NODE_ETY);
    vector<Offset<Etymology>> etymologies = vector<Offset<Etymology>>();

    while (current_ety != 0) {
        string description = getAttributeIfExists(current_ety, ATTR_DESCRIPTION);
        auto usages = this->get_usages(current_ety);

        etymologies.push_back(CreateEtymology(
                builder,
                builder.CreateString(description),
                usages
        ));

        current_ety = current_ety->next_sibling(NODE_USAGE);
    }

    return builder.CreateVector(etymologies);
}

/**
 * Gets all of the usages in an entry node and returns them as a FlatBuffer object
 * @param entry_node
 * @return
 */
Offset<Vector<Offset<Usage>>> DictionaryWriter::get_usages(xml_node<> *ety_node) {
    xml_node<> *current_usage = ety_node->first_node(NODE_USAGE);
    vector<Offset<Usage>> usages = vector<Offset<Usage>>();

    while (current_usage != 0) {
        string part_of_speech = getAttributeIfExists(current_usage, ATTR_PART_OF_SPEECH);
        auto definitions = this->get_definitions(current_usage);
        auto groups = this->get_groups(current_usage);

        usages.push_back(CreateUsage(
                builder,
                builder.CreateString(part_of_speech),
                definitions,
                groups
        ));

        current_usage = current_usage->next_sibling(NODE_USAGE);
    }

    return builder.CreateVector(usages);
}

/**
 * Gets all of the entries in a dictionary node and returns them as a FlatBuffer object
 * @param dictionary_node
 * @return
 */
Offset<Vector<Offset<Entry>>> DictionaryWriter::get_entries(xml_node<> *dictionary_node) {
    xml_node<> *current_entry = dictionary_node->first_node(NODE_ENTRY);
    vector<Offset<Entry>> entries = vector<Offset<Entry>>();

    while (current_entry != 0) {
        string term = getAttributeIfExists(current_entry, ATTR_TERM);
        auto etymologies = this->get_etymologies(current_entry);

        entries.push_back(CreateEntry(builder, builder.CreateString(term), etymologies));
        current_entry = current_entry->next_sibling(NODE_ENTRY);
    }

    return builder.CreateVectorOfSortedTables(&entries);
}

/**
 * Compressed a buffer of a set size and writes it to the output file path
 * @param buf
 * @param size
 * @param path
 * @return
 */
bool DictionaryWriter::output_compressed_buffer(uint8_t *buf, int size, const char* output_file) {
    string compressed_str;

    Compress((char*)buf, size, &compressed_str);

    const char* compressed = compressed_str.c_str();
    unsigned long compressed_size = compressed_str.size();

    ofstream output(output_file, ios::out | ios::binary);
    output.write(reinterpret_cast<char *>(&ODICT_SIGNATURE), sizeof(ODICT_SIGNATURE));
    output.write(reinterpret_cast<char *>(&ODICT_VERSION), sizeof(ODICT_VERSION));
    output.write(reinterpret_cast<char *>(&compressed_size), sizeof(compressed_size));
    output.write(compressed, compressed_size);
    output.close();

    printf("Wrote %lu bytes (compressed from %d) to %s\n",
           compressed_size + sizeof(compressed_size),
           size,
           output_file
    );

    return true;
}

/**
 * Generates an .odict file given an input XML file
 * @param input_file
 * @param output_file
 */
void DictionaryWriter::generate(const char *input_file, const char *output_file) {
    Timer *timer = new Timer();
    timer->start();

    // 1) Read the input file
    rapidxml::file<> xmlFile(input_file);
    rapidxml::xml_document<> doc;

    // 2) Parse it
    doc.parse<0>(xmlFile.data());

    // 3) Find the base dictionary node
    xml_node<> *dictionary_node = doc.first_node("dictionary");

    if (dictionary_node != 0) {
        // 4) Get all of the entries (and children) from the dictionary root
        auto entries = this->get_entries(dictionary_node);
        auto dictionary = CreateDictionary(builder, entries);

        // 5) Make the dictionary and store it in a buffer
        FinishDictionaryBuffer(builder, dictionary);

        // 6) Get the buffer and its size
        uint8_t *buf = builder.GetBufferPointer();
        int size = builder.GetSize();

        // 7) Write the output to disk
        this->output_compressed_buffer(buf, size, output_file);

        // 8) Display total runtime
        printf("Completed in %f seconds\n", timer->stop());
    } else {
        cout << "Could not find base dictionary node. Terminating." << endl;
    }

}
