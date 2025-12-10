//! The `build.rs` script for this crate.

#[cfg(not(feature = "gen"))]
fn main() {}

#[cfg(all(feature = "gen", feature = "femtopb"))]
fn main() -> anyhow::Result<()> {
    use std::fs;
    let protobufs_dir = "src/protobufs";
    let target = "src/generated-no-std";
    fs::create_dir_all(target)?;

    let mut protos_vec = vec![];
    for entry in walkdir::WalkDir::new(protobufs_dir)
        .into_iter()
        .map(|e| e.unwrap())
        .filter(|e| {
            e.path()
                .extension()
                .is_some_and(|ext| ext.to_str().unwrap() == "proto")
        })
    {
        let path = entry.path();
        protos_vec.push(path.to_owned().to_string_lossy().into_owned());
    }
    protos_vec.sort();
    println!("{protos_vec:#?}");
    let vec_str: Vec<&str> = protos_vec.iter().map(|s| s.as_str()).collect();
    let protos = vec_str.as_slice();

    let mut config = femtopb_build::Config::new();
    config
        //.derive_defmt(true)
        .protos(protos)
        .target(target)
        .includes(&["src/protobufs"]);
    config.compile()
}

#[cfg(all(feature = "gen", not(feature = "femtopb")))]
fn main() -> std::io::Result<()> {
    let src_dir = "src/protobufs/";
    let gen_dir = "src/generated/";

    println!("cargo:rerun-if-changed={src_dir}");
    println!("cargo:rerun-if-changed={gen_dir}");

    // Allows protobuf compilation without installing the `protoc` binary
    match protoc_bin_vendored::protoc_bin_path() {
        Ok(protoc_path) => {
            if std::env::var("PROTOC").ok().is_some() {
                println!("Using PROTOC set in environment.");
            } else {
                println!("Setting PROTOC to protoc-bin-vendored version.");
                std::env::set_var("PROTOC", protoc_path);
            }
        }
        Err(err) => {
            println!("Install protoc yourself, protoc-bin-vendored failed: {err}");
        }
    }
    // Get sorted list of .proto files inside src_dir
    let mut protos: Vec<_> = walkdir::WalkDir::new(src_dir)
        .into_iter()
        .filter_map(|x| x.ok())
        .map(|x| x.into_path())
        .filter(|x| x.extension().is_some_and(|x| x == "proto"))
        .collect();
    protos.sort();

    let mut config = prost_build::Config::new();

    #[cfg(not(feature = "std"))]
    {
        config.btree_map(&["."]);
    }

    #[cfg(feature = "ts-gen")]
    {
        print!("ts-gen enabled");
        config.type_attribute(
            ".",
            "#[cfg_attr(feature = \"ts-gen\", derive(specta::Type))]",
        );
    }

    #[cfg(feature = "serde")]
    {
        print!("serde enabled");
        config.type_attribute(
            ".",
            "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]",
        );
        config.type_attribute(
            ".",
            "#[cfg_attr(feature = \"serde\", serde(rename_all = \"camelCase\"))]",
        );
        config.type_attribute(".", "#[allow(clippy::doc_lazy_continuation)]");
    }

    #[cfg(feature = "rkyv")]
    {
        print!("rkyv enabled");
        config.type_attribute(
            ".",
            "#[cfg_attr(feature = \"rkyv\", derive(rkyv::Serialize, rkyv::Deserialize, rkyv::Archive))]",
        );
        config.type_attribute(
            ".",
            "#[cfg_attr(feature = \"rkyv\", rkyv(compare(PartialEq),derive(Debug)))]",
        );
    }

    config.out_dir(gen_dir);
    config.compile_protos(&protos, &[src_dir])
}
