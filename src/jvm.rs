use jni_sys::{JavaVM, JavaVMInitArgs, JavaVMOption, jint, JNI_FALSE, JNI_VERSION_1_8, JNIEnv};
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

/*
fn java_option_string_to_jvm_option(java_option_string: &str) -> JavaVMOption {
    let java_option = JavaVMOption::default();

    // *mut c_char
    java_option.optionString = java_option_string.as;

    java_option
}
*/

impl Jvm {

    /// Instantiates the JVM and the JNI environment.
    ///
    /// # Arguments
    ///
    /// * `jvm_option_strings` - a list of JVM options.
    ///
    /// # Example
    ///
    /// ```
    /// use rucaja::Jvm;
    /// let jvm = Jvm::new(&["-Xcheck:jni"]);
    /// ```
    pub fn new(jvm_option_strings: &[&str]) -> Jvm {

        // Create the JVM structure.
        let mut jvm = Jvm {
            jvm: ptr::null_mut(),
            jni_environment: ptr::null_mut(),
        };

        // Wrap the JVM option string slices in a vector of `CString`s.
        let mut jvm_option_cstrings : Vec<CString> = Vec::new();

        for jvm_option_string in jvm_option_strings.iter() {
            jvm_option_cstrings.push(CString::new(*jvm_option_string).unwrap());
        }

        // Create a vector of `JavaVMOption` each referencing a `CString`.
        let mut jvm_options : Vec<JavaVMOption> = Vec::new();

        for jvm_option_cstring in jvm_option_cstrings.iter() {

            let mut jvm_option = JavaVMOption::default();
            jvm_option.optionString = jvm_option_cstring.as_ptr() as *mut i8;

            jvm_options.push(jvm_option);
        }

        // Create the JVM arguments.
        let mut jvm_arguments = JavaVMInitArgs::default();
        jvm_arguments.version = JNI_VERSION_1_8;
        jvm_arguments.options = jvm_options.as_mut_ptr();
        jvm_arguments.nOptions = jvm_options.len() as i32;
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

        JvmClass::new(self, jvm_class_ptr)
    }

    /// Tries to resolve the JVM constructor with the given signature in the given JVM class.
    #[inline(always)]
    pub fn get_constructor(&self, jvm_class: &JvmClass, jvm_method_signature: &str) -> Option<JvmMethod> {

        self.get_method(jvm_class, "<init>", jvm_method_signature)
    }

    /// Tries to resolve the JVM method with the given name and signature in the given JVM class.
    pub fn get_method(&self, jvm_class: &JvmClass, jvm_method_name: &str, jvm_method_signature: &str) -> Option<JvmMethod> {

        let jvm_method_name_cstring = CString::new(jvm_method_name).unwrap();
        let jvm_method_signature_cstring = CString::new(jvm_method_signature).unwrap();

        let jvm_method_ptr = unsafe {
            (**self.jni_environment).GetMethodID.unwrap()(
                self.jni_environment, *jvm_class.jvm_class_ptr(), jvm_method_name_cstring.as_ptr(),
                jvm_method_signature_cstring.as_ptr()
            )
        };

        JvmMethod::new(jvm_method_ptr)
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

        JvmMethod::new(jvm_method_ptr)
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
