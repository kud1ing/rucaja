use jni_sys::{JavaVM, JavaVMInitArgs, jint, JNI_FALSE, JNI_VERSION_1_8, JNIEnv};
use jvm_class::JvmClass;
use jvm_method::JvmMethod;
use std::ffi::CString;
use std::ptr;
use std::os::raw::c_void;


/// Represents the JVM and the JNI environment.
pub struct Jvm {

    /// The JVM.
    jvm: *mut JavaVM,

    /// The JNI environment.
    jni_environment: *mut JNIEnv,
}

impl Jvm {

    /// Instantiates the JVM and the JNI environment.
    pub fn new() -> Jvm {

        let mut jvm = Jvm {
            jvm: ptr::null_mut(),
            jni_environment: ptr::null_mut(),
        };

        // Create the JVM arguments.
        let mut jvm_arguments = JavaVMInitArgs::default();
        jvm_arguments.version = JNI_VERSION_1_8;
        jvm_arguments.nOptions = 0;
        jvm_arguments.ignoreUnrecognized = JNI_FALSE;

        // Create the JVM.
        unsafe {
            let _ = JNI_CreateJavaVM(
                &mut jvm.jvm,
                (&mut jvm.jni_environment as *mut *mut JNIEnv) as *mut *mut c_void,
                (&mut jvm_arguments as *mut JavaVMInitArgs) as *mut c_void
            );
        }

        jvm
    }

    /// Tries to call the given JVM method in the given JVM class.
    /// Currently panics if a JVM exception occurs.
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

    /// Tries to resolve the JVM class with the given name.
    pub fn get_class(&self, jvm_class_name: &str) -> Option<JvmClass> {

        let jvm_class_name_cstring = CString::new(jvm_class_name).unwrap();

        let jvm_class_ptr = unsafe {
            (**self.jni_environment).FindClass.unwrap()(
                self.jni_environment, jvm_class_name_cstring.as_ptr()
            )
        };

        JvmClass::maybe_new(&self, jvm_class_ptr)
    }

    /// Tries to resolve the static JVM method with the given name and signature in the given JVM class.
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

    /// Returns the JNI environment.
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
    // TODO: use `JNI_CreateJavaVM()` from rust-jni-sys > 0.2.1:
    fn JNI_CreateJavaVM(pvm: *mut *mut JavaVM, penv: *mut *mut c_void, args: *mut c_void) -> jint;
}
