#include <jni.h>
#include <stdio.h>
#include <iostream>
#include "bridge/archive.h"

using namespace std;

#ifndef CGO_EXPORT_BRIDGE_EXISTS
#error bridging header not found
#endif

extern "C" JNIEXPORT jstring JNICALL
Java_org_odict_Dictionary_lookup(JNIEnv *env, jobject, jstring query, jstring dictID)
{
  const char *dictionaryID = env->GetStringUTFChars(dictID, 0);
  const char *entry_term = env->GetStringUTFChars(query, 0);
  char *result = LookupEntry((char *)entry_term, (char *)dictionaryID);
  jstring str = env->NewStringUTF(result);

  free(result);

  return str;
}

extern "C" JNIEXPORT void JNICALL
Java_org_odict_Dictionary_index(JNIEnv *env, jobject, jstring path, jboolean force)
{
  char *p = (char *)env->GetStringUTFChars(path, 0);
  
  IndexDictionary(p, force);

  env->ReleaseStringUTFChars(path, p);
}

extern "C" JNIEXPORT jstring JNICALL
Java_org_odict_Dictionary_search(JNIEnv *env, jobject, jstring query, jstring dictionary_id)
{
  char *q = (char *)env->GetStringUTFChars(query, 0);
  char *dictid = (char *)env->GetStringUTFChars(dictionary_id, 0);
  const char *result = SearchDictionary(q, dictid);

  jstring res = env->NewStringUTF(result);

  env->ReleaseStringUTFChars(query, q);
  env->ReleaseStringUTFChars(dictionary_id, dictid);

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
