<div align="center">
  <img src="./assets/logo.png">

<br>
Rmap is a port scanner written in Rust. It has some options, such as the target and ports to scan or the timeout.
</div>

# Index
* ### [🗳️ Installation](#🗳️-installation)
* ### [🖱️ Use](#🖱️-use)
* ### [📋 License](#📋-license)

# 🗳️ Installation

You must have Rust installed on your computer to be able to compile the code, either with the ***rustc*** compiler
```
git clone git@github.com:rxfatalslash/rmap.git
cd rmap/src
rustc main.rs
./main
```
Or the ***cargo*** project manager
```
git clone git@github.com:rxfatalslash/rmap.git
cd rmap
cargo build --release
cd target/release
```

# 🖱️ Use

This script works by passing it certain parameters and options.
```
$ ./rmap -h
Usage: rmap [OPTIONS] <TARGET>

Arguments:
  <TARGET>  IP address to scan, use , to scan one or more hosts, or enter an ip of type x.x.x.0 to scan all hosts on the network

Options:
  -p, --ports <PORTS>      Ports to scan, use , to scan one or more ports, - to scan a range between this values, _ to scan the entire port range [default: 1-1024]
  -t, --timeout <TIMEOUT>  Timeout in milliseconds, the default value is a random number between 0 and 60
  -h, --help               Print help
  -V, --version            Print version
```

# 📋 License
This project is licensed under the terms of the [GNU General Public License, version 3](https://www.gnu.org/licenses/gpl-3.0.html) (GPLv3).

## LICENSE SUMMARY
### Permissions:

* **FREEDOM TO USE:** You are free to use, modify, and distribute this software.

* **SOURCE CODE ACCESS:** You must provide access to the source code of any modified versions of the software under the same GPLv3 license.

### Conditions:

* **COPYLEFT:** Any derivative work must also be open-source and distributed under the GPLv3 license.

* **NOTICES:** When distributing the software, you must include a copy of the GPLv3 license and provide appropriate notices.

### Limitations:

* **NO WARRANTY:** The software is provided as-is with no warranties or guarantees.

* **LIABILITY:** The authors or copyright holders are not liable for any damages or issues arising from the use of the software.

<a href="https://www.gnu.org/licenses/gpl-3.0.html" target="_blank">
  <img src="https://upload.wikimedia.org/wikipedia/commons/9/93/GPLv3_Logo.svg" width="80" height="15" />
</a>