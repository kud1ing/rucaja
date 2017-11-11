/// Constructs a type safe Rust wrapper struct for a JVM class.
macro_rules! jvm_wrapper_struct {
    ($rust_struct_name:ident, $java_type:ident) => (

        /// A Rust wrapper for a JVM class.
        pub struct $rust_struct_name<'a> {

            jvm: &'a Jvm,

            // A non-null JVM-pointer.
            jvm_ptr: $java_type,
        }

        impl<'a> $rust_struct_name<'a> {

            /// Returns a reference to the JVM object.
            pub fn jvm_ptr(&self) -> &$java_type {
                &self.jvm_ptr
            }

            /// Instantiates the JVM wrapper struct.
            pub fn new(jvm: &Jvm, jvm_ptr: $java_type) -> Option<$rust_struct_name> {

                if jvm_ptr.is_null() {
                    return None;
                }

                // Create a global JVM reference to the given JVM object, to prevent GC from claiming it.
                let jvm_ptr_global = unsafe {
                    let jvm_attachment = JvmAttachment::new(jvm.jvm());
                    (**jvm_attachment.jni_environment()).NewGlobalRef.unwrap()(jvm_attachment.jni_environment(), jvm_ptr)
                };

                // Could not get the global JVM reference .
                if jvm_ptr_global.is_null() {
                    return None;
                }

                Some(
                    $rust_struct_name {
                        jvm: jvm,
                        jvm_ptr: jvm_ptr_global
                    }
                )
            }
        }

        impl<'a> Drop for $rust_struct_name<'a> {

            fn drop(&mut self) {

                unsafe {

                    // Attach the current native thread to the JVM.
                    let jvm_attachment = JvmAttachment::new(self.jvm.jvm());

                    // Delete the global JVM reference to the JVM object.
                    (**jvm_attachment.jni_environment()).DeleteGlobalRef.unwrap()(
                        jvm_attachment.jni_environment(),
                        self.jvm_ptr
                    );
                }
            }
        }
    )
}
