extern crate jni_sys;

use jni_sys::{JavaVM, JavaVMInitArgs, jclass, jint, jmethodID, JNI_FALSE, JNI_VERSION_1_8, JNIEnv};
use std::ffi::CString;
use std::ptr;


pub struct Jvm {
    jvm: *mut JavaVM,
    jni_environment: *mut JNIEnv,
}

impl Jvm {

    ///
    pub fn new() -> Jvm {

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

    ///
    pub fn call_static_void_method(&self, java_class: &jclass, java_method_id: &jmethodID) {
        unsafe {
            (**self.jni_environment).CallStaticVoidMethod.unwrap()(
                self.jni_environment, *java_class, *java_method_id
            );
        }
    }

    ///
    pub fn find_class(&self, java_class_name: &str) -> jclass {

        let java_class_name_cstring = CString::new(java_class_name).unwrap();

        unsafe {
            (**self.jni_environment).FindClass.unwrap()(
                self.jni_environment, java_class_name_cstring.as_ptr()
            )
        }
    }

    ///
    pub fn get_static_method_id(&self, java_class: &jclass, java_method_name: &str, java_method_signature: &str) -> jmethodID {

        let java_method_name_cstring = CString::new(java_method_name).unwrap();
        let java_method_signature_cstring = CString::new(java_method_signature).unwrap();

        unsafe {
            (**self.jni_environment).GetStaticMethodID.unwrap()(
                self.jni_environment, *java_class, java_method_name_cstring.as_ptr(),
                java_method_signature_cstring.as_ptr()
            )
        }
    }
}


impl Drop for Jvm {

    fn drop(&mut self) {

        unsafe {
            // Destroy the JVM.
            (**self.jvm).DestroyJavaVM.unwrap()(self.jvm);
        }
    }
}


#[link(name="jvm")]
extern {
    // TODO: use this signature: first cast to *mut *mut *const jni_sys::JNINativeInterface_ and then to *mut *mut c_void.
    // TODO: use `JNI_CreateJavaVM()` from rust-jni-sys > 0.2.1:
    //fn JNI_CreateJavaVM(pvm: *mut *mut JavaVM, penv: *mut *mut c_void, args: *mut c_void) -> jint;
    fn JNI_CreateJavaVM(pvm: *mut *mut JavaVM, penv: *mut *mut JNIEnv, args: *mut JavaVMInitArgs) -> jint;
}
