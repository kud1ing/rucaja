use jni_sys::{JNIEnv, JavaVM};
use std::os::raw::c_void;
use std::ptr;

// =================================================================================================

/// A native thread's attachment to an embedded JVM.
/// There should be only exactly one `JvmAttachment` instance at the same time.
/// The thread is automatically detached when `JvmAttachment` goes out of scope (RAII).
pub struct JvmAttachment {
    /// The JNI environment.
    jni_environment: *mut JNIEnv,

    /// The JVM.
    jvm: *mut JavaVM,
}

impl JvmAttachment {
    ///
    pub fn new(jvm: *mut JavaVM) -> Option<JvmAttachment> {
        // Initialize the data.
        let mut jvm_attachment = JvmAttachment {
            jni_environment: ptr::null_mut(),
            jvm,
        };

        unsafe {
            // Try to attach the current native thread to an embedded JVM.
            let _ = (**jvm).AttachCurrentThread?(
                jvm,
                (&mut jvm_attachment.jni_environment as *mut *mut JNIEnv) as *mut *mut c_void,
                ptr::null_mut(),
            );
        }

        // TODO: interpret the result

        Some(jvm_attachment)
    }

    ///
    pub fn jni_environment(&self) -> *mut JNIEnv {
        self.jni_environment
    }
}

// =================================================================================================

impl Drop for JvmAttachment {
    fn drop(&mut self) {
        unsafe {
            // Try to detach the current native thread from the embedded JVM.
            let _ = (**self.jvm).DetachCurrentThread.unwrap()(self.jvm);

            // TODO: interpret the result
        }
    }
}

// =================================================================================================
