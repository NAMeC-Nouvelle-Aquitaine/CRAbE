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
grsim
```

#### CRAbE

Launch the IA
```bash
cargo run
```

## MACOS

TODO

## Windows

TODO