# GDNative bindings for Rust

Rust bindings to the [Godot game engine](http://godotengine.org/).

## Work in progress

The bindings, while usable, are a work in progress. Some APIs are missing and the existing ones are still in flux.

## "Hello World" example

The most general use-case of the bindings will be to interact with Godot using the generated wrapper
classes, as well as providing custom functionality by exposing Rust types as *NativeScript*s.

NativeScript is an extension for GDNative that allows a dynamic library to register "script classes" 
to Godot.

(The following section is a very quick-and-dirty rundown of how to get started with the Rust bindings.
For a more complete and detailed introduction see the [Godot documentation page](https://docs.godotengine.org/en/latest/tutorials/plugins/gdnative/gdnative-c-example.html).)

As is tradition, a simple "Hello World" should serve as an introduction.

(A copy of this "hello world" project can be found in the [`examples`](examples/hello_world) folder. )

### The project setup

Starting with an empty Godot project, a `cargo` project can be created inside the project folder.

```sh
cargo init --lib
```

To use the GDNative bindings in your project you have to add the `gdnative` crate as a dependency.

```toml
[dependencies]
gdnative = "0.5.0"
```

Since GDNative can only use C-compatible dynamic libraries, the crate type has to be set accordingly.

```toml
[lib]
crate-type = ["cdylib"]
```

### The Rust source code

In the `src/lib.rs` file should have the following contents:

```rust
use gdnative::*;

/// The HelloWorld "class"
#[derive(NativeClass)]
#[inherit(Node)]
pub struct HelloWorld;

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl HelloWorld {
    
    /// The "constructor" of the class.
    fn _init(_owner: Node) -> Self {
        HelloWorld
    }
    
    // In order to make a method known to Godot, the #[export] attribute has to be used.
    // In Godot script-classes do not actually inherit the parent class.
    // Instead they are"attached" to the parent object, called the "owner".
    // The owner is passed to every single exposed method.
    #[export]
    fn _ready(&self, _owner: Node) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        godot_print!("hello, world.");
    }
}

// Function that registers all exposed classes to Godot
fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<HelloWorld>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
```

### Creating the NativeScript instance.

After building the library with `cargo build`, the resulting library should be in the `target/debug/` folder.

All NativeScript classes live in a GDNative library.
To specify the GDNative library, a `GDNativeLibrary` resource has to be created.
This can be done in the "Inspector" panel in the Godot editor by clicking the "new resource" button in the top left.

With the `GDNativeLibrary` resource created, the path to the generated binary can be set.

**NOTE**: Resources do not autosave, so after specifying the path, make sure to save
the `GDNativeLibrary` resource by clicking the "tool" button in the Inspector panel in the top right.

Now the `HelloWorld` class can be added to any node by clicking the "add script" button.
In the popup-select the "NativeScript" option and set the class name to "HelloWorld".

**NOTE**: After creation, the NativeScript resource does not automatically point to the `GDNativeLibrary` resource.
Make sure to set click the "library" field in the Inspector and "load" the library.

## Contributing

See the [contribution guidelines](CONTRIBUTING.md)

## License

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be licensed under the [MIT license](LICENSE.md), without any additional terms or conditions.
