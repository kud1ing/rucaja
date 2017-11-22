use jni_sys::jstring;
use jvm::Jvm;
use jvm_attachment::JvmAttachment;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;


jvm_wrapper!(JvmString, jstring);


impl<'a>  JvmString<'a>  {


    /// Creates and returns a JVM string.
    pub unsafe fn new(jvm: &'a Jvm, string: &str) -> Option<JvmString<'a>> {

        // Attach the current native thread to the JVM.
        let jvm_attachment = JvmAttachment::new(jvm.jvm());

        let string_as_cstring = CString::new(string).unwrap();

        let jvm_string_ptr = (**jvm_attachment.jni_environment()).NewStringUTF.unwrap()(
            jvm_attachment.jni_environment(),
            string_as_cstring.as_ptr()
        );

        JvmString::from_jvm_ptr(jvm, jvm_string_ptr)
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
