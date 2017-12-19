#include <iostream>
#include <sys/stat.h>

#include "../Converters/JSONConverter.h"
#include "../Util/EndianTypes.h"

#include "DictionaryReader.h"

/**
 * Checks for file existance
 * @param path path to file
 * @return
 */
bool file_exists(const char *path) {
    struct stat buffer;
    return (stat(path, &buffer) == 0);
}

/**
 * Reads the buffer from a compressed odict file
 * @param path
 * @return
 */
const uint8_t *DictionaryReader::GetBuffer(const char *path) {
    ifstream input(path, ios::in | ios::binary | ios::ate);

    if (!file_exists(path)) {
        cout << "File " << path << " does not exist." << endl;
        exit(0);
    }

    cout << "Reading file " << path << "..." << endl;

    // Prep data to be read in
    char *signature = new char[sizeof(ODICT_SIGNATURE)]();
    unsigned short version = 0U;
    unsigned long compressed_size = 0UL;

    // Calculate data sizes based on OS
    int size_of_sig = sizeof(ODICT_SIGNATURE) - 1; // -1 for terminating character
    int size_of_short = sizeof(unsigned short);
    int size_of_long = sizeof(unsigned long);
    int file_size = input.tellg();

    assert(file_size > -1);

    // Read the signature
    input.seekg(0);
    input.read(signature, size_of_sig);

    // Read the file version
    input.seekg(size_of_sig);
    input.read(reinterpret_cast<char *>(&version), size_of_short);
    version = little_short(version);

    // Read the length of the compressed data
    input.seekg(size_of_sig + size_of_short);
    input.read(reinterpret_cast<char *>(&compressed_size), size_of_long);
    compressed_size = little_long(compressed_size);

    // Make varying assertions to ensure file is valid
    int header_length = size_of_long + size_of_sig + size_of_short;

    assert(strcmp(signature, ODICT_SIGNATURE) == 0);
    assert((compressed_size + header_length) == file_size);

    // Read in compressed data
    const char *compressed = new char[compressed_size]();
    input.seekg(header_length);
    input.read((char *) compressed, compressed_size);
    input.close();

    string decompressed = string();

    // Decompress the buffer and return it
    if (Uncompress(compressed, compressed_size, &decompressed)) {
        unsigned long size = decompressed.size();
        vector<uint8_t> *copy_vec = new vector<uint8_t>(decompressed.begin(), decompressed.end());
        const uint8_t *buf = &(*copy_vec)[0];

        printf("File built with ODict v%d.0\n", version);
        printf("Decompressed dictionary from %lu bytes to %lu\n", compressed_size, size);

        Verifier verifier = Verifier(buf, size);
        return (VerifyDictionaryBuffer(verifier)) ? buf : 0;
    } else {
        cout << "Compressed data is corrupted. Could not decompress." << endl;
        exit(0);
    }
}

const void DictionaryReader::generateIndex(const Dictionary* dict) {
    IndexBuilder *builder = new IndexBuilder(dict->id()->c_str());
    auto entries = dict->entries();
    auto entry = entries->begin();

    while(entry != entries->end()) {
        auto etymologies = entry->etymologies();
        auto ety = etymologies->begin();

        string keyword_block = "";

        while (ety != etymologies->end()) {
            auto usages = ety->usages();
            auto usage = usages->begin();

            while (usage != usages->end()) {
                auto groups = usage->groups();
                auto group = groups->begin();

                auto definitions = usage->definitions();
                auto definition = definitions->begin();

                while(group != groups->end()) {
                    auto definitions = group->definitions();
                    auto definition = definitions->begin();

                    while(definition != definitions->end()) {
                        keyword_block += definition->str() + " ";
                        definition++;
                    }

                    group++;
                }

                while(definition != definitions->end()) {
                    keyword_block += definition->str() + " ";
                    definition++;
                }
                usage++;
            }
            ety++;
        }
        builder->addDocument(entry->term()->c_str(), keyword_block.c_str());
        entry++;
    }
    builder->build();
}

const void DictionaryReader::generateIndex(const uint8_t *buffer) {
    auto dict = GetDictionary(buffer);
    this->generateIndex(dict);
}

/**
 * Loads a dictionary into memory as a raw buffer
 * @param dictionary_path
 * @return
 */
const uint8_t *DictionaryReader::ReadAsBuffer(const char *dictionary_path, bool buildIndex) {
    const uint8_t *buf = this->GetBuffer(dictionary_path);

    if (buf == 0) {
        cout << "Data is corrupted. Cannot read dictionary.\n" << endl;
        exit(0);
    }

    if (buildIndex)
        generateIndex(buf);

    return buf;
}

/**
 * Loads a dictionary into memory as a raw buffer
 * @param dictionary_path
 * @return
 */
const uint8_t *DictionaryReader::ReadAsBuffer(const char *dictionary_path) {
    return this->ReadAsBuffer(dictionary_path, true);
}

/**
 * Loads a dictionary as a Flatbuffers Dictionary object
 * @param dictionary_path
 * @return
 */
const Dictionary *DictionaryReader::ReadAsDictionary(const char *dictionary_path) {
    const Dictionary *dict = GetDictionary(this->ReadAsBuffer(dictionary_path, false));
    this->generateIndex(dict);
    return dict;
}