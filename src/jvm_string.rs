use jni_sys::jstring;
use jvm::Jvm;
use jvm_attachment::JvmAttachment;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;


/// Represents an string in the JVM.
pub struct JvmString<'a> {

    jvm: &'a Jvm,

    // Guaranteed not to be a null pointer.
    jvm_string_ptr: jstring,
}

impl<'a> JvmString<'a> {

    ///
    pub fn jvm_string_ptr(&self) -> &jstring {
        &self.jvm_string_ptr
    }

    ///
    pub fn new(jvm: &Jvm, jvm_string_ptr: jstring) -> Option<JvmString> {

        if jvm_string_ptr.is_null() {
            return None;
        }

        // Create a global JVM reference to the given JVM string, to prevent GC claiming it.
        let jvm_object_ptr_global = unsafe {
            let jvm_attachment = JvmAttachment::new(jvm.jvm());
            (**jvm_attachment.jni_environment()).NewGlobalRef.unwrap()(jvm_attachment.jni_environment(), jvm_string_ptr)
        };

        if jvm_object_ptr_global.is_null() {
            return None;
        }

        Some(
            JvmString {
                jvm: jvm,
                jvm_string_ptr: jvm_object_ptr_global
            }
        )
    }
}

impl<'a> Drop for JvmString<'a> {
    fn drop(&mut self) {

        // Delete the global JVM reference to the JVM string.
        unsafe {
            let jvm_attachment = JvmAttachment::new(self.jvm.jvm());

            (**jvm_attachment.jni_environment()).DeleteGlobalRef.unwrap()(
                jvm_attachment.jni_environment(), self.jvm_string_ptr
            );
        }
    }
}

impl<'a> ToString for JvmString<'a> {

    fn to_string(&self) -> String {

        return unsafe {

            // Attach the current native thread to the JVM.
            let jvm_attachment = JvmAttachment::new(self.jvm.jvm());

            // Allocate a char buffer for the `jstring` in the JVM.
            let char_buffer: *const c_char =
                (**jvm_attachment.jni_environment()).GetStringUTFChars.unwrap()(
                    jvm_attachment.jni_environment(),
                    self.jvm_string_ptr,
                    ptr::null_mut()
                );

            // Construct a `String` from that char buffer.
            let string = CStr::from_ptr(char_buffer).to_str().unwrap().to_string();

            // Deallocate the char buffer.
            (**jvm_attachment.jni_environment()).ReleaseStringUTFChars.unwrap()(
                jvm_attachment.jni_environment(), self.jvm_string_ptr, char_buffer
            );

            // Return that `String`.
            string
        }

    }
}