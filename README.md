# Rucaja (Rust calls Java)

[![Clippy Linting Result](https://clippy.bashy.io/github/kud1ing/rucaja/master/badge.svg)](https://clippy.bashy.io/github/kud1ing/rucaja/master/log)

Calling Java code from Rust via JNI.

The interface is not yet stable and the code is currently only tested on macOS 10.12.
Your platform might need adjustments in `build.rs`.


## Tracing

JVM options:

* -verbose:jni
* -Xcheck:jni:trace


## Mac

Trying to run may give:

    dyld: Library not loaded: @rpath/libjvm.dylib
      Referenced from: ./target/debug/rucaja
      Reason: image not found
    Abort trap: 6

this might require something like:

    sudo ln -s $(/usr/libexec/java_home)/jre/lib/server/libjvm.dylib /usr/local/lib


## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.