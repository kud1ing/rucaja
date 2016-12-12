use jni_sys::jclass;

///
pub struct JvmClass {

    // Guaranteed not to be a null pointer.
    java_class_ptr: jclass,
}

impl JvmClass {

    ///
    pub fn java_class_ptr(&self) -> &jclass {
        &self.java_class_ptr
    }

    ///
    pub fn maybe_new(java_class_ptr: jclass) -> Option<JvmClass> {

        if java_class_ptr.is_null() {
            return None
        }

        Some(JvmClass { java_class_ptr: java_class_ptr } )
    }
}
