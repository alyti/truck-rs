use anyhow::Result;
use serde::Serialize;
use std::fs;
use std::io::ErrorKind;
use std::io::Write;

#[derive(Serialize, Default)]
pub struct Config {
    #[serde(rename = "target.x86_64-unknown-linux-gnu")]
    target_linux_gnu: Target,
    #[serde(rename = "target.x86_64-unknown-linux-musl")]
    target_linux_musl: Target,
    #[serde(rename = "target.x86_64-apple-darwin")]
    target_macos_intel: Target,
    #[serde(rename = "target.aarch64-apple-darwin")]
    target_macos_arm: Target,
    build: Build,
}

#[derive(Serialize, Default)]
struct Target {
    linker: String,
    rustflags: Vec<String>,
}

#[derive(Serialize, Default)]
struct Build {
    #[serde(rename = "rustc-wrapper")]
    wrapper: String,
    rustflags: Vec<String>,
}

impl Config {
    pub fn def(&mut self, name: Option<&String>) -> bool {
        self.write_all(false, name).is_ok()
    }

    pub fn fast(&mut self, name: Option<&String>) -> bool {
        let linker = "/usr/bin/clang".to_string();
        let linker_flag_linux = vec![
            "-C".to_string(),
            "link-arg=--lld-path=/usr/bin/lld".to_string(),
        ];
        let linker_flag_mac = vec![
            "-C".to_string(),
            "link-arg=--lld-path=/usr/bin/zld".to_string(),
        ];

        self.target_linux_gnu = Target {
            linker: linker.clone(),
            rustflags: linker_flag_linux.clone(),
        };
        self.target_linux_musl = Target {
            linker: linker.clone(),
            rustflags: linker_flag_linux,
        };
        self.target_macos_intel = Target {
            linker: linker.clone(),
            rustflags: linker_flag_mac.clone(),
        };
        self.target_macos_arm = Target {
            linker,
            rustflags: linker_flag_mac,
        };

        self.write_all(false, name).is_ok()
    }

    pub fn faster(&mut self, name: Option<&String>) -> bool {
        let linker = "/usr/bin/clang".to_string();
        let linker_flag = vec![
            "-C".to_string(),
            "link-arg=--lld-path=/usr/bin/mold".to_string(),
        ];
        
         let linker_flag_mac = vec![
            "-C".to_string(),
            "link-arg=--lld-path=/usr/bin/zld".to_string(),
        ];

        self.target_linux_gnu = Target {
            linker: linker.clone(),
            rustflags: linker_flag.clone(),
        };
        self.target_linux_musl = Target {
            linker: linker.clone(),
            rustflags: linker_flag,
        };
        self.target_macos_intel = Target {
            linker: linker.clone(),
            rustflags: linker_flag_mac.clone(),
        };
        self.target_macos_arm = Target {
            linker,
            rustflags: linker_flag_mac,
        };
        self.write_all(false, name).is_ok()
    }

    pub fn faster_nightly(&mut self, name: Option<&String>) -> bool {
        if let Err(err) = fs::remove_file("rust-toolchain.toml") {
            if err.kind() == ErrorKind::NotFound {
            } else {
                println!("FILE_ACCESS denied");
                return false;
            }
        }
        let linker = "/usr/bin/clang".to_string();
        let linker_flag = vec![
            "-C".to_string(),
            "link-arg=--lld-path=/usr/bin/mold".to_string(),
        ];

        self.target_linux_gnu = Target {
            linker: linker.clone(),
            rustflags: linker_flag.clone(),
        };
        self.target_linux_musl = Target {
            linker: linker.clone(),
            rustflags: linker_flag.clone(),
        };
        self.target_macos_intel = Target {
            linker: linker.clone(),
            rustflags: linker_flag.clone(),
        };
        self.target_macos_arm = Target {
            linker,
            rustflags: linker_flag,
        };

        self.build = Build {
            wrapper: "sccache".to_string(),
            rustflags: vec!["-Z".to_string(), "share-generics=y".to_string()],
        };

        self.write_all(true, name).is_ok()
    }

    fn write_all(&self, tlc: bool, name: Option<&String>) -> Result<()> {
        let project_path = match name {
            Some(name) => std::env::current_dir().unwrap().join(name),
            None => std::env::current_dir().unwrap(),
        };
        let config_path = project_path.join(".cargo");
        if let Err(err) = std::fs::create_dir(&config_path) {
            if let ErrorKind::AlreadyExists = err.kind() {
            } else {
                return Err(anyhow::Error::new(err));
            }
        }
        if tlc {
            let tool_chain = toml::toml! {
                [toolchain]
                channel = "nightly"
            };
            let tool_chain = toml::to_string_pretty(&tool_chain)?;
            let mut toolchain_file = fs::File::create(&project_path.join("rust-toolchain.toml"))?;
            toolchain_file.write_all(tool_chain.as_bytes())?;
        }
        let config = toml::to_string_pretty(&self)?;
        let mut config_file = fs::File::create(&config_path.join("config.toml"))?;
        config_file.write_all(config.as_bytes())?;
        Ok(())
    }
}
