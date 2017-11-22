use jni_sys::{jboolean, jint, jmethodID, jobject, jvalue};
use jvm::{print_and_panic_on_jvm_exception, print_jvm_exception};
use jvm_attachment::JvmAttachment;
use jvm_class::JvmClass;
use jvm_object::JvmObject;
use std::ffi::CString;


/// Represents a method in a class in the JVM.
pub struct JvmMethod {
    
    // Guaranteed not to be a null pointer.
    jvm_method_ptr: jmethodID,
}

impl<'a> JvmMethod {

    /// Tries to call the given JVM object constructor in the given JVM class.
    /// Currently panics if a JVM exception occurs.
    pub unsafe fn call_constructor(
        jvm_attachment: &JvmAttachment, jvm_class: &JvmClass, jvm_constructor_method: &JvmMethod, args: *const jvalue
    ) -> jobject {

        let object = (**jvm_attachment.jni_environment()).NewObjectA.unwrap()(
            jvm_attachment.jni_environment(),
            jvm_class.jvm_ptr(),
            jvm_constructor_method.jvm_ptr(),
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
        jvm_attachment: &JvmAttachment, jvm_class: &JvmClass, jvm_method: &JvmMethod, args: *const jvalue
    ) -> jboolean {

        let result = (**jvm_attachment.jni_environment()).CallStaticBooleanMethodA.unwrap()(
            jvm_attachment.jni_environment(),
            jvm_class.jvm_ptr(),
            jvm_method.jvm_ptr(),
            args
        );

        print_and_panic_on_jvm_exception(jvm_attachment.jni_environment());

        result
    }

    // TODO: call_static_byte_method()

    // TODO: call_static_char_method()

    // TODO: call_static_double_method()

    // TODO: call_static_float_method()

    /// Tries to call the given JVM static int method in the given JVM class.
    /// Currently panics if a JVM exception occurs.
    pub unsafe fn call_static_int_method(
        jvm_attachment: &JvmAttachment, jvm_class: &JvmClass, jvm_method: &JvmMethod, args: *const jvalue
    ) -> jint {

        let result = (**jvm_attachment.jni_environment()).CallStaticIntMethodA.unwrap()(
            jvm_attachment.jni_environment(),
            jvm_class.jvm_ptr(),
            jvm_method.jvm_ptr(),
            args
        );

        print_and_panic_on_jvm_exception(jvm_attachment.jni_environment());

        result
    }

    // TODO: call_static_long_method()

    ///
    pub unsafe fn call_static_object_method(
        jvm_attachment: &'a JvmAttachment, jvm_class: &JvmClass, jvm_method: &JvmMethod, args: *const jvalue
    ) -> Option<JvmObject<'a>> {

        let result = (**jvm_attachment.jni_environment()).CallStaticObjectMethodA.unwrap()(
            jvm_attachment.jni_environment(),
            jvm_class.jvm_ptr(),
            jvm_method.jvm_ptr(),
            args
        );

        print_and_panic_on_jvm_exception(jvm_attachment.jni_environment());

        JvmObject::from_jvm_ptr(jvm_attachment, result)
    }

    /// Tries to call the given JVM static void method in the given JVM class.
    /// Currently panics if a JVM exception occurs.
    pub unsafe fn call_static_void_method(
        jvm_attachment: &JvmAttachment, jvm_class: &JvmClass, jvm_method: &JvmMethod, args: *const jvalue
    ) {
        (**jvm_attachment.jni_environment()).CallStaticVoidMethodA.unwrap()(
            jvm_attachment.jni_environment(),
            jvm_class.jvm_ptr(),
            jvm_method.jvm_ptr(),
            args
        );

        print_and_panic_on_jvm_exception(jvm_attachment.jni_environment());
    }

    /// Tries to resolve the JVM constructor with the given signature in the given JVM class.
    pub unsafe fn get_constructor(jvm_attachment: &JvmAttachment, jvm_class: &JvmClass, jvm_method_signature: &str) -> Option<JvmMethod> {

        JvmMethod::get_method(jvm_attachment, jvm_class, "<init>", jvm_method_signature)
    }

    /// Tries to resolve the JVM method with the given name and signature in the given JVM class.
    pub unsafe fn get_method(
        jvm_attachment: &JvmAttachment, jvm_class: &JvmClass, jvm_method_name: &str, jvm_method_signature: &str
    ) -> Option<JvmMethod> {

        let jvm_method_name_cstring = CString::new(jvm_method_name).unwrap();
        let jvm_method_signature_cstring = CString::new(jvm_method_signature).unwrap();

        let jvm_method_ptr =
            (**jvm_attachment.jni_environment()).GetMethodID.unwrap()(
                jvm_attachment.jni_environment(),
                jvm_class.jvm_ptr(),
                jvm_method_name_cstring.as_ptr(),
                jvm_method_signature_cstring.as_ptr()
            );

        // Print any JVM exception.
        print_jvm_exception(jvm_attachment.jni_environment());

        JvmMethod::new(jvm_method_ptr)
    }

    /// Tries to resolve the static JVM method with the given name and signature in the given JVM class.
    pub unsafe fn get_static_method(
        jvm_attachment: &JvmAttachment, jvm_class: &JvmClass, jvm_method_name: &str, jvm_method_signature: &str
    ) -> Option<JvmMethod> {

        let jvm_method_name_cstring = CString::new(jvm_method_name).unwrap();
        let jvm_method_signature_cstring = CString::new(jvm_method_signature).unwrap();

        let jvm_method_ptr =
            (**jvm_attachment.jni_environment()).GetStaticMethodID.unwrap()(
                jvm_attachment.jni_environment(),
                jvm_class.jvm_ptr(),
                jvm_method_name_cstring.as_ptr(),
                jvm_method_signature_cstring.as_ptr()
            );

        // Print any JVM exception.
        print_jvm_exception(jvm_attachment.jni_environment());

        JvmMethod::new(jvm_method_ptr)
    }

    ///
    pub fn jvm_ptr(&self) -> jmethodID {
        self.jvm_method_ptr
    }

    ///
    pub fn new(jvm_method_ptr: jmethodID) -> Option<JvmMethod> {

        if jvm_method_ptr.is_null() {
            return None;
        }

        // A `jmethodID` is not a `jobject`, that's why it's neither necessary nor possible to call `NewGlobalRef()`.
        
        Some(JvmMethod { jvm_method_ptr } )
    }
}
