use jni_sys::jmethodID;
use jvm::{Jvm, print_jvm_exception};
use jvm_attachment::JvmAttachment;
use jvm_class::JvmClass;
use std::ffi::CString;


/// Represents a method in a class in the JVM.
pub struct JvmMethod {
    
    // Guaranteed not to be a null pointer.
    jvm_method_ptr: jmethodID,
}

impl JvmMethod {

    /// Tries to resolve the JVM constructor with the given signature in the given JVM class.
    pub unsafe fn get_constructor(jvm: &Jvm, jvm_class: &JvmClass, jvm_method_signature: &str) -> Option<JvmMethod> {

        JvmMethod::get_method(jvm, jvm_class, "<init>", jvm_method_signature)
    }

    /// Tries to resolve the JVM method with the given name and signature in the given JVM class.
    pub unsafe fn get_method(
        jvm: &Jvm, jvm_class: &JvmClass, jvm_method_name: &str, jvm_method_signature: &str
    ) -> Option<JvmMethod> {

        // Attach the current native thread to the JVM.
        let jvm_attachment = JvmAttachment::new(jvm.jvm());

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
        jvm: &Jvm, jvm_class: &JvmClass, jvm_method_name: &str, jvm_method_signature: &str
    ) -> Option<JvmMethod> {

        // Attach the current native thread to the JVM.
        let jvm_attachment = JvmAttachment::new(jvm.jvm());

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
