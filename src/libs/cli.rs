use crate::libs::tasks::inputs::vision::VisionInputTaskCli;
use crate::libs::tasks::outputs::sim_commands::SimCommandsOutputTaskCli;
use crate::libs::tasks::outputs::usb_commands::UsbCommandsOutputTaskCli;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// team color
    #[arg(short, long)]
    pub(crate) y: bool,

    /// is real or simulation
    #[arg(short, long)]
    pub real: bool,

    /// is game controller active
    #[arg(long = "gc")]
    pub game_controller: bool,

    #[command(flatten)]
    #[command(next_help_heading = "vision")]
    pub vision: VisionInputTaskCli,

    #[command(flatten)]
    #[command(next_help_heading = "sim commands")]
    pub sim_commands: SimCommandsOutputTaskCli,

    #[command(flatten)]
    #[command(next_help_heading = "usb commands")]
    pub usb_commands: UsbCommandsOutputTaskCli,
}
