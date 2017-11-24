use jni_sys::{
    JavaVM, JavaVMInitArgs, JavaVMOption, JNI_CreateJavaVM, JNI_ERR, JNI_EDETACHED, JNI_EVERSION,
    JNI_ENOMEM, JNI_EEXIST, JNI_EINVAL, JNI_FALSE, JNI_OK, JNI_VERSION_1_8, JNIEnv, jboolean, jbyte,
    jchar, jint, jdouble, jfloat, jlong, jobject, jshort, jvalue
};
use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;

// =================================================================================================

#[link(name="jvm")]
extern {
}

// =================================================================================================

/// Wraps a `jboolean` in a `jvalue`.
pub fn jvalue_from_jboolean(arg: jboolean) -> jvalue {
    jvalue { z: arg }
}

/// Wraps a `jbyte` in a `jvalue`.
pub fn jvalue_from_jbyte(arg: jbyte) -> jvalue {
    jvalue { b: arg }
}

/// Wraps a `jchar` in a `jvalue`.
pub fn jvalue_from_jchar(arg: jchar) -> jvalue {
    jvalue { c: arg }
}

/// Wraps a `jdouble` in a `jvalue`.
pub fn jvalue_from_jdouble(arg: jdouble) -> jvalue {
    jvalue { d: arg }
}

/// Wraps a `jint` in a `jvalue`.
pub fn jvalue_from_jint(arg: jint) -> jvalue {
    jvalue { i: arg }
}

/// Wraps a `jfloat` in a `jvalue`.
pub fn jvalue_from_jfloat(arg: jfloat) -> jvalue {
    jvalue { f: arg }
}

/// Wraps a `jlong` in a `jvalue`.
pub fn jvalue_from_jlong(arg: jlong) -> jvalue {
    jvalue { j: arg }
}

/// Wraps a `jobject` in a `jvalue`.
pub fn jvalue_from_jobject(arg: jobject) -> jvalue {
    jvalue { l: arg }
}

/// Wraps a `jshort` in a `jvalue`.
pub fn jvalue_from_jshort(arg: jshort) -> jvalue {
    jvalue { s: arg }
}

///
pub fn jvm_exception_occured(jni_environment: *mut JNIEnv) -> bool {
    return unsafe {
        !(**jni_environment).ExceptionOccurred.unwrap()(jni_environment).is_null()
    }
}

///
pub fn print_and_panic_on_jvm_exception(jni_environment: *mut JNIEnv) {

    // A JVM exception occurred.
    if jvm_exception_occured(jni_environment) {

        unsafe {
            // Print the JVM exception.
            (**jni_environment).ExceptionDescribe.unwrap()(jni_environment);
        }

        // Panic.
        panic!("An exception occurred");
    };
}

///
pub fn print_jvm_exception(jni_environment: *mut JNIEnv) {

    // A JVM exception occurred.
    if jvm_exception_occured(jni_environment) {

        unsafe {
            // Print the JVM exception.
            (**jni_environment).ExceptionDescribe.unwrap()(jni_environment);
        }
    };
}

// =================================================================================================

/// Holds a reference to the embedded JVM.
pub struct Jvm {

    /// The JVM.
    jvm: *mut JavaVM,
}

impl Jvm {

    ///
    pub fn jvm(&self) -> *mut JavaVM {
        self.jvm
    }

    /// Tries to instantiate the embedded JVM.
    ///
    /// The JNI does not allow the creation of multiple JVMs in the same process (it seems, not even
    /// sequentially). An attempt will result in a `panic`.
    ///
    /// # Arguments
    ///
    /// * `jvm_option_strings` - a list of JVM option strings.
    ///
    /// # Example
    ///
    /// ```
    /// use rucaja::Jvm;
    /// {
    ///   Jvm::new(&["-Xcheck:jni"]);
    /// }
    /// ```
    pub fn new(jvm_option_strings: &[&str]) -> Jvm {
        // Wrap the JVM option string slices in a vector of `CString`s.
        let mut jvm_option_cstrings: Vec<CString> = Vec::new();

        for jvm_option_string in jvm_option_strings {
            jvm_option_cstrings.push(CString::new(*jvm_option_string).unwrap());
        }

        // Create a vector of `JavaVMOption`s, each referencing a `CString`.
        let mut jvm_options: Vec<JavaVMOption> = Vec::new();

        for jvm_option_cstring in &jvm_option_cstrings {
            let jvm_option = JavaVMOption {
                optionString: jvm_option_cstring.as_ptr() as *mut i8,
                extraInfo: ptr::null_mut() as *mut c_void
            };

            jvm_options.push(jvm_option);
        }

        // Create the JVM arguments.
        let mut jvm_arguments = JavaVMInitArgs {
            version: JNI_VERSION_1_8,
            options: jvm_options.as_mut_ptr(),
            nOptions: jvm_options.len() as i32,
            ignoreUnrecognized: JNI_FALSE
        };

        // Initialize space for a pointer to the JNI environment.
        let mut jvm: *mut JavaVM = ptr::null_mut();
        let mut jni_environment: *mut JNIEnv = ptr::null_mut();

        // Try to instantiate the JVM.
        let result = unsafe {
            JNI_CreateJavaVM(
                &mut jvm,
                (&mut jni_environment as *mut *mut JNIEnv) as *mut *mut c_void,
                (&mut jvm_arguments as *mut JavaVMInitArgs) as *mut c_void
            )
        };

        // There was an error while trying to instantiate the JVM.
        if result != JNI_OK {

            // Translate the error code to a message.
            let error_message = match result {
                JNI_EDETACHED => "thread detached from JVM",
                JNI_EEXIST => "JVM exists already",
                JNI_EINVAL => "invalid arguments",
                JNI_ENOMEM => "not enough memory",
                JNI_ERR => "unknown error",
                JNI_EVERSION => "JNI version error",
                _ => "unknown JNI error value",
            };

            panic!("`JNI_CreateJavaVM()` signaled an error: {}", error_message);
        }

        Jvm { jvm }
    }
}

// =================================================================================================

impl Drop for Jvm {

    fn drop(&mut self) {

        // The Java 7 documentation states that VM unloading is not supported.
        // The Java 8 documentation does not mention this restriction anymore. Calling
        // `DestroyJavaVM()` led to `SIGSEV`s with Java 8, though.
    }
}

// =================================================================================================
