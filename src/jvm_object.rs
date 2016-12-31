use jni_sys::jobject;
use jvm::Jvm;
use jvm_attachment::JvmAttachment;


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
        let jvm_object_ptr_global = unsafe {
            let jvm_attachment = JvmAttachment::new(jvm.jvm());
            (**jvm_attachment.jni_environment()).NewGlobalRef.unwrap()(jvm_attachment.jni_environment(), jvm_object_ptr)
        };

        if jvm_object_ptr_global.is_null() {
            return None;
        }

        Some(
            JvmObject {
                jvm: jvm,
                jvm_object_ptr: jvm_object_ptr_global
            }
        )
    }
}


impl<'a> Drop for JvmObject<'a> {
    fn drop(&mut self) {

        // Delete the global JVM reference to the JVM object.
        unsafe {
            let jvm_attachment = JvmAttachment::new(self.jvm.jvm());

            (**jvm_attachment.jni_environment()).DeleteGlobalRef.unwrap()(
                jvm_attachment.jni_environment(), self.jvm_object_ptr
            );
        }
    }
}