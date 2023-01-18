use crate::libs::cli::Cli;
use crate::libs::data::{Command, DataStore};
use gilrs::{Axis, Button, Event, GamepadId, Gilrs};

pub struct GamepadInputTask {
    gilrs: Gilrs,
    active_gamepad: Option<GamepadId>,
    command: Command
}

impl GamepadInputTask {
    pub fn with_cli(_cli: &mut Cli) -> Self
    where
        Self: Sized,
    {
        let gilrs = Gilrs::new().unwrap();

        // Iterate over all connected gamepads
        for (_id, gamepad) in gilrs.gamepads() {
            println!("{} is {:?}", gamepad.name(), gamepad.power_info());
        }

        Self {
            gilrs,
            active_gamepad: None,
            command: Command {
                id: 0,
                forward_velocity: 0.0,
                left_velocity: 0.0,
                angular_velocity: 0.0,
                charge: false,
                kick: None,
                dribbler: 0.0,
            }
        }
    }

    pub fn run(&mut self, _data_store: &DataStore) -> Command {
        // Examine new events
        while let Some(Event { id, event, time }) = self.gilrs.next_event() {
            println!("{:?} New event from {}: {:?}", time, id, event);
            self.active_gamepad = Some(id);
        }

        // You can also use cached gamepad state
        if let Some(gamepad) = self.active_gamepad.map(|id| self.gilrs.gamepad(id)) {
            // Move Local Velocity
            if gamepad.value(Axis::LeftStickY).abs() > 0.2 {
                self.command.forward_velocity = gamepad.value(Axis::LeftStickY);
            } else {
                self.command.forward_velocity = 0.0;
            }

            if gamepad.value(Axis::LeftStickX).abs() > 0.2 {
                self.command.left_velocity = -gamepad.value(Axis::LeftStickX) * 2.0;
            } else {
                self.command.left_velocity = 0.0;
            }

            if gamepad.value(Axis::RightStickX).abs() > 0.1 {
                self.command.angular_velocity = gamepad.value(Axis::RightStickX) * -3.14;
            } else {
                self.command.angular_velocity = 0.0;
            }

            if gamepad.is_pressed(Button::LeftTrigger) {
                self.command.dribbler = 1.0;
            } else {
                self.command.dribbler = 0.0;
            }

            self.command.clone()
        } else {
            Command::default()
        }
    }
}
