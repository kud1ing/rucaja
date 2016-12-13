use jni_sys::{JavaVM, JavaVMInitArgs, jint, JNI_FALSE, JNI_VERSION_1_8, JNIEnv};
use jvm_class::JvmClass;
use jvm_method::JvmMethod;
use std::ffi::CString;
use std::ptr;

///
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
    pub fn call_static_void_method(&self, jvm_class: &JvmClass, jvm_method: &JvmMethod) {
        unsafe {
            (**self.jni_environment).CallStaticVoidMethod.unwrap()(
                self.jni_environment, *jvm_class.jvm_class_ptr(), *jvm_method.jvm_method_ptr()
            );

            // An exception occured.
            if !(**self.jni_environment).ExceptionOccurred.unwrap()(self.jni_environment).is_null() {
                panic!("An exception occured");
            };
        }
    }

    ///
    pub fn get_class(&self, jvm_class_name: &str) -> Option<JvmClass> {

        let jvm_class_name_cstring = CString::new(jvm_class_name).unwrap();

        let jvm_class_ptr = unsafe {
            (**self.jni_environment).FindClass.unwrap()(
                self.jni_environment, jvm_class_name_cstring.as_ptr()
            )
        };

        JvmClass::maybe_new(&self, jvm_class_ptr)
    }

    ///
    pub fn get_static_method(&self, jvm_class: &JvmClass, jvm_method_name: &str, jvm_method_signature: &str) -> Option<JvmMethod> {

        let jvm_method_name_cstring = CString::new(jvm_method_name).unwrap();
        let jvm_method_signature_cstring = CString::new(jvm_method_signature).unwrap();

        let jvm_method_ptr = unsafe {
            (**self.jni_environment).GetStaticMethodID.unwrap()(
                self.jni_environment, *jvm_class.jvm_class_ptr(), jvm_method_name_cstring.as_ptr(),
                jvm_method_signature_cstring.as_ptr()
            )
        };

        JvmMethod::maybe_new(jvm_method_ptr)
    }

    ///
    pub fn jni_environment(&self) -> *mut JNIEnv {
        self.jni_environment
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
