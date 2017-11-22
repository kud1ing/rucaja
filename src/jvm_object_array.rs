use jni_sys::{ jobjectArray, jsize };
use jvm_attachment::JvmAttachment;
use jvm_class::JvmClass;
use jvm_object::JvmObject;

jvm_array_wrapper!(JvmObjectArray, jobjectArray);

impl<'a> JvmObjectArray<'a> {
    pub fn new(jvm_attachment: &'a JvmAttachment, length: jsize, clazz: &JvmClass, init: &JvmObject) -> Option<JvmObjectArray<'a>> {
        let jvm_array_ptr = unsafe {

            (**jvm_attachment.jni_environment()).NewObjectArray.unwrap()(
                jvm_attachment.jni_environment(),
                length,
                clazz.jvm_ptr(),
                init.jvm_ptr()
            )
        };

        JvmObjectArray::from_jvm_ptr(jvm_attachment, jvm_array_ptr)
    }
}
