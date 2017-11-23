use std::env;

#[cfg(target_os = "linux")]
fn find_default_java_home() -> String {
    String::from("/usr/lib/jvm/default-java") // This might just be Debian, not sure.
}

#[cfg(target_os = "macos")]
fn find_default_java_home() -> String {
    /*
     * FIXME Find version dynamically.
     * Use: /usr/libexec/java_home -v 1.8
     *
     * Someone that uses macOS should figure this out.
     */
    String::from("/Library/Java/JavaVirtualMachines/jdk1.8.0_51.jdk/Contents/Home")
}

#[cfg(target_os = "windows")]
fn find_default_java_home() -> String {
    /*
     * Windows is silly about how it handles programs.  32-bit programs install to
     * `Program Files (x86)` on 64-bit machines, but programs that match bitness install to the
     * normal `Program Files` directory.
     *
     * Someone that actually uses Windows should figure this out.
     */
    String::from("C:\\Program Files\\Java\\jre8") // FIXME Find version dynamically.
}

fn main() {

    // Try to determine the Java home directory so that we can link to `libjvm`.
    let java_home = env::var("JAVA_HOME").ok().unwrap_or(find_default_java_home());

    // I don't have this on my machine, so I'm not 100% sure what it does.
    print!("cargo:rustc-link-search=native=");
    println!("{}/jre/lib/server", java_home);

    if cfg!(target_arch = "x86_64") {
        print!("cargo:rustc-link-search=native=");
        println!("{}/jre/lib/amd64/server", java_home);
    } else if cfg!(target_arch = "x86") {
        print!("cargo:rustc-link-search=native=");
        println!("{}/jre/lib/i386/server", java_home);
    } else {
        panic!("rucaja is not currently supported on your architecture")
    }

    /*
     * We still need to do annoying things like setting LD_LIBRARY_PATH when we run the executable.
     * It might be better to determine all this at runtime after all.
     */

}
