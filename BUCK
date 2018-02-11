# The ODict Library
cxx_library(
  name = 'libodict',
  soname = 'odict',
  header_namespace = 'odict',
  headers = glob([
    'src/*.h',
    'src/**/*.h'
  ]),
  srcs = glob([
    'src/**/*.cpp',
  ]),
  deps = [
    '//vendor:flatbuffers',
    '//vendor:snappy',
    '//vendor:rapidxml',
    '//vendor:boost-filesystem',
    '//vendor:boost-system',
    '//vendor:clownfish',
    '//vendor:lucy'
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