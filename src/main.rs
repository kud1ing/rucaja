extern crate jni_sys;

use jni_sys::{JavaVM, JavaVMInitArgs, JavaVMOption, jclass, jint, JNI_FALSE, JNI_VERSION_1_8, JNIEnv, JNINativeInterface_};
// use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::ptr;


#[link(name="jvm")]
extern {
    fn JNI_CreateJavaVM(pvm: *mut *mut JavaVM, penv: *mut *mut JNIEnv, args: *mut JavaVMInitArgs) -> jint;
}

fn main() {

    //let mut jvm_options = [JavaVMOption::default()];
    // jvm_options[0].optionString = CString::new("-Djava.class.path=/usr/lib/java").unwrap().into_raw();

    let mut jvm_arguments = JavaVMInitArgs::default();
    jvm_arguments.version = JNI_VERSION_1_8;
    jvm_arguments.nOptions = 0;
    //jvm_arguments.options = jvm_options.as_mut_ptr();
    jvm_arguments.ignoreUnrecognized = JNI_FALSE;

    let mut jvm : *mut JavaVM = ptr::null_mut();
    let mut env : *mut JNIEnv = ptr::null_mut();

    unsafe {
        let _ = JNI_CreateJavaVM(&mut jvm, &mut env as *mut _, &mut jvm_arguments as *mut _);
    }

    // TODO: make this compile
    let java_class = (*env).FindClass.unwrap()("Test");

    // TODO
    /*

    jclass java_class = env->FindClass("Test");
    jmethodID java_method_id = env->GetStaticMethodID(java_class, "helloRust", "()V");
    env->CallStaticVoidMethod(java_class, java_method_id, 100);

    jvm->DestroyJavaVM();
    */
}
