extern crate rucaja;

use rucaja::{Jvm, JvmMethod, JvmObjectArray, JvmObject, jvalue_from_jobject, jvalue_from_jint};

#[test]
fn test_java_arrays() {
    unsafe {
        let jvm = Jvm::new(&[
            "-Xcheck:jni"
        ]);

        let integer_clazz = jvm.get_class("java/lang/Integer").unwrap();

        let integer_constructor = jvm.get_constructor(
            &integer_clazz,
            "(I)V"
        ).unwrap();

        let integer_jvm_ptr = jvm.call_constructor(
            &integer_clazz,
            &integer_constructor,
            vec![
                jvalue_from_jint(42)
            ].as_ptr()
        );

        let integer_object = JvmObject::from_jvm_ptr(&jvm, integer_jvm_ptr).unwrap();

        let arrays_clazz = jvm.get_class("java/util/Arrays").unwrap();

        let binary_search_method = JvmMethod::get_static_method(
            &jvm,
            &arrays_clazz,
            "binarySearch",
            "([Ljava/lang/Object;Ljava/lang/Object;)I"
        ).unwrap();

        let array_length = 10;

        let integer_array_object = JvmObjectArray::new(
            &jvm,
            array_length,
            &integer_clazz,
            &integer_object
        ).unwrap();

        let result = jvm.call_static_int_method(
            &arrays_clazz,
            &binary_search_method,
            vec![
                jvalue_from_jobject(integer_array_object.jvm_ptr()),
                jvalue_from_jobject(integer_object.jvm_ptr())
            ].as_ptr()
        );

        assert!( 0 <= result && result < array_length);
    }
}
