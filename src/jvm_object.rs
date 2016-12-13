use jni_sys::jobject;
use jvm::Jvm;

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
    pub fn maybe_new(jvm: &Jvm, jvm_object_ptr: jobject) -> Option<JvmObject> {

        if jvm_object_ptr.is_null() {
            return None
        }

        // Create a global reference to the given JVM object, to prevent GC claiming it.
        unsafe {
            (**jvm.jni_environment()).NewGlobalRef.unwrap()(jvm.jni_environment(), jvm_object_ptr);
        }

        Some(JvmObject { jvm_object_ptr: jvm_object_ptr } )
    }
}

// TODO
/*
impl Drop for JvmObject {
    fn drop(&mut self) {

        unsafe {
            // Destroy the global reference to the JVM object.
            (*env)->DeleteGlobalRef(env, bufferCls);
        }
    }
}
*/