# CRAbE - Central AI of NAMeC

## Installation

### Linux

#### Dépendances

Installer les dépendances suivantes :
```bash
sudo apt install protobuf-compiler curl
```

Installer rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### GrSim

Installer GrSim
```bash
sudo apt install git build-essential cmake pkg-config \
                   libqt5opengl5-dev libgl1-mesa-dev libglu1-mesa-dev \
                   libprotobuf-dev protobuf-compiler libode-dev libboost-dev
```

Clone the repo
```bash
mkdir -p ~/project/ssl
cd ~/project/ssl/
git clone https://github.com/RoboCup-SSL/grSim.git
cd grSim
```

Create a build directory
```bash
mkdir build
cd build
```

Build
```bash
cmake -DCMAKE_INSTALL_PREFIX=/usr/local ..
make
sudo make install
```

Launch grSim
```bash
grSim
```

#### CRAbE

Clone the CRAbE repository in 
```
cd ~/project/ssl/
git clone git@github.com:NAMeC-SSL/CRAbE.git
```

Launch the IA with `cargo`
```bash
cd CRAbE/
cargo run --bin main
```

To test that the Rust framework works, you can quickly edit the file `src/bin/main.rs` and uncomment line 34,
and add the following use statement at the top of the file

```rust
use software::libs::tasks::examples::move_to_ball::MoveToBallExampleTask;
```
Open grSim and launch the previous `cargo run`
The robots should be moving around weirdly, means it runs.
## MACOS

TODO

## Windows

TODO
