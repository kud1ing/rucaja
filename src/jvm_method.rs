use jni_sys::jmethodID;

///
pub struct JvmMethod {
    
    // Guaranteed not to be a null pointer.
    java_method_ptr: jmethodID,
}

impl JvmMethod {

    ///
    pub fn java_method_ptr(&self) -> &jmethodID {
        &self.java_method_ptr
    }

    ///
    pub fn maybe_new(java_method_ptr: jmethodID) -> Option<JvmMethod> {

        if java_method_ptr.is_null() {
            return None
        }

        Some(JvmMethod { java_method_ptr: java_method_ptr } )
    }
}

