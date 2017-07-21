#include <iostream>

#include "DictionaryReader.h"
#include "EntryJSONConverter.h"

/**
 * Reads the buffer from a compressed odict file
 * @param path
 * @return
 */
const uint8_t *DictionaryReader::read_buffer(const char *path) {
    ifstream input(path, ios::in | ios::binary | ios::ate);

    // Prep data to be read in
    char *signature = new char[sizeof(ODICT_SIGNATURE)]();
    unsigned int version;
    unsigned long compressed_size;
    const char *compressed = new char[compressed_size]();
    string decompressed;

    // Calculate data sizes based on OS
    int size_of_sig = sizeof(ODICT_SIGNATURE);
    int size_of_int = sizeof(unsigned int);
    int size_of_long = sizeof(unsigned long);
    int file_size = input.tellg();

    // Read the signature
    input.seekg(0);
    input.read(signature, size_of_sig);

    // Read the file version
    input.seekg(size_of_sig);
    input.read(reinterpret_cast<char *>(&version), size_of_int);

    // Read the length of the compressed data
    input.seekg(size_of_sig + size_of_int);
    input.read(reinterpret_cast<char *>(&compressed_size), size_of_long);

    // Make varying assertions to ensure file is valid
    int header_length = size_of_long + size_of_sig + size_of_int;

    assert(strcmp(signature, ODICT_SIGNATURE) == 0);
    assert((compressed_size + header_length) == file_size);

    // Read in compressed data
    input.seekg(header_length);
    input.read((char *) compressed, compressed_size);
    input.close();

    // Decompress the buffer
    Uncompress(compressed, compressed_size, &decompressed);

    // Copy the buffer into a new uint8_t object
    unsigned long size = decompressed.size();
    vector<uint8_t> *copy_vec = new vector<uint8_t>(decompressed.begin(), decompressed.end());
    const uint8_t *buf = &(*copy_vec)[0];

    printf("File built with ODict v%d.0\n", version);
    printf("Decompressed dictionary from %lu bytes to %lu\n", compressed_size, size);

    // Return 0 if we have a dead buffer
    if (buf == NULL)
        return 0;

    // Verify and return the buffer
    Verifier verifier = Verifier(buf, size);
    return (VerifyDictionaryBuffer(verifier)) ? buf : 0;
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

    auto buf = this->read_buffer(dictionary_path);

    if (buf == 0) {
        throw runtime_error("Data is corrupted. Cannot read dictionary.\n");
    } else {
        auto dict = GetDictionary(buf);
        auto entries = dict->entries();
        auto result = entries->LookupByKey(word);

        if (result != nullptr) {
            printf("Completed in %f seconds\n", timer->stop());
            return (new EntryJSONConverter())->convert(result);
        } else {
            printf("Could not find dictionary entry \"%s\" in file %s", word, dictionary_path);
        }
    }

    return "{}";
}