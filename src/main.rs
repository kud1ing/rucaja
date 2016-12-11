extern crate jni_sys;

use jni_sys::{JavaVM, JavaVMInitArgs, JavaVMOption, JNI_FALSE, JNI_VERSION_1_8, JNIInvokeInterface_};
use std::ffi::CString;
use std::os::raw::c_void;


#[link(name="jvm")]
extern {
    fn JNI_CreateJavaVM(pvm: *mut *mut JavaVM, penv: *mut *mut c_void, args: *mut c_void);
}

fn main() {

    let mut jvm_options = [JavaVMOption::default()];
    // jvm_options[0].optionString = CString::new("-Djava.class.path=/usr/lib/java").unwrap().into_raw();

    let mut jvm_arguments = [JavaVMInitArgs::default()];
    jvm_arguments[0].version = JNI_VERSION_1_8;
    jvm_arguments[0].nOptions = 1;
    jvm_arguments[0].options = jvm_options.as_mut_ptr();
    jvm_arguments[0].ignoreUnrecognized = JNI_FALSE;

    let mut jvm = [JNIInvokeInterface_::default()];
    let mut env = [JNIInvokeInterface_::default()];

    // TODO: make this compile
    // JNI_CreateJavaVM(jvm.as_mut_ptr(), env.as_mut_ptr(), jvm_arguments.as_mut_ptr() as *mut c_void);

    // TODO
    /*
    JNI_CreateJavaVM(&jvm, (void**)&env, &jvm_arguments);

    jclass java_class = env->FindClass("Main");
    jmethodID java_method_id = env->GetStaticMethodID(java_class, "test", "(I)V");
    env->CallStaticVoidMethod(java_class, java_method_id, 100);

    jvm->DestroyJavaVM();
    */
}
