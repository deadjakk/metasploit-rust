
# Metasploit Rust Shim
Rust shim for the metasploit-framework

# Pre-requisites 
1. Install cargo, the rust toolchain
2. The `script` subcommand for cargo (you can install this using `cargo install cargo-script` once you have cargo installed.

# To Install

1. Replace the bridge.rb file located in <your base msf framework directory>/lib/msf/core/modules/external/bridge.rb
  with the file provided in this repo.
  
2. Place the example .crs file in ~/.msf4/modules/exploits/, or wherever you'd like to store your modules.

Note: the module file MUST use .crs as the extension, this is how the shim will determine this file.
For more information on setting up dependencies for cargo-script check [the repo for cargo-script](https://github.com/DanielKeep/cargo-script
  
## Working:
  
  ![rust-shim-working](https://user-images.githubusercontent.com/30613497/125356809-59d30580-e32c-11eb-8908-5da4a30299fb.png)
