#include <jni.h>
#include <stdio.h>
#include <iostream>
#include "bridge/archive.h"

using namespace std;

#ifndef CGO_EXPORT_BRIDGE_EXISTS
#error bridging header not found
#endif

extern "C" JNIEXPORT jstring JNICALL
Java_org_odict_Dictionary_lookup(JNIEnv *env, jobject, jstring query, jstring dict)
{
  const char *dictionary = env->GetStringUTFChars(dict, 0);
  const char *entry_term = env->GetStringUTFChars(query, 0);
  char *result = LookupEntry((char *)entry_term, (char *)dictionary);
  jstring str = env->NewStringUTF(result);

  free(result);

  return str;
}

extern "C" JNIEXPORT void JNICALL
Java_org_odict_Dictionary_index(JNIEnv *env, jobject, jstring encoded)
{
  char *enc = (char *)env->GetStringUTFChars(encoded, 0);
  IndexDictionary(enc);
  env->ReleaseStringUTFChars(encoded, enc);
}

extern "C" JNIEXPORT jstring JNICALL
Java_org_odict_Dictionary_search(JNIEnv *env, jobject, jstring query, jstring encoded)
{
  char *q = (char *)env->GetStringUTFChars(query, 0);
  char *enc = (char *)env->GetStringUTFChars(encoded, 0);
  const char *result = SearchDictionary(q, enc);

  jstring res = env->NewStringUTF(result);

  env->ReleaseStringUTFChars(query, q);
  env->ReleaseStringUTFChars(encoded, enc);

  free((char *)result);

  return res;
}

extern "C" JNIEXPORT jstring JNICALL
Java_org_odict_Dictionary_read(JNIEnv *env, jobject, jstring path)
{
  const char *dictionary_path = env->GetStringUTFChars(path, 0);
  char *encoded = ReadDictionary((char *)dictionary_path);

  jstring res = env->NewStringUTF(encoded);

  env->ReleaseStringUTFChars(path, dictionary_path);

  free(encoded);

  return res;
}

extern "C" JNIEXPORT void JNICALL
Java_org_odict_Dictionary_write(JNIEnv *env, jobject, jstring xml, jstring path)
{
  const char *dictionary_path = env->GetStringUTFChars(path, 0);
  const char *content = env->GetStringUTFChars(xml, 0);

  WriteDictionary((char *)content, (char *)dictionary_path);

  env->ReleaseStringUTFChars(path, dictionary_path);
  env->ReleaseStringUTFChars(xml, content);
}

extern "C" JNIEXPORT void JNICALL
Java_org_odict_Dictionary_compile(JNIEnv *env, jobject, jstring path)
{
  const char *dictionary_path = env->GetStringUTFChars(path, 0);

  CompileDictionary((char *)dictionary_path);

  env->ReleaseStringUTFChars(path, dictionary_path);
}
