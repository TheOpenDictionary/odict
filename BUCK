SNAPPY_VERSION = '1.1.7'

# The ODict Library
cxx_library(
  name = 'libodict',
  header_namespace = 'odict',
  headers = subdir_glob([
    ('src', '**/*.h'),
  ]),
  srcs = glob([
    'src/**/*.cpp',
  ]),
  deps = ['//vendor/flatbuffers:flatbuffers'],
  visibility = ['PUBLIC']
)

# The ODict Binary
cxx_binary(
  name = 'odict',
  srcs = ['src/main.cpp'],
  deps = [':libodict'],
  visibility = ['PUBLIC']
)