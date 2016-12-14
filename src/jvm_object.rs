use jni_sys::jobject;
use jvm::Jvm;


/// Represents an object in the JVM.
pub struct JvmObject<'a> {

    jvm: &'a Jvm,

    // Guaranteed not to be a null pointer.
    jvm_object_ptr: jobject,
}

impl<'a> JvmObject<'a> {

    ///
    pub fn jvm_object_ptr(&self) -> &jobject {
        &self.jvm_object_ptr
    }

    ///
    pub fn new(jvm: &Jvm, jvm_object_ptr: jobject) -> Option<JvmObject> {

        if jvm_object_ptr.is_null() {
            return None;
        }

        // Create a global JVM reference to the given JVM object, to prevent GC claiming it.
        unsafe {
            (**jvm.jni_environment()).NewGlobalRef.unwrap()(jvm.jni_environment(), jvm_object_ptr);
        }

        Some(
            JvmObject {
                jvm: jvm,
                jvm_object_ptr: jvm_object_ptr
            }
        )
    }
}


impl<'a> Drop for JvmObject<'a> {
    fn drop(&mut self) {

        // Delete the global JVM reference to the JVM object.
        unsafe {
            (**self.jvm.jni_environment()).DeleteGlobalRef.unwrap()(
                self.jvm.jni_environment(), self.jvm_object_ptr
            );
        }
    }
}