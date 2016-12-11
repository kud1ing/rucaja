extern crate jni_sys;

use jni_sys::{JavaVM, JavaVMInitArgs, jclass, jint, jmethodID, JNI_FALSE, JNI_VERSION_1_8, JNIEnv};
use std::ffi::CString;
use std::ptr;


struct Jvm {
    jvm: *mut JavaVM,
    jni_environment: *mut JNIEnv,
}

impl Jvm {

    ///
    fn new() -> Jvm {

        let mut jvm = Jvm {
            jvm: ptr::null_mut(),
            jni_environment: ptr::null_mut(),
        };

        let mut jvm_arguments = JavaVMInitArgs::default();
        jvm_arguments.version = JNI_VERSION_1_8;
        jvm_arguments.nOptions = 0;
        jvm_arguments.ignoreUnrecognized = JNI_FALSE;

        unsafe {
            let _ = JNI_CreateJavaVM(
                &mut jvm.jvm, &mut jvm.jni_environment as *mut _, &mut jvm_arguments as *mut _
            );
        }

        jvm
    }
}


#[link(name="jvm")]
extern {
    //fn JNI_CreateJavaVM(pvm: *mut *mut JavaVM, penv: *mut *mut c_void, args: *mut c_void) -> jint;
    fn JNI_CreateJavaVM(pvm: *mut *mut JavaVM, penv: *mut *mut JNIEnv, args: *mut JavaVMInitArgs) -> jint;
}

fn main() {

    let java_class_name : CString = CString::new("Test").unwrap();
    let java_method_name : CString = CString::new("helloRust").unwrap();
    let java_method_signature : CString = CString::new("()V").unwrap();

    let jvm = Jvm::new();

    unsafe {
        // Get the Java class.
        let java_class : jclass = (**jvm.jni_environment).FindClass.unwrap()(
            jvm.jni_environment, java_class_name.as_ptr()
        );

        // Get the Java method ID.
        let java_method_id : jmethodID = (**jvm.jni_environment).GetStaticMethodID.unwrap()(
            jvm.jni_environment, java_class, java_method_name.as_ptr(), java_method_signature.as_ptr()
        );

        // Call the Java static void method.
        (**jvm.jni_environment).CallStaticVoidMethod.unwrap()(
            jvm.jni_environment, java_class, java_method_id
        );

        // Destroy the JVM.
        (**jvm.jvm).DestroyJavaVM.unwrap()(jvm.jvm);
    }

}
