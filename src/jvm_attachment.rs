use jni_sys::{JavaVM, JNIEnv};
use std::ptr;
use std::os::raw::c_void;

// =================================================================================================

/// A native thread's attachment to a JVM.
pub struct JvmAttachment {

    /// The JNI environment.
    jni_environment: *mut JNIEnv,

    /// The JVM.
    jvm: *mut JavaVM,
}


impl JvmAttachment {

    ///
    unsafe fn new(jvm: *mut JavaVM) -> JvmAttachment {

        let mut jvm_attachment = JvmAttachment {
            jni_environment: ptr::null_mut(),
            jvm: jvm,
        };

        // Try to attach the current native thread to the JVM.
        let result = (**jvm).AttachCurrentThread.unwrap()(
            jvm,
            (&mut jvm_attachment.jni_environment as *mut *mut JNIEnv) as *mut *mut c_void,
            ptr::null_mut(),
        );

        jvm_attachment
    }
}

// =================================================================================================

impl Drop for JvmAttachment {

    fn drop(&mut self) {

        unsafe {
            // Try to detach the current native thread from the JVM.
            let result = (**self.jvm).DetachCurrentThread.unwrap()(
                self.jvm,
            );
        }
    }
}

// =================================================================================================