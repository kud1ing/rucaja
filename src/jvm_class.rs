use jni_sys::jclass;

///
pub struct JvmClass {

    // Guaranteed not to be a null pointer.
    jvm_class_ptr: jclass,
}

impl JvmClass {

    ///
    pub fn jvm_class_ptr(&self) -> &jclass {
        &self.jvm_class_ptr
    }

    ///
    pub fn maybe_new(jvm_class_ptr: jclass) -> Option<JvmClass> {

        if jvm_class_ptr.is_null() {
            return None
        }

        Some(JvmClass { jvm_class_ptr: jvm_class_ptr } )
    }
}
