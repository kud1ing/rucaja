use jni_sys::jstring;
use jvm_attachment::JvmAttachment;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;


jvm_wrapper!(JvmString, jstring);


impl<'a>  JvmString<'a>  {

    /// Creates and returns a JVM string.
    pub fn new(jvm_attachment: &'a JvmAttachment, string: &str) -> Option<JvmString<'a>> {

        let string_as_cstring = CString::new(string).unwrap();

        let jvm_string_ptr = unsafe {
            (**jvm_attachment.jni_environment()).NewStringUTF?(
                jvm_attachment.jni_environment(),
                string_as_cstring.as_ptr()
            )
        };

        JvmString::from_jvm_ptr(jvm_attachment, jvm_string_ptr)
    }
}

impl<'a> ToString for JvmString<'a> {

    fn to_string(&self) -> String {

        return unsafe {

            // Allocate a char buffer for the `jstring` in the JVM.
            let char_buffer: *const c_char =
                (**self.jvm_attachment.jni_environment()).GetStringUTFChars.unwrap()(
                    self.jvm_attachment.jni_environment(),
                    self.jvm_ptr,
                    ptr::null_mut()
                );

            // Construct a `String` from that char buffer.
            let string = CStr::from_ptr(char_buffer).to_str().unwrap().to_string();

            // Deallocate the char buffer.
            (**self.jvm_attachment.jni_environment()).ReleaseStringUTFChars.unwrap()(
                self.jvm_attachment.jni_environment(),
                self.jvm_ptr,
                char_buffer
            );

            // Return that `String`.
            string
        }

    }
}
