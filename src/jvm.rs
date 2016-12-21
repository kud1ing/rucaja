use jni_sys::{
    JavaVM, JavaVMInitArgs, JavaVMOption, JNI_FALSE, JNI_VERSION_1_8, JNIEnv,
    jboolean, jbyte, jchar, jint, jdouble, jfloat, jlong, jobject, jshort, jvalue
};
use jvm_class::JvmClass;
use jvm_method::JvmMethod;
use std::ffi::CString;
use std::ptr;
use std::os::raw::c_void;


/// Wraps a `jboolean` in a `jvalue`.
pub unsafe fn jvalue_from_jboolean(arg: jboolean) -> jvalue {

    let mut jvalue = jvalue::default();
    *jvalue.z() = arg;

    jvalue
}

/// Wraps a `jbyte` in a `jvalue`.
pub unsafe fn jvalue_from_jbyte(arg: jbyte) -> jvalue {

    let mut jvalue = jvalue::default();
    *jvalue.b() = arg;

    jvalue
}

/// Wraps a `jchar` in a `jvalue`.
pub unsafe fn jvalue_from_jchar(arg: jchar) -> jvalue {

    let mut jvalue = jvalue::default();
    *jvalue.c() = arg;

    jvalue
}

/// Wraps a `jdouble` in a `jvalue`.
pub unsafe fn jvalue_from_jdouble(arg: jdouble) -> jvalue {

    let mut jvalue = jvalue::default();
    *jvalue.d() = arg;

    jvalue
}

/// Wraps a `jint` in a `jvalue`.
pub unsafe fn jvalue_from_jint(arg: jint) -> jvalue {

    let mut jvalue = jvalue::default();
    *jvalue.i() = arg;

    jvalue
}

/// Wraps a `jfloat` in a `jvalue`.
pub unsafe fn jvalue_from_jfloat(arg: jfloat) -> jvalue {

    let mut jvalue = jvalue::default();
    *jvalue.f() = arg;

    jvalue
}

/// Wraps a `jlong` in a `jvalue`.
pub unsafe fn jvalue_from_jlong(arg: jlong) -> jvalue {

    let mut jvalue = jvalue::default();
    *jvalue.j() = arg;

    jvalue
}

/// Wraps a `jobject` in a `jvalue`.
pub unsafe fn jvalue_from_jobject(arg: jobject) -> jvalue {

    let mut jvalue = jvalue::default();
    *jvalue.l() = arg;

    jvalue
}

/// Wraps a `jshort` in a `jvalue`.
pub unsafe fn jvalue_from_jshort(arg: jshort) -> jvalue {

    let mut jvalue = jvalue::default();
    *jvalue.s() = arg;

    jvalue
}


/// Represents the JVM and the JNI environment.
pub struct Jvm {

    /// The JVM.
    jvm: *mut JavaVM,

    /// The JNI environment.
    jni_environment: *mut JNIEnv,
}


///
unsafe fn print_and_panic_on_jvm_exception(jni_environment: *mut JNIEnv) {

    // An exception occurred.
    if !(**jni_environment).ExceptionOccurred.unwrap()(jni_environment).is_null() {

        // Print the JVM exception.
        (**jni_environment).ExceptionDescribe.unwrap()(jni_environment);

        panic!("An exception occurred");
    };
}

///
unsafe fn print_jvm_exception(jni_environment: *mut JNIEnv) {

    // An exception occurred.
    if !(**jni_environment).ExceptionOccurred.unwrap()(jni_environment).is_null() {

        // Print the JVM exception.
        (**jni_environment).ExceptionDescribe.unwrap()(jni_environment);
    };
}


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
    /// unsafe {
    ///   Jvm::new(&["-Xcheck:jni"]);
    /// }
    /// ```
    pub unsafe fn new(jvm_option_strings: &[&str]) -> Jvm {

        // Create the JVM structure.
        let mut jvm = Jvm {
            jvm: ptr::null_mut(),
            jni_environment: ptr::null_mut(),
        };

        // Wrap the JVM option string slices in a vector of `CString`s.
        let mut jvm_option_cstrings : Vec<CString> = Vec::new();

        for jvm_option_string in jvm_option_strings {
            jvm_option_cstrings.push(CString::new(*jvm_option_string).unwrap());
        }

        // Create a vector of `JavaVMOption` each referencing a `CString`.
        let mut jvm_options : Vec<JavaVMOption> = Vec::new();

        for jvm_option_cstring in &jvm_option_cstrings {

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
        let _ = JNI_CreateJavaVM(
            &mut jvm.jvm,
            (&mut jvm.jni_environment as *mut *mut JNIEnv) as *mut *mut c_void,
            (&mut jvm_arguments as *mut JavaVMInitArgs) as *mut c_void
        );

        jvm
    }

    // TODO: call_boolean_method()

    // TODO: call_byte_method()

    // TODO: call_char_method()

    // TODO: call_double_method()

    // TODO: call_float_method()

    // TODO: call_int_method()

    // TODO: call_long_method()

    // TODO: call_object_method()

    // TODO: call_short_method()

    // TODO: call_void_method()


    // TODO: call_nonvirtual_boolean_method()

    // TODO: call_nonvirtual_byte_method()

    // TODO: call_nonvirtual_char_method()

    // TODO: call_nonvirtual_double_method()

    // TODO: call_nonvirtual_float_method()

    // TODO: call_nonvirtual_int_method()

    // TODO: call_nonvirtual_long_method()

    // TODO: call_nonvirtual_object_method()

    // TODO: call_nonvirtual_short_method()

    // TODO: call_nonvirtual_void_method()



    // TODO: call_static_boolean_method()

    /// Tries to call the given JVM static boolean method in the given JVM class.
    /// Currently panics if a JVM exception occurs.
    pub unsafe fn call_static_boolean_method(
        &self, jvm_class: &JvmClass, jvm_method: &JvmMethod, args: *const jvalue
    ) -> jboolean {

        let result = (**self.jni_environment).CallStaticBooleanMethodA.unwrap()(
            self.jni_environment, *jvm_class.jvm_class_ptr(), *jvm_method.jvm_method_ptr(), args
        );

        print_and_panic_on_jvm_exception(self.jni_environment);

        result
    }

    // TODO: call_static_byte_method()

    // TODO: call_static_char_method()

    // TODO: call_static_double_method()

    // TODO: call_static_float_method()

    // TODO: call_static_int_method()

    // TODO: call_static_long_method()

    // TODO: call_static_object_method()
    pub unsafe fn call_static_object_method(
        &self, jvm_class: &JvmClass, jvm_method: &JvmMethod, args: *const jvalue
    ) -> jobject {

        let result = (**self.jni_environment).CallStaticObjectMethodA.unwrap()(
            self.jni_environment, *jvm_class.jvm_class_ptr(), *jvm_method.jvm_method_ptr(), args
        );

        print_and_panic_on_jvm_exception(self.jni_environment);

        result
    }

    /// Tries to call the given JVM static void method in the given JVM class.
    /// Currently panics if a JVM exception occurs.
    pub unsafe fn call_static_void_method(
        &self, jvm_class: &JvmClass, jvm_method: &JvmMethod, args: *const jvalue
    ) {
        (**self.jni_environment).CallStaticVoidMethodA.unwrap()(
            self.jni_environment, *jvm_class.jvm_class_ptr(), *jvm_method.jvm_method_ptr(), args
        );

        print_and_panic_on_jvm_exception(self.jni_environment);

    }

    /// Tries to resolve the JVM class with the given name.
    pub unsafe fn get_class(&self, jvm_class_name: &str) -> Option<JvmClass> {

        let jvm_class_name_cstring = CString::new(jvm_class_name).unwrap();

        let jvm_class_ptr =
            (**self.jni_environment).FindClass.unwrap()(
                self.jni_environment, jvm_class_name_cstring.as_ptr()
            );

        // An exception occurred, probably a `java.lang.NoClassDefFoundError`.
        if !(**self.jni_environment).ExceptionOccurred.unwrap()(self.jni_environment).is_null() {

            // Print any JVM exception.
            print_jvm_exception(self.jni_environment);
        };

        if jvm_class_ptr.is_null() {
            return None;
        }

        JvmClass::new(self, jvm_class_ptr)
    }

    /// Tries to resolve the JVM constructor with the given signature in the given JVM class.
    pub unsafe fn get_constructor(&self, jvm_class: &JvmClass, jvm_method_signature: &str) -> Option<JvmMethod> {

        self.get_method(jvm_class, "<init>", jvm_method_signature)
    }

    /// Tries to resolve the JVM method with the given name and signature in the given JVM class.
    pub unsafe fn get_method(
        &self, jvm_class: &JvmClass, jvm_method_name: &str, jvm_method_signature: &str
    ) -> Option<JvmMethod> {

        let jvm_method_name_cstring = CString::new(jvm_method_name).unwrap();
        let jvm_method_signature_cstring = CString::new(jvm_method_signature).unwrap();

        let jvm_method_ptr =
            (**self.jni_environment).GetMethodID.unwrap()(
                self.jni_environment, *jvm_class.jvm_class_ptr(), jvm_method_name_cstring.as_ptr(),
                jvm_method_signature_cstring.as_ptr()
            );

        // Print any JVM exception.
        print_jvm_exception(self.jni_environment);

        if jvm_method_ptr.is_null() {
            return None;
        }

        JvmMethod::new(jvm_method_ptr)
    }

    /// Tries to resolve the static JVM method with the given name and signature in the given JVM class.
    pub unsafe fn get_static_method(
        &self, jvm_class: &JvmClass, jvm_method_name: &str, jvm_method_signature: &str
    ) -> Option<JvmMethod> {

        let jvm_method_name_cstring = CString::new(jvm_method_name).unwrap();
        let jvm_method_signature_cstring = CString::new(jvm_method_signature).unwrap();

        let jvm_method_ptr =
            (**self.jni_environment).GetStaticMethodID.unwrap()(
                self.jni_environment, *jvm_class.jvm_class_ptr(), jvm_method_name_cstring.as_ptr(),
                jvm_method_signature_cstring.as_ptr()
            );

        // Print any JVM exception.
        print_jvm_exception(self.jni_environment);

        if jvm_method_ptr.is_null() {
            return None;
        }

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
            // TODO: Destroy the JVM. This SIGSEGVs.
            // (**self.jvm).DestroyJavaVM.unwrap()(self.jvm);
        }
    }
}


#[link(name="jvm")]
extern {
    // TODO: use `JNI_CreateJavaVM()` from rust-jni-sys > 0.2.1:
    fn JNI_CreateJavaVM(pvm: *mut *mut JavaVM, penv: *mut *mut c_void, args: *mut c_void) -> jint;
}


#[cfg(test)]
mod tests {

    use super::Jvm;

    #[test]
    fn test_drop() {

        // TODO: This SIGSEGVs. It goes away if we do not call `DestroyJavaVM()`
        for _ in 0..10 {
            unsafe {
                Jvm::new(&[]);
            }
        }
    }

}