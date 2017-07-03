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

    unsigned long compressed_size;
    string decompressed;

    int size_of_long = sizeof(unsigned long);
    int file_size = input.tellg();

    input.seekg(0);
    input.read(reinterpret_cast<char *>(&compressed_size), size_of_long);

    assert((compressed_size + size_of_long) == file_size);

    const char *compressed = new char[compressed_size];

    input.seekg(size_of_long);
    input.read((char *) compressed, compressed_size);
    input.close();

    Uncompress(compressed, compressed_size, &decompressed);

    unsigned long size = decompressed.size();

    vector<uint8_t> *copy_vec = new vector<uint8_t>(decompressed.begin(), decompressed.end());
    const uint8_t *buf = &(*copy_vec)[0];

    printf("Decompressed dictionary from %lu bytes to %lu\n", compressed_size, size);

    if (buf == NULL)
        return 0;

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