#include <jni.h>
#include <stdio.h>
#include <iostream>
#include "bridge/bridge.h"

using namespace std;

#ifndef CGO_EXPORT_BRIDGE_EXISTS
#error bridging header not found
#endif

extern "C" JNIEXPORT jstring JNICALL
Java_org_odict_Dictionary_lookupEntry(JNIEnv *env, jobject, jstring query, jstring path)
{
  const char *dictionary_path = env->GetStringUTFChars(path, 0);
  const char *entry_term = env->GetStringUTFChars(query, 0);
  return env->NewStringUTF(LookupEntry((char *)entry_term, (char *)dictionary_path));
}

extern "C" JNIEXPORT void JNICALL
Java_org_odict_Dictionary_index(JNIEnv *env, jobject, jbyteArray bytes)
{
  jsize len = env->GetArrayLength(bytes);

  char *buf = new char[len];

  cout << strlen(buf);
  env->GetByteArrayRegion(bytes, 0, len, reinterpret_cast<jbyte *>(buf));
  cout << len;
  cout << strlen(buf) << endl;
  cout << strln(jbyteArray) << endl;
  IndexDictionary(buf);

  // if (isCopy)
  // {
  //   env->ReleaseByteArrayElements(bytes, b, 0);
  // }
}

extern "C" JNIEXPORT jstring JNICALL
Java_org_odict_Dictionary_search(JNIEnv *env, jobject, jstring query, jbyteArray bytes)
{
  jboolean isCopy;
  jbyte *b = env->GetByteArrayElements(bytes, &isCopy);

  const char *q = env->GetStringUTFChars(query, 0);
  const char *result = SearchDictionary((char *)q, (char *)b);

  if (isCopy)
  {
    env->ReleaseByteArrayElements(bytes, b, 0);
  }

  return env->NewStringUTF(result);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_org_odict_Dictionary_read(JNIEnv *env, jobject, jstring path)
{
  const char *dictionary_path = env->GetStringUTFChars(path, 0);
  ReadDictionary_return vec = ReadDictionary((char *)dictionary_path);
  jbyteArray bytes = env->NewByteArray(vec.r0);

  env->SetByteArrayRegion(bytes, 0, vec.r0, reinterpret_cast<jbyte *>(vec.r1));

  return bytes;
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
