extern crate prost_build;

use std::path::{Path, PathBuf};

fn compile_packet(filename: &str, protos: &[impl AsRef<Path>], includes: &[impl AsRef<Path>]) {
    let mut build = prost_build::Config::new();

    build
        .default_package_filename(filename)
        .out_dir(PathBuf::from("src/libs/protobuf/"))
        .compile_protos(protos, includes)
        .expect(format!("Failed to compile {} protobuf files", filename).as_str());
}

fn main() {
    compile_packet("simulation_packet",
                   &[
                       "proto/simulation/ssl_simulation_control.proto",
                       "proto/simulation/ssl_simulation_robot_control.proto",
                       "proto/simulation/ssl_simulation_robot_feedback.proto"
                   ],
                   &["proto/simulation/"]);

    compile_packet("vision_packet",
                   &["proto/vision/messages_robocup_ssl_wrapper.proto"],
                   &["proto/vision"]);

    compile_packet("game_controller_packet",
                   &["proto/game_controller/ssl_gc_referee_message.proto"],
                   &["proto/game_controller"]);

    compile_packet("robot_packet",
                   &["proto/robot/protocol_robot_catie_2022.proto"],
                   &["proto/robot"]);
}
