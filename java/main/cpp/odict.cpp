#include <jni.h>
#include <stdio.h>
// #include "odict.h"

extern "C" JNIEXPORT jint JNICALL Java_org_odict_Jni_foo(JNIEnv *, jobject) {
   return 42;
}
