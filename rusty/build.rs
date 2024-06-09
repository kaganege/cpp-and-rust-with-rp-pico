use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

extern crate bindgen;
extern crate cbindgen;

const PICO_SDK_PATH: &'static str = env!(
  "PICO_SDK_PATH",
  concat!(env!("USERPROFILE"), "/.pico-sdk/sdk/1.5.1")
);
const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

fn main() {
  println!("cargo:rerun-if-changed=wrapper.h");
  println!("cargo:rerun-if-changed=src/lib.rs");
  println!("cargo:rerun-if-changed=../{PKG_NAME}.h");

  let current_dir = env::current_dir().unwrap();
  let project_path = current_dir.parent().unwrap();
  let build_path = project_path.join("build");

  let mut args: Vec<String> = vec![
    format!(
      "-I{}/.pico-sdk/toolchain/13_2_Rel1/arm-none-eabi/include",
      env!("USERPROFILE")
    ),
    format!(
      "-I{}",
      build_path.join("generated").join("pico_base").display()
    ),
    format!(
      "-I{}",
      Path::new(PICO_SDK_PATH)
        .join("src")
        .join("boards")
        .join("include")
        .display()
    ),
    format!("-I{}", project_path.display()),
  ];
  let rp2_common = Path::new(PICO_SDK_PATH)
    .join("src")
    .join("rp2_common")
    .read_dir()
    .expect("rp2_common read_dir call failed");
  let common = Path::new(PICO_SDK_PATH)
    .join("src")
    .join("common")
    .read_dir()
    .expect("common read_dir call failed");
  let rp2040 = Path::new(PICO_SDK_PATH)
    .join("src")
    .join("rp2040")
    .read_dir()
    .expect("common read_dir call failed");

  args.extend(
    rp2_common
      .filter_map(Result::ok)
      .filter(|entry| entry.file_type().is_ok_and(|file_type| file_type.is_dir()))
      .map(|entry| format!("-I{}", entry.path().join("include").display())),
  );
  args.extend(
    common
      .filter_map(Result::ok)
      .filter(|entry| entry.file_type().is_ok_and(|file_type| file_type.is_dir()))
      .map(|entry| format!("-I{}", entry.path().join("include").display())),
  );
  args.extend(
    rp2040
      .filter_map(Result::ok)
      .filter(|entry| entry.file_type().is_ok_and(|file_type| file_type.is_dir()))
      .map(|entry| format!("-I{}", entry.path().join("include").display())),
  );

  if !build_path.exists() {
    fs::create_dir(build_path).unwrap();
  } else if build_path.read_dir().unwrap().count() <= 2 {
    Command::new("cmake")
      .arg("..")
      .current_dir(build_path)
      .output()
      .unwrap();
  }

  let bindings = bindgen::Builder::default()
    .use_core()
    .generate_inline_functions(true)
    .ctypes_prefix("crate::ctypes")
    .disable_untagged_union()
    .prepend_enum_name(false)
    .layout_tests(false)
    .header("wrapper.h")
    .clang_args(args)
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .generate()
    .expect("Unable to generate bindings");

  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");

  let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  match cbindgen::generate(&crate_dir) {
    Ok(gen) => gen,
    Err(e) => match e {
      // Ignore syntax errors because those will be handled later on by cargo build.
      cbindgen::Error::ParseSyntaxError {
        crate_name: _,
        src_path: _,
        error: _,
      } => return,
      _ => panic!("{:?}", e),
    },
  }
  .write_to_file(format!("../{PKG_NAME}.h"));
}
