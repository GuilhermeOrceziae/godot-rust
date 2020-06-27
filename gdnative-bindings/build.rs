use gdnative_bindings_generator::*;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write as _};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let generated_rs = out_path.join("generated.rs");
    let icalls_rs = out_path.join("icalls.rs");

    let api = Api::new();
    let binding_res = generate_bindings(&api);

    {
        use heck::SnakeCase as _;

        let mut output = BufWriter::new(File::create(&generated_rs).unwrap());

        for (class_name, code) in binding_res.class_bindings {
            write!(
                &mut output,
                r#"
                pub mod {mod_name} {{
                    use super::*;
                    {content}
                }}
                pub use crate::generated::{mod_name}::{class_name};
                "#,
                mod_name = class_name.to_snake_case(),
                class_name = class_name,
                content = code,
            )
            .unwrap();
        }
    }

    {
        let mut output = BufWriter::new(File::create(&icalls_rs).unwrap());

        write!(&mut output, "{}", binding_res.icalls).unwrap();
    }

    if cfg!(feature = "formatted") {
        format_file(&generated_rs);
        format_file(&icalls_rs);
    }

    // build.rs will automatically be recompiled and run if it's dependencies are updated.
    // Ignoring all but build.rs will keep from needless rebuilds.
    // Manually rebuilding the crate will ignore this.
    println!("cargo:rerun-if-changed=build.rs");
}

fn format_file(output_rs: &PathBuf) {
    print!(
        "Formatting generated file: {}... ",
        output_rs.file_name().and_then(|s| s.to_str()).unwrap()
    );
    match Command::new("rustup")
        .arg("run")
        .arg("stable")
        .arg("rustfmt")
        .arg("--edition=2018")
        .arg(output_rs)
        .output()
    {
        Ok(_) => println!("Done"),
        Err(err) => {
            println!("Failed");
            println!("Error: {}", err);
        }
    }
}
