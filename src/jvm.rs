use jni_sys::{
    JavaVM, JavaVMInitArgs, JavaVMOption, JNI_CreateJavaVM, JNI_ERR, JNI_EDETACHED, JNI_EVERSION,
    JNI_ENOMEM, JNI_EEXIST, JNI_EINVAL, JNI_FALSE, JNI_OK, JNI_VERSION_1_8, JNIEnv, jboolean, jbyte,
    jchar, jint, jdouble, jfloat, jlong, jobject, jshort, jstring, jvalue
};
use jvm_attachment::JvmAttachment;
use jvm_class::JvmClass;
use jvm_method::JvmMethod;
use std::ffi::CString;
use std::ptr;
use std::os::raw::c_void;

// =================================================================================================

#[link(name="jvm")]
extern {
}

// =================================================================================================

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

///
unsafe fn jvm_exception_occured(jni_environment: *mut JNIEnv) -> bool {
    return !(**jni_environment).ExceptionOccurred.unwrap()(jni_environment).is_null()
}

///
unsafe fn print_and_panic_on_jvm_exception(jni_environment: *mut JNIEnv) {

    // A JVM exception occurred.
    if jvm_exception_occured(jni_environment) {

        // Print the JVM exception.
        (**jni_environment).ExceptionDescribe.unwrap()(jni_environment);

        // Panic.
        panic!("An exception occurred");
    };
}

///
unsafe fn print_jvm_exception(jni_environment: *mut JNIEnv) {

    // A JVM exception occurred.
    if jvm_exception_occured(jni_environment) {

        // Print the JVM exception.
        (**jni_environment).ExceptionDescribe.unwrap()(jni_environment);
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
    /// unsafe {
    ///   Jvm::new(&["-Xcheck:jni"]);
    /// }
    /// ```
    pub unsafe fn new(jvm_option_strings: &[&str]) -> Jvm {

        // Wrap the JVM option string slices in a vector of `CString`s.
        let mut jvm_option_cstrings : Vec<CString> = Vec::new();

        for jvm_option_string in jvm_option_strings {
            jvm_option_cstrings.push(CString::new(*jvm_option_string).unwrap());
        }

        // Create a vector of `JavaVMOption`s, each referencing a `CString`.
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

        // Initialize space for a pointer to the JNI environment.
        let mut jvm: *mut JavaVM = ptr::null_mut();
        let mut jni_environment : *mut JNIEnv = ptr::null_mut();

        // Try to instantiate the JVM.
        let result = JNI_CreateJavaVM(
            &mut jvm,
            (&mut jni_environment as *mut *mut JNIEnv) as *mut *mut c_void,
            (&mut jvm_arguments as *mut JavaVMInitArgs) as *mut c_void
        );

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

        Jvm {
            jvm: jvm,
        }
    }

    /// Tries to call the given JVM object constructor in the given JVM class.
    /// Currently panics if a JVM exception occurs.
    pub unsafe fn call_constructor(
        &self, jvm_class: &JvmClass, jvm_constructor_method: &JvmMethod, args: *const jvalue
    ) -> jobject {
        // Attach the current native thread to the JVM.
        let jvm_attachment = JvmAttachment::new(self.jvm);

        let object = (**jvm_attachment.jni_environment()).NewObjectA.unwrap()(
            jvm_attachment.jni_environment(),
            *jvm_class.jvm_class_ptr(),
            *jvm_constructor_method.jvm_method_ptr(),
            args
        );

        print_and_panic_on_jvm_exception(jvm_attachment.jni_environment());

        object
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

        // Attach the current native thread to the JVM.
        let jvm_attachment = JvmAttachment::new(self.jvm);

        let result = (**jvm_attachment.jni_environment()).CallStaticBooleanMethodA.unwrap()(
            jvm_attachment.jni_environment(),
            *jvm_class.jvm_class_ptr(),
            *jvm_method.jvm_method_ptr(),
            args
        );

        print_and_panic_on_jvm_exception(jvm_attachment.jni_environment());

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

        // Attach the current native thread to the JVM.
        let jvm_attachment = JvmAttachment::new(self.jvm);

        let result = (**jvm_attachment.jni_environment()).CallStaticObjectMethodA.unwrap()(
            jvm_attachment.jni_environment(),
            *jvm_class.jvm_class_ptr(),
            *jvm_method.jvm_method_ptr(),
            args
        );

        print_and_panic_on_jvm_exception(jvm_attachment.jni_environment());

        result
    }

    /// Tries to call the given JVM static void method in the given JVM class.
    /// Currently panics if a JVM exception occurs.
    pub unsafe fn call_static_void_method(
        &self, jvm_class: &JvmClass, jvm_method: &JvmMethod, args: *const jvalue
    ) {

        // Attach the current native thread to the JVM.
        let jvm_attachment = JvmAttachment::new(self.jvm);

        (**jvm_attachment.jni_environment()).CallStaticVoidMethodA.unwrap()(
            jvm_attachment.jni_environment(),
            *jvm_class.jvm_class_ptr(),
            *jvm_method.jvm_method_ptr(),
            args
        );

        print_and_panic_on_jvm_exception(jvm_attachment.jni_environment());

    }

    /// Tries to resolve the JVM class with the given name.
    pub unsafe fn get_class(&self, jvm_class_name: &str) -> Option<JvmClass> {

        // Attach the current native thread to the JVM.
        let jvm_attachment = JvmAttachment::new(self.jvm);

        let jvm_class_name_cstring = CString::new(jvm_class_name).unwrap();

        let jvm_class_ptr =
            (**jvm_attachment.jni_environment()).FindClass.unwrap()(
                jvm_attachment.jni_environment(),
                jvm_class_name_cstring.as_ptr()
            );

        // Print any JVM exception.
        print_jvm_exception(jvm_attachment.jni_environment());

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

        // Attach the current native thread to the JVM.
        let jvm_attachment = JvmAttachment::new(self.jvm);

        let jvm_method_name_cstring = CString::new(jvm_method_name).unwrap();
        let jvm_method_signature_cstring = CString::new(jvm_method_signature).unwrap();

        let jvm_method_ptr =
            (**jvm_attachment.jni_environment()).GetMethodID.unwrap()(
                jvm_attachment.jni_environment(),
                *jvm_class.jvm_class_ptr(),
                jvm_method_name_cstring.as_ptr(),
                jvm_method_signature_cstring.as_ptr()
            );

        // Print any JVM exception.
        print_jvm_exception(jvm_attachment.jni_environment());

        if jvm_method_ptr.is_null() {
            return None;
        }

        JvmMethod::new(jvm_method_ptr)
    }

    /// Tries to resolve the static JVM method with the given name and signature in the given JVM class.
    pub unsafe fn get_static_method(
        &self, jvm_class: &JvmClass, jvm_method_name: &str, jvm_method_signature: &str
    ) -> Option<JvmMethod> {

        // Attach the current native thread to the JVM.
        let jvm_attachment = JvmAttachment::new(self.jvm);

        let jvm_method_name_cstring = CString::new(jvm_method_name).unwrap();
        let jvm_method_signature_cstring = CString::new(jvm_method_signature).unwrap();

        let jvm_method_ptr =
            (**jvm_attachment.jni_environment()).GetStaticMethodID.unwrap()(
                jvm_attachment.jni_environment(),
                *jvm_class.jvm_class_ptr(),
                jvm_method_name_cstring.as_ptr(),
                jvm_method_signature_cstring.as_ptr()
            );

        // Print any JVM exception.
        print_jvm_exception(jvm_attachment.jni_environment());

        if jvm_method_ptr.is_null() {
            return None;
        }

        JvmMethod::new(jvm_method_ptr)
    }

    /// Creates and returns a non-interned JVM string.
    pub unsafe fn new_jstring(&self, string: &str) -> jstring {

        // Attach the current native thread to the JVM.
        let jvm_attachment = JvmAttachment::new(self.jvm);

        // Get the string class.
        let string_class =
            self.get_class("java/lang/String").
            expect("Could not find class `java.lang.String`");

        // Get the constructor.
        // TODO: choose a string constructor that accepts a string.
        let string_constructor = self.get_constructor(
            &string_class,
            "(Ljava/lang/String;)V",
            ).expect("Could not find JVM method");

        let string_as_cstring = CString::new(string).unwrap();

        // TODO: this still returns an interned string.

        // Call the constructor.
        let args = vec![jvalue_from_jobject(self.new_jstring_interned(string))];
        self.call_constructor(&string_class, &string_constructor, args.as_ptr())
    }

    /// Creates and returns an interned JVM string.
    pub unsafe fn new_jstring_interned(&self, string: &str) -> jstring {

        // Attach the current native thread to the JVM.
        let jvm_attachment = JvmAttachment::new(self.jvm);

        let string_as_cstring = CString::new(string).unwrap();

        // TODO: call the constructor on "java/lang/String" instead because `NewStringUTF()`
        // creates interned strings which give `java.security.AccessControlContext`.

        let result = (**jvm_attachment.jni_environment()).NewStringUTF.unwrap()(
            jvm_attachment.jni_environment(),
            string_as_cstring.as_ptr()
        );

        print_and_panic_on_jvm_exception(jvm_attachment.jni_environment());

        result
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
