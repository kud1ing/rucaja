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

    public static boolean static_method_that_returns_a_boolean(boolean arg) {
        return !arg;
    }

    public static byte static_method_that_returns_a_byte() {
        return 42;
    }

    public static char static_method_that_returns_a_char() {
        return 'a';
    }

    public static double static_method_that_returns_a_double() {
        return 42.0f;
    }

    public static float static_method_that_returns_a_float() {
        return 42.0f;
    }

    public static int static_method_that_returns_a_int() {
        return 42;
    }

    public static long static_method_that_returns_a_long() {
        return 42L;
    }

    public static String static_method_that_returns_a_string() {
        return new String("Foo");
    }
    public static String static_method_that_returns_an_interned_string() {
        return "Foo";
    }

    public static void static_void_method() {
        System.out.println(":)");
    }
}
