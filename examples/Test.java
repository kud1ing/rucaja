/*
 You can see the function signatures using:

 `javap -s Test.class`
 */
class Test {

    public static boolean staticBooleanMethodFalse() {
        return false;
    }
    public static boolean staticBooleanMethodTrue() {
        return true;
    }

    public static byte staticByteMethod() {
        return 42;
    }

    public static char staticCharMethod() {
        return 'a';
    }

    public static double staticDoubleMethod() {
        return 42.0f;
    }

    public static float staticFloatMethod() {
        return 42.0f;
    }

    public static int staticIntMethod() {
        return 42;
    }

    public static long staticLongMethod() {
        return 42L;
    }

    public static String staticObjectMethod() {
        return "Foo";
    }

    public static void staticVoidMethod() {
        System.out.println("staticVoidMethod()");
    }

    public static void main(String [] args) {
    }
}
