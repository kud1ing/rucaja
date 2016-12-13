use jni_sys::jobject;

///
pub struct JvmObject {

    // Guaranteed not to be a null pointer.
    jvm_object_ptr: jobject,
}

impl JvmObject {

    ///
    pub fn jvm_object_ptr(&self) -> &jobject {
        &self.jvm_object_ptr
    }

    ///
    pub fn maybe_new(jvm_object_ptr: jobject) -> Option<JvmObject> {

        if jvm_object_ptr.is_null() {
            return None
        }

        Some(JvmObject { jvm_object_ptr: jvm_object_ptr } )
    }
}