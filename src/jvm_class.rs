use jni_sys::jclass;
use jvm::Jvm;


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
    pub fn maybe_new(jvm: &Jvm, jvm_class_ptr: jclass) -> Option<JvmClass> {

        if jvm_class_ptr.is_null() {
            return None
        }

        // Create a global reference to the given JVM class object, to prevent GC claiming it.
        unsafe {
            (**jvm.jni_environment()).NewGlobalRef.unwrap()(jvm.jni_environment(), jvm_class_ptr);
        }

        Some(JvmClass { jvm_class_ptr: jvm_class_ptr } )
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