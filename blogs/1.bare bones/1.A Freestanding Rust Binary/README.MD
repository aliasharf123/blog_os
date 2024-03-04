# A Freestanding Rust Binary

<aside>
💡 All code will find in source file and to get all steps you can read full detailed blog.

</aside>

## **Introduction**

To write an operating system kernel, we need code that does not depend on any operating system features. This means that we can’t use threads, files, heap memory, the network, random numbers, standard output, or any other features requiring OS abstractions or specific hardware. Which makes sense, since we’re trying to write our own OS and our own drivers therefore, we can’t use most of the [Rust standard library](https://doc.rust-lang.org/std/).

## **Disabling the Standard Library**

in order to create an OS kernel in Rust, we need to create an executable that can be run without an underlying operating system. Such an executable is often called a “freestanding” or “bare-metal” executable.

By default, all Rust crates link the [standard library](https://doc.rust-lang.org/std/), which depends on the operating system for features, it also depends on the C standard library `libc`, which closely interacts with OS services, so we have to disable the automatic inclusion of the standard library through the `[no_std` attribute](https://doc.rust-lang.org/1.30.0/book/first-edition/using-rust-without-the-standard-library.html).

## **Panic Implementation**

The `panic_handler` attribute defines the function that the compiler should invoke when a [panic](https://doc.rust-lang.org/stable/book/ch09-01-unrecoverable-errors-with-panic.html) occurs. The standard library provides its own panic handler function, but in a `no_std` environment we need to define it ourselves.

## **The `eh_personality` Language Item**

Language items are specialized functions and types crucial for the compiler's internal workings. One example is the `Copy` trait, designated as a language item with the `#[lang = "copy"]` attribute. Although it's possible to provide custom implementations for language items, it's generally discouraged due to their highly unstable nature and lack of type checking.

One specific language item is `eh_personality`, which designates a function used for [stack unwinding](https://www.bogotobogo.com/cplusplus/stackunwinding.php) in case of panics. Rust defaults to unwinding to execute destructors during a panic, ensuring proper memory cleanup. However, this process is intricate and OS-specific, requiring libraries like [libunwind](https://www.nongnu.org/libunwind/) on Linux or [structured exception handling](https://docs.microsoft.com/en-us/windows/win32/debug/structured-exception-handling) on Windows. In the context of building an operating system, it's advisable to avoid unwinding and explore alternative approaches for handling errors.

### **Disabling Unwinding**

There are other use cases as well for which unwinding is undesirable, so Rust provides an option to [abort on panic](https://github.com/rust-lang/rust/pull/32900) instead. This disables the generation of unwinding symbol information and thus considerably reduces binary size.

## **The `start` attribute**

In Rust binaries linked with the standard library, execution starts with the C runtime library, `crt0`, before reaching Rust's minimal runtime and the main function. The runtime performs essential tasks like setting up stack guards and handling panics. For a freestanding executable without access to Rust's runtime and crt0, defining a custom entry point by overwriting crt0 becomes necessary. This ensures proper initialization for the program.

## **Linker Errors**

The linker is a program that combines the generated code into an executable. Since the executable format differs between Linux, Windows, and macOS, each system has its own linker that throws a different error. The fundamental cause of the errors is the same: the default configuration of the linker assumes that our program depends on the C runtime, which it does not.

To solve the errors, we need to tell the linker that it should not include the C runtime. We can do this either by passing a certain set of arguments to the linker or by building for a bare metal target.

### **Building for a Bare Metal Target**

By default Rust tries to build an executable that is able to run in your current system environment. For example, if you’re using Windows on `x86_64`, Rust tries to build an `.exe` Windows executable that uses `x86_64` instructions. This environment is called your “host” system.

By compiling for our host triple, the Rust compiler and the linker assume that there is an underlying operating system such as Linux or Windows that uses the C runtime by default, which causes the linker errors. So, to avoid the linker errors, we can compile for a different environment with no underlying operating system.

An example of such a bare metal environment is the `thumbv7em-none-eabihf` target triple, which describes an [embedded](https://en.wikipedia.org/wiki/Embedded_system) [ARM](https://en.wikipedia.org/wiki/ARM_architecture) system. The details are not important, all that matters is that the target triple has no underlying operating system, which is indicated by the `none` in the target triple.

### References:

- Full Detailed Blog: [A Freestanding Rust Binary | Writing an OS in Rust (phil-opp.com)](https://os.phil-opp.com/freestanding-rust-binary/)
