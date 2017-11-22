extern crate rucaja;

use rucaja::{Jvm, JvmAttachment, JvmClass, JvmMethod, JvmString, jvalue_from_jobject};

#[test]
fn test_java_integers() {
    unsafe {
        let jvm = Jvm::new(&[
            "-Xcheck:jni"
        ]);

        // Attach the current native thread to the JVM.
        let jvm_attachment = JvmAttachment::new(jvm.jvm());

        let integer_clazz = JvmClass::get_class(&jvm_attachment, "java/lang/Integer").unwrap();

        let parse_int_method = JvmMethod::get_static_method(
            &jvm_attachment,
            &integer_clazz,
            "parseInt",
            "(Ljava/lang/String;)I"
        ).unwrap();

        for value in -500..500 {
            let jvm_string = JvmString::new(&jvm_attachment, value.to_string().as_str()).unwrap();

            let jvm_int = JvmMethod::call_static_int_method(
                &jvm_attachment,
                &integer_clazz,
                &parse_int_method,
                vec![
                    jvalue_from_jobject(jvm_string.jvm_ptr())
                ].as_ptr()
            );

            assert_eq!(jvm_int, value);
        }
    }
}
