#include <jni.h>
#include <stdio.h>
#include "Main.h"

JNIEXPORT jint JNICALL main_java_org_odict_Jni_foo(JNIEnv *, jobject) {
   return 42;
}
