extern crate jni_sys;

use jni_sys::{JavaVM, JavaVMInitArgs, JavaVMOption, jclass, jint, jmethodID, JNI_FALSE, JNI_VERSION_1_8, JNIEnv, JNINativeInterface_};
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::ptr;
use std::str;


#[link(name="jvm")]
extern {
    fn JNI_CreateJavaVM(pvm: *mut *mut JavaVM, penv: *mut *mut JNIEnv, args: *mut JavaVMInitArgs) -> jint;
}

fn main() {

    let java_class_name : CString = CString::new("Test").unwrap();
    let java_method_name : CString = CString::new("helloRust").unwrap();
    let java_method_signature : CString = CString::new("()V").unwrap();

    let mut jvm_arguments = JavaVMInitArgs::default();
    jvm_arguments.version = JNI_VERSION_1_8;
    jvm_arguments.nOptions = 0;
    jvm_arguments.ignoreUnrecognized = JNI_FALSE;

    let mut jvm : *mut JavaVM = ptr::null_mut();
    let mut env : *mut JNIEnv = ptr::null_mut();

    unsafe {
        let result = JNI_CreateJavaVM(&mut jvm, &mut env as *mut _, &mut jvm_arguments as *mut _);

        // Get the Java class.
        let java_class : jclass = (**env).FindClass.unwrap()(env, java_class_name.as_ptr());

        // Get the Java method ID.
        let java_method_id : jmethodID = (**env).GetStaticMethodID.unwrap()(
            env, java_class, java_method_name.as_ptr(), java_method_signature.as_ptr()
        );

        // Call the Java method.
        (**env).CallStaticVoidMethod.unwrap()(env, java_class, java_method_id);

        // TODO
        //jvm->DestroyJavaVM();
    }

}
