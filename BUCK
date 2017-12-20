# The ODict Library
cxx_library(
  name = 'libodict',
  header_namespace = 'odict',
  headers = glob([
    'src/*.h',
    'src/**/*.h'
  ]),
  srcs = glob([
    'src/**/*.cpp',
  ]),
  deps = [
    '//vendor/flatbuffers:flatbuffers',
    '//vendor/snappy:snappy',
    '//vendor/rapidxml:rapidxml'
  ],
  visibility = ['PUBLIC']
)

# The ODict Binary
cxx_binary(
  name = 'odict',
  srcs = ['src/main.cpp'],
  deps = [':libodict'],
  visibility = ['PUBLIC']
)