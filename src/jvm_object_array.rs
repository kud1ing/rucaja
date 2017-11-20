use jni_sys::{ jobjectArray, jsize };
use jvm::Jvm;
use jvm_attachment::JvmAttachment;
use jvm_class::JvmClass;
use jvm_object::JvmObject;

jvm_array_wrapper!(JvmObjectArray, jobjectArray);

impl<'a> JvmObjectArray<'a> {
    pub fn new(jvm: &'a Jvm, length: jsize, clazz: &JvmClass, init: &JvmObject) -> Option<JvmObjectArray<'a>> {
        let jvm_array_ptr = unsafe {
            let jvm_attachment = JvmAttachment::new(jvm.jvm());
            (**jvm_attachment.jni_environment()).NewObjectArray.unwrap()(
                jvm_attachment.jni_environment(),
                length,
                clazz.jvm_ptr(),
                init.jvm_ptr()
            )
        };

        JvmObjectArray::from_jvm_ptr(jvm, jvm_array_ptr)
    }
}
