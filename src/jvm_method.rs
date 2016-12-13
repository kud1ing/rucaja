use jni_sys::jmethodID;

///
pub struct JvmMethod {
    
    // Guaranteed not to be a null pointer.
    jvm_method_ptr: jmethodID,
}

impl JvmMethod {

    ///
    pub fn jvm_method_ptr(&self) -> &jmethodID {
        &self.jvm_method_ptr
    }

    ///
    pub fn maybe_new(jvm_method_ptr: jmethodID) -> Option<JvmMethod> {

        if jvm_method_ptr.is_null() {
            return None
        }

        Some(JvmMethod { jvm_method_ptr: jvm_method_ptr } )
    }
}

