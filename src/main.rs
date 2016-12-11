extern crate jni_sys;

use jni_sys::{JavaVM, JavaVMInitArgs, JavaVMOption, jclass, jint, jmethodID, JNI_FALSE, JNI_VERSION_1_8, JNIEnv, JNINativeInterface_};
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::ptr;


#[link(name="jvm")]
extern {
    fn JNI_CreateJavaVM(pvm: *mut *mut JavaVM, penv: *mut *mut JNIEnv, args: *mut JavaVMInitArgs) -> jint;
}

fn main() {

    let java_class_name : *const c_char = CString::new("Test").unwrap().as_ptr();
    let java_method_name : *const c_char = CString::new("helloRust").unwrap().as_ptr();
    let java_method_signature : *const c_char = CString::new("()V").unwrap().as_ptr();

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

        println!("java_class_name {:?}", java_class_name);

        let java_class : jclass = (**env).FindClass.unwrap()(env, java_class_name);

        println!("java_class {:?}", java_class);

        //let java_method_id : jmethodID = (**env).GetStaticMethodID.unwrap()(
        //    env, java_class, java_method_name, java_method_signature
        //);

        //println!("java_method_id {:?}", java_method_id);

        // TODO: This crashes.
        //(**env).CallStaticVoidMethod.unwrap()(env, java_class, java_method_id);

        // TODO
        //jvm->DestroyJavaVM();
    }

}
