extern crate jni_sys;

use jni_sys::{JavaVM, JavaVMInitArgs, JavaVMOption, jint, JNI_FALSE, JNI_VERSION_1_8};
// use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;


#[link(name="jvm")]
extern {
    fn JNI_CreateJavaVM(pvm: *mut *mut JavaVM, penv: *mut *mut c_void, args: *mut JavaVMInitArgs) -> jint;
}

fn main() {

    //let mut jvm_options = [JavaVMOption::default()];
    // jvm_options[0].optionString = CString::new("-Djava.class.path=/usr/lib/java").unwrap().into_raw();

    let mut jvm_arguments = JavaVMInitArgs::default();
    jvm_arguments.version = JNI_VERSION_1_8;
    jvm_arguments.nOptions = 0;
    //jvm_arguments.options = jvm_options.as_mut_ptr();
    jvm_arguments.ignoreUnrecognized = JNI_FALSE;

    let mut jvm = ptr::null_mut();
    let mut env = ptr::null_mut();

    unsafe {
        let _ = JNI_CreateJavaVM(&mut jvm, &mut env as *mut _, &mut jvm_arguments as *mut _);
    }

    // TODO
    /*

    jclass java_class = env->FindClass("Main");
    jmethodID java_method_id = env->GetStaticMethodID(java_class, "test", "(I)V");
    env->CallStaticVoidMethod(java_class, java_method_id, 100);

    jvm->DestroyJavaVM();
    */
}
