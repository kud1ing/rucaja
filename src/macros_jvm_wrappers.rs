/// Constructs a type safe Rust wrapper struct for a JVM class.
macro_rules! jvm_wrapper {
    ($rust_struct_name:ident, $java_type:ident) => {
        /// The Rust wrapper for the corresponding JVM object.
        pub struct $rust_struct_name<'a> {
            // A reference to the JVM attachment.
            jvm_attachment: &'a JvmAttachment,

            // A non-null pointer to an object in the JVM.
            jvm_ptr: $java_type,
        }

        impl<'a> $rust_struct_name<'a> {
            /// Returns the pointer of the wrapped JVM object.
            pub fn jvm_ptr(&self) -> $java_type {
                self.jvm_ptr
            }

            /// Instantiates the JVM wrapper struct.
            pub fn from_jvm_ptr(
                jvm_attachment: &JvmAttachment,
                jvm_ptr: $java_type,
            ) -> Option<$rust_struct_name> {
                if jvm_ptr.is_null() {
                    return None;
                }

                let jvm_ptr_global = unsafe {
                    // Hold a global JVM reference to the given JVM object, in order to prevent
                    // the JVM GC from claiming it.
                    (**jvm_attachment.jni_environment()).NewGlobalRef?(
                        jvm_attachment.jni_environment(),
                        jvm_ptr,
                    )
                };

                // Could not get the global JVM reference .
                if jvm_ptr_global.is_null() {
                    return None;
                }

                Some($rust_struct_name {
                    jvm_attachment: jvm_attachment,
                    jvm_ptr: jvm_ptr_global,
                })
            }
        }

        impl<'a> Drop for $rust_struct_name<'a> {
            fn drop(&mut self) {
                unsafe {
                    // Delete the global JVM reference to the JVM object.
                    (**self.jvm_attachment.jni_environment())
                        .DeleteGlobalRef
                        .unwrap()(self.jvm_attachment.jni_environment(), self.jvm_ptr);
                }
            }
        }
    };
}

/// Constructs a type safe Rust wrapper struct for a JVM array class.
macro_rules! jvm_array_wrapper {
    ($rust_struct_name:ident, $java_type:ident) => {
        jvm_wrapper!($rust_struct_name, $java_type);

        impl<'a> $rust_struct_name<'a> {
            /// Returns the array length.
            pub fn length(&self) -> jsize {
                unsafe {
                    (**self.jvm_attachment.jni_environment())
                        .GetArrayLength
                        .unwrap()(self.jvm_attachment.jni_environment(), self.jvm_ptr)
                }
            }
        }
    };
}
