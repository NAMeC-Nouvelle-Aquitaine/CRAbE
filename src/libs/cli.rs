use crate::inputs_outputs::simulation_client::SimulationClientCli;
use crate::inputs_outputs::usb_client::USBClientCli;
use crate::inputs_outputs::vision::VisionCli;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// team color
    #[arg(short = 'y', long)]
    pub(crate) yellow: bool,

    /// is real or simulation
    #[arg(short, long)]
    pub real: bool,

    /// is game controller active
    #[arg(long = "gc")]
    pub game_controller: bool,

    #[command(flatten)]
    #[command(next_help_heading = "vision")]
    pub vision: VisionCli,

    #[command(flatten)]
    #[command(next_help_heading = "sim commands")]
    pub sim_commands: SimulationClientCli,

    #[command(flatten)]
    #[command(next_help_heading = "usb commands")]
    pub usb_commands: USBClientCli,
}
