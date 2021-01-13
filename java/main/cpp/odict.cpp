#include <jni.h>
#include <stdio.h>

#include "bridge/bridge.h"

#ifndef CGO_EXPORT_BRIDGE_EXISTS
#error bridging header not found
#endif

extern "C" JNIEXPORT jstring JNICALL
Java_org_odict_ODict_lookupEntry(JNIEnv *env, jobject, jstring query, jstring path)
{
  const char *dictionary_path = env->GetStringUTFChars(path, 0);
  const char *entry_term = env->GetStringUTFChars(query, 0);
  return env->NewStringUTF(LookupEntry((char *)dictionary_path, (char *)entry_term));
}

extern "C" JNIEXPORT void JNICALL
Java_org_odict_ODict_indexDictionary(JNIEnv *env, jobject, jstring path)
{
  const char *dictionary_path = env->GetStringUTFChars(path, 0);
  IndexDictionary((char *)dictionary_path);
}

extern "C" JNIEXPORT jstring JNICALL
Java_org_odict_ODict_searchDictionary(JNIEnv *env, jobject, jstring query, jstring path)
{
  const char *dictionary_path = env->GetStringUTFChars(path, 0);
  const char *q = env->GetStringUTFChars(query, 0);
  return env->NewStringUTF(SearchDictionary((char *)q, (char *)dictionary_path));
}

extern "C" JNIEXPORT void JNICALL
Java_org_odict_ODict_writeDictionary(JNIEnv *env, jobject, jstring xml, jstring path)
{
  const char *dictionary_path = env->GetStringUTFChars(path, 0);
  const char *content = env->GetStringUTFChars(xml, 0);
  WriteDictionary((char *)content, (char *)dictionary_path);
}

extern "C" JNIEXPORT void JNICALL
Java_org_odict_ODict_compileDictionary(JNIEnv *env, jobject, jstring path)
{
  const char *dictionary_path = env->GetStringUTFChars(path, 0);
  CompileDictionary((char *)dictionary_path);
}
