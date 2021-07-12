# Metasploit Rust Shim
Rust shim for the metasploit-framework

# Pre-requisites 
1. Cargo install 
2. The `script` subcommand for cargo (you can install this using `cargo install cargo-script` once you have cargo installed.

# To Install

1. Replace the bridge.rb file located in <your base msf framework directory>/lib/msf/core/modules/external/bridge.rb
  with the file provided in this repo.
  
2. Place the example .crs file in ~/.msf4/modules/exploits/, or whever you'd like
