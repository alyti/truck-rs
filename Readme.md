 # Truck
 For Supercharging cargo

 ## What does it do?
 It is based on the reddit comment https://www.reddit.com/r/rust/comments/ua09tc/comment/i5w7n6g/?utm_source=share&utm_medium=web2x&context=3 . It improves cargo build speed

 ## How does it perform compared to Flex?
 Both does the same thing but Truck also comes with mold support and uses lesser number of lines for achieiving the purpose

 ## Why does it exist in the first place?
 This is created for newbies for their first PR. This project is barebones and can be extended.(Like adding colored output or fixing mistakes in the Readme.md file and if the PR makes sense, I will push it). It is only for educational purpose.

 ## How does it work?
 It creates a .cargo/config.toml file and rust-toolchain.toml file which replaces default linker with lld(Linux) or zld(MacOS) and mold(linux) and ccache with sccache. OSee the reddit comment for more details)

 ## Requirements
 - sccache: cargo install sccache
 - zld(Mac Users): `brew install zld`
 - lld(Linux Users): Install using your package manager (For Arch users, `sudo pacman -Syu lld`)
 - mold(Linux Users): Install using your package manager (For Arch users, `sudo pacman -Syu mold`)
 - Rust Nightly(optional): But mandatory for -ffn option
 
 ## How to install
 `cargo install truck-rs`
 *Make sure $CARGO_HOME/bin is in path*
 
 ## Quick Start
 - `truck [TRUCK FLAGS]` Creates optimisations on existing projects
 - `truck new my_file [TRUCK FLAGS]` Similar to cargo new myfile but with truck optimisations
 - `truck init my_file [TRUCK FLAGS]` Similar to cargo init but with truck optimisations

 **Run cargo build or run** to see the magic

 ## Truck Flags
 - noflags: Default build
 - -f: Fast(uses zld and lld)
 - -ff: Faster than -f(uses mold)
 - -ffn: Fastest (uses mold and sccache)

 ## Warning
 - Like fleet, it doesn't work if you are not importing multiple crates.
 - Using truck replaces your existing .cargo/config.toml and .rust-toolchain.toml file.
 - Also, we don't guarrantee it will be perfect alternative for cargo since there is a reason cargo by default doesn't use mold or lld or zld or sccache. (Read the reddit comment for more details)

 ## LICENSE
 MIT

 **If you find anything that can impove Rust's performance, please add it in optimisations.md**
