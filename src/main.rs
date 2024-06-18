use walkdir::{WalkDir, DirEntry};
 use std::fs::File;
 use std::io::Write;
 use std::io::Error;


fn is_filetype(entry: &DirEntry, _type: &str) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| s.to_lowercase().ends_with(_type))
}

enum FileType {
    None,
    Rust,
    Haskell,
    Cpp,
    Python,
}

fn main() -> Result<(), Error> {
    let mut filetype = FileType::None;
    for entry in WalkDir::new(".") {
        let entry = entry.unwrap();
        if is_filetype(&entry, ".rs") {
            filetype = FileType::Rust;
        }
        if is_filetype(&entry, ".hs") {
            filetype = FileType::Haskell;
        }
        if is_filetype(&entry, ".cpp") {
            filetype = FileType::Cpp;
        }
        if is_filetype(&entry, ".py") {
            filetype = FileType::Python;
        }
    }
    let mut file = File::create("shell.nix")?;
    let shell = shell_nix(filetype);

    file.write_all(shell.as_bytes())?;

    let mut envrc = File::create(".envrc")?;
    envrc.write_all(b"use nix")?;
    Ok(())

}

fn shell_nix(lang: FileType) -> String {
    match lang {
        FileType::Rust => format!("{{ pkgs ? import <nixpkgs> {{}} }}:
    pkgs.mkShell {{
      # nativeBuildInputs is usually what you want -- tools you need to run
      nativeBuildInputs = with pkgs.buildPackages; [ 
        libgcc
        pkg-config
        openssl 
        ];
        PKG_CONFIG_PATH = \"${{pkgs.openssl.dev}}/lib/pkgconfig\";
  }}"),
        
        FileType::Cpp => format!("{{ pkgs ? import <nixpkgs> {{}} }}:
        pkgs.mkShell {{
          # nativeBuildInputs is usually what you want -- tools you need to run
          nativeBuildInputs = with pkgs.buildPackages; [ 
                libgcc
            ];
      }}"),
        FileType::Haskell => format!("{{ pkgs ? import <nixpkgs> {{}} }}:
        pkgs.mkShell {{
          # nativeBuildInputs is usually what you want -- tools you need to run
          nativeBuildInputs = with pkgs.buildPackages; [ 
                haskell.compiler.native-bignum.ghcHEAD
            ];
      }}"),
      FileType::Python => format!("{{ pkgs ? import <nixpkgs> {{}} }}:
    pkgs.mkShell {{
      # nativeBuildInputs is usually what you want -- tools you need to run
      nativeBuildInputs = with pkgs.buildPackages; [ 
            python3
            python311Packages.numpy
            python311Packages.pandas
            python311Packages.scipy
            python311Packages.matplotlib
            python311Packages.seaborn
            python311Packages.scikit-learn
            python311Packages.statsmodels
        ];
  }}"),
        FileType::None => String::from("")
    }
}