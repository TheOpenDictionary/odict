#include <jni.h>
#include <stdio.h>
#include <iostream>
#include "bridge/bridge.h"

using namespace std;

#ifndef CGO_EXPORT_BRIDGE_EXISTS
#error bridging header not found
#endif

extern "C" JNIEXPORT jstring JNICALL
Java_org_odict_Dictionary_lookup(JNIEnv *env, jobject, jstring query, jstring dict)
{
  const char *dictionary = env->GetStringUTFChars(dict, 0);
  const char *entry_term = env->GetStringUTFChars(query, 0);
  return env->NewStringUTF(LookupEntry((char *)entry_term, (char *)dictionary));
}

extern "C" JNIEXPORT void JNICALL
Java_org_odict_Dictionary_index(JNIEnv *env, jobject, jstring encoded)
{
  IndexDictionary((char *)env->GetStringUTFChars(encoded, 0));
}

extern "C" JNIEXPORT jstring JNICALL
Java_org_odict_Dictionary_search(JNIEnv *env, jobject, jstring query, jstring encoded)
{
  const char *q = env->GetStringUTFChars(query, 0);
  const char *result = SearchDictionary((char *)q, (char *)env->GetStringUTFChars(encoded, 0));

  return env->NewStringUTF(result);
}

extern "C" JNIEXPORT jstring JNICALL
Java_org_odict_Dictionary_read(JNIEnv *env, jobject, jstring path)
{
  const char *dictionary_path = env->GetStringUTFChars(path, 0);
  char *encoded = ReadDictionary((char *)dictionary_path);

  return env->NewStringUTF(encoded);
}

extern "C" JNIEXPORT void JNICALL
Java_org_odict_Dictionary_write(JNIEnv *env, jobject, jstring xml, jstring path)
{
  const char *dictionary_path = env->GetStringUTFChars(path, 0);
  const char *content = env->GetStringUTFChars(xml, 0);
  WriteDictionary((char *)content, (char *)dictionary_path);
}

extern "C" JNIEXPORT void JNICALL
Java_org_odict_Dictionary_compile(JNIEnv *env, jobject, jstring path)
{
  const char *dictionary_path = env->GetStringUTFChars(path, 0);
  CompileDictionary((char *)dictionary_path);
}
