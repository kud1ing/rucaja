// This is what main.rs should replicate.
//
// Compile it with:
//
//   clang++ -I /Library/Java/JavaVirtualMachines/jdk1.8.0_51.jdk/Contents/Home/include -I /Library/Java/JavaVirtualMachines/jdk1.8.0_51.jdk//Contents/Home/include/darwin -ljvm -L /Library/Java/JavaVirtualMachines/jdk1.8.0_51.jdk/Contents/Home/jre/lib/server test.cpp

#include "jni.h"
#include <iostream>

int main() {

    //JavaVMOption* options = new JavaVMOption[1];
    //options[0].optionString = "-Djava.class.path=/usr/lib/java";

    JavaVMInitArgs vm_args;
    vm_args.version = JNI_VERSION_1_8;
    vm_args.nOptions = 0;
    //vm_args.options = options;
    vm_args.ignoreUnrecognized = false;

    JavaVM* jvm;
    JNIEnv* env;

    JNI_CreateJavaVM(&jvm, (void**)&env, &vm_args);

    //delete[] options;

    jclass java_class = env->FindClass("Test");
    jmethodID java_method_id = env->GetStaticMethodID(java_class, "helloRust", "()V");

    std::cout << "java_class " << java_class << std::endl;
    std::cout << "java_method_id " << java_method_id << std::endl;

    env->CallStaticVoidMethod(java_class, java_method_id);

    // TODO
    //jvm->DestroyJavaVM();
}