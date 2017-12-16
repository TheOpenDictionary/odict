#include <iostream>
#include <sys/stat.h>

#include "DictionaryReader.h"
#include "Converters/JSONConverter.h"
#include "Util/EndianTypes.h"

bool file_exists(const char *path) {
    struct stat buffer;
    return (stat(path, &buffer) == 0);
}

/**
 * Reads the buffer from a compressed odict file
 * @param path
 * @return
 */
const uint8_t *DictionaryReader::read_buffer(const char *path) {
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
    string decompressed = string();

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
    input.read((char*)compressed, compressed_size);
    input.close();

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

/**
 * Looks up a word in the dictionary and returns the proper JSON
 * @param word
 * @param dictionary_path
 * @return
 */
string DictionaryReader::lookup(const char *word, const char *dictionary_path) {
    Timer *timer = new Timer();
    timer->start();

    auto *buf = this->read_buffer(dictionary_path);

    if (buf == 0) {
        cout << "Data is corrupted. Cannot read dictionary.\n" << endl;
        exit(0);
    } else {
        auto dict = GetDictionary(buf);
        auto entries = dict->entries();
        auto result = entries->LookupByKey(word);

        if (result != nullptr) {
            printf("Completed in %f seconds\n", timer->stop());
            return (new JSONConverter())->convert(result);
        } else {
            printf("Could not find dictionary entry \"%s\" in file %s", word, dictionary_path);
        }
    }

    return "{}";
}