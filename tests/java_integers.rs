extern crate rucaja;

use rucaja::{Jvm, JvmString, jvalue_from_jobject};

#[test]
fn test_java_integers() {
    unsafe {
        let jvm = Jvm::new(&[
            "-Xcheck:jni"
        ]);

        let integer_clazz = jvm.get_class("java/lang/Integer").unwrap();

        let parse_int_method = jvm.get_static_method(
            &integer_clazz,
            "parseInt",
            "(Ljava/lang/String;)I"
        ).unwrap();

        for value in -500..500 {
            let jvm_string = JvmString::new(&jvm, value.to_string().as_str()).unwrap();

            let jvm_int = jvm.call_static_int_method(
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
