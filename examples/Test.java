/*
 You can see the function signatures using:

    javap -s Test.class
 */
class Test {

    public static void main(String [] args) {
    }

    public static void println(Object object) {
        System.out.println(object);
    }

    public static boolean static_boolean_method(boolean arg) {
        return !arg;
    }

    public static byte static_byte_method() {
        return 42;
    }

    public static char static_char_method() {
        return 'a';
    }

    public static double static_double_method() {
        return 42.0f;
    }

    public static float static_float_method() {
        return 42.0f;
    }

    public static int static_int_method() {
        return 42;
    }

    public static long static_long_method() {
        return 42L;
    }

    public static String static_string_method() {
        return new String("Foo");
    }
    public static String static_string_interned_method() {
        return "Foo";
    }

    public static void static_void_method() {
        System.out.println(":)");
    }
}
