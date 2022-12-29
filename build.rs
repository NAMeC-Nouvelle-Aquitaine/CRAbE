extern crate prost_build;

use std::path::PathBuf;

fn main() {
    let mut simulation_build = prost_build::Config::new();
    simulation_build
        .default_package_filename("simulation_packet")
        .out_dir(PathBuf::from("src/libs/protobuf/"));
    simulation_build
        .compile_protos(
            &[
                "proto/simulation/ssl_simulation_control.proto",
                "proto/simulation/ssl_simulation_robot_control.proto",
                "proto/simulation/ssl_simulation_robot_feedback.proto",
            ],
            &["proto/simulation/"],
        )
        .expect("Failed to compile simulation protobuf files");

    // Build Vision protobuf
    let mut vision_build = prost_build::Config::new();
    vision_build
        .default_package_filename("vision_packet")
        .out_dir(PathBuf::from("src/libs/protobuf/"));
    vision_build
        .compile_protos(
            &["proto/vision/messages_robocup_ssl_wrapper.proto"],
            &["proto/vision"],
        )
        .expect("Failed to compile vision protobuf files");

    // Build GameController protobuf
    let mut build_gc = prost_build::Config::new();
    build_gc
        .default_package_filename("game_controller_packet")
        .out_dir(PathBuf::from("src/libs/protobuf/"));
    build_gc
        .compile_protos(
            &["proto/game_controller/ssl_gc_referee_message.proto"],
            &["proto/game_controller"],
        )
        .expect("Failed to compile game_controller protobuf files");
}
