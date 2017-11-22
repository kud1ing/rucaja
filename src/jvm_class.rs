use jni_sys::jclass;
use jvm::print_jvm_exception;
use jvm_attachment::JvmAttachment;
use std::ffi::CString;

jvm_wrapper!(JvmClass, jclass);


impl<'a>  JvmClass<'a> {

    /// Tries to resolve the JVM class with the given name.
    pub unsafe fn get_class(jvm_attachment: &'a JvmAttachment, jvm_class_name: &str) -> Option<JvmClass<'a>> {

        let jvm_class_name_cstring = CString::new(jvm_class_name).unwrap();

        let jvm_class_ptr =
            (**jvm_attachment.jni_environment()).FindClass.unwrap()(
                jvm_attachment.jni_environment(),
                jvm_class_name_cstring.as_ptr()
            );

        // Print any JVM exception.
        print_jvm_exception(jvm_attachment.jni_environment());

        JvmClass::from_jvm_ptr(jvm_attachment, jvm_class_ptr)
    }
}