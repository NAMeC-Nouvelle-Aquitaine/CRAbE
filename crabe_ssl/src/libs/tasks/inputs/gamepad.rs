use crate::libs::cli::Cli;
use crate::libs::data::{Command, DataStore, Kick};
use gilrs::{Axis, Button, Event, GamepadId, Gilrs};

pub struct GamepadInputTask {
    gilrs: Gilrs,
    active_gamepad: Option<GamepadId>,
    is_charging: bool,
}

impl GamepadInputTask {
    pub fn with_cli(_cli: &Cli) -> Self
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
            is_charging: false,
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
            let mut command = Command::default();

            command.id = 5; // TODO : Make id changeable

            // Move Local Velocity
            if gamepad.value(Axis::LeftStickY).abs() > 0.2 {
                command.forward_velocity = gamepad.value(Axis::LeftStickY);
            }

            if gamepad.value(Axis::LeftStickX).abs() > 0.2 {
                command.left_velocity = -gamepad.value(Axis::LeftStickX) * 2.0;
            }

            if gamepad.value(Axis::RightStickX).abs() > 0.1 {
                command.angular_velocity = gamepad.value(Axis::RightStickX) * -3.14;
            }

            if gamepad.is_pressed(Button::LeftTrigger) {
                command.dribbler = 1.0;
            }

            if gamepad.is_pressed(Button::North) {
                self.is_charging = !self.is_charging;
            }

            command.charge = self.is_charging;

            // TODO : Handle power !
            if gamepad.is_pressed(Button::RightTrigger2) {
                command.kick = Some(Kick::StraightKick { power: 1.0 });
            } else if gamepad.is_pressed(Button::LeftTrigger2) {
                command.kick = Some(Kick::ChipKick { power: 1.0 });
            }

            command
        } else {
            Command::default()
        }
    }
}
