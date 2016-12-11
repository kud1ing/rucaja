extern crate jni_sys;

use jni_sys::{JavaVMInitArgs, JavaVMOption, JNI_FALSE, JNI_VERSION_1_8};
use std::ffi::CString;

fn main() {

    let mut jvm_options = [JavaVMOption::default()];
    jvm_options[0].optionString = CString::new("-Djava.class.path=/usr/lib/java").unwrap().into_raw();

    let mut jvm_arguments = JavaVMInitArgs::default();
    jvm_arguments.version = JNI_VERSION_1_8;
    jvm_arguments.nOptions = 1;
    jvm_arguments.options = jvm_options.as_mut_ptr();
    jvm_arguments.ignoreUnrecognized = JNI_FALSE;

    // TODO
    /*
    JavaVM *jvm;
    JNIEnv *env;
    JNI_CreateJavaVM(&jvm, (void**)&env, &jvm_arguments);
    delete jvm_options;

    jclass java_class = env->FindClass("Main");
    jmethodID java_method_id = env->GetStaticMethodID(java_class, "test", "(I)V");
    env->CallStaticVoidMethod(java_class, java_method_id, 100);

    jvm->DestroyJavaVM();
    */
}
