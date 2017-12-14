#include "Dictionary/DictionaryWriter.h"
#include "Dictionary/DictionarySearch.h"

static const char *CMD_GENERATE = "generate";
static const char *CMD_READ = "lookup";
static const string OUTPUT_EXT = "odict";

/**
 * Shows CLI usage info
 */
void show_usage() {
    cout << "Usage: odict [generate|lookup] [..args]" << endl;
}

/**
 * Returns the filename (minus the extension) from a full filepath
 * @param path
 * @return
 */
string get_filename_from_path(string path) {
    string input_file(path);

    int last_index_of_slash = input_file.find_last_of("/");
    int last_index_of_dot = input_file.find_last_of(".");

    last_index_of_slash = (last_index_of_slash < 0) ? 0 : last_index_of_slash + 1;
    last_index_of_dot = (last_index_of_dot < 0) ? input_file.length() - 1 : last_index_of_dot;

    int substr_len = last_index_of_dot - last_index_of_slash;

    string filename = input_file.substr(last_index_of_slash, substr_len);

    return "./" + filename + "." + OUTPUT_EXT;
}

int main(int argv, char *args[]) {
    auto ed = new EndianTypes();
    ed->init();
    if (argv < 3) show_usage();
    else {
        string input_file(args[2]);
        string output_full_path = get_filename_from_path(input_file);

        if (strcmp(args[1], CMD_GENERATE) == 0) {
            if (argv < 3) cout << "Usage: odict generate [xml file]" << endl;
            else {
                DictionaryWriter *generator = new DictionaryWriter();
                generator->generate(input_file.c_str(), output_full_path.c_str());
            }
        } else if (strcmp(args[1], CMD_READ) == 0) {
            if (argv < 4) cout << "Usage: odict lookup [word] [odict file]" << endl;
            else {
                DictionarySearch *search = new DictionarySearch(args[3]);
                const char* output = search->searchByEntry(args[2]);
                cout << endl << output << endl;
            }
        } else {
            show_usage();
        }
    }
//    search();
}