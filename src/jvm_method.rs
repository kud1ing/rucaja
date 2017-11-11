use jni_sys::jmethodID;


/// Represents a method in a class in the JVM.
pub struct JvmMethod {
    
    // Guaranteed not to be a null pointer.
    jvm_method_ptr: jmethodID,
}

impl JvmMethod {

    ///
    pub fn jvm_ptr(&self) -> &jmethodID {
        &self.jvm_method_ptr
    }

    ///
    pub fn new(jvm_method_ptr: jmethodID) -> Option<JvmMethod> {

        if jvm_method_ptr.is_null() {
            return None;
        }

        Some(JvmMethod { jvm_method_ptr: jvm_method_ptr } )
    }
}

