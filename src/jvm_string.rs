use jni_sys::jstring;
use jvm::Jvm;
use jvm_attachment::JvmAttachment;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;


jvm_wrapper!(JvmString, jstring);

impl<'a> ToString for JvmString<'a> {

    fn to_string(&self) -> String {

        return unsafe {

            // Attach the current native thread to the JVM.
            let jvm_attachment = JvmAttachment::new(self.jvm.jvm());

            // Allocate a char buffer for the `jstring` in the JVM.
            let char_buffer: *const c_char =
                (**jvm_attachment.jni_environment()).GetStringUTFChars.unwrap()(
                    jvm_attachment.jni_environment(),
                    self.jvm_ptr,
                    ptr::null_mut()
                );

            // Construct a `String` from that char buffer.
            let string = CStr::from_ptr(char_buffer).to_str().unwrap().to_string();

            // Deallocate the char buffer.
            (**jvm_attachment.jni_environment()).ReleaseStringUTFChars.unwrap()(
                jvm_attachment.jni_environment(),
                self.jvm_ptr,
                char_buffer
            );

            // Return that `String`.
            string
        }

    }
}
