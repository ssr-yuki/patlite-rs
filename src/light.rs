/***********************************************************
* Copyright 2022 Yuki Onishi
*
* This software is released under the MIT license.
* http://opensource.org/licenses/mit-license.php
***********************************************************/

/// Light colors
#[derive(Copy, Clone, Debug)]
pub enum Color {
    RED,
    YELLOW,
    GREEN,
    BLUE,
    WHITE,
}

/// Lighting patterns
#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum LightState {
    OFF,
    ON,
    BLINK_FAST,
    BLINK_SLOW,
    FLASH_DOUBLE,
    FLASH,
}

/// A struct to hold the light states
pub struct LightSetting {
    states: [LightState; 5],
}

impl LightSetting {
    /// Creates a struct in which all lights are set to OFF.
    pub fn new() -> Self {
        LightSetting {
            states: [LightState::OFF; 5],
        }
    }

    /// Gets the current state of a specified light.
    pub fn get(&self, color: Color) -> LightState {
        self.states[color as usize]
    }

    /// Sets a state of a specified light.
    pub fn set(&mut self, color: Color, state: LightState) {
        self.states[color as usize] = state;
    }

    /// Sets all light to once state.
    pub fn set_all(&mut self, state: LightState) {
        self.states = [state; 5]
    }

    /// Generates a binary representation used in sending commands.
    pub fn binary(&self) -> Vec<u8> {
        let b5 = (self.states[Color::RED as usize] as u8).rotate_left(4)
            | self.states[Color::YELLOW as usize] as u8;
        let b6 = (self.states[Color::GREEN as usize] as u8).rotate_left(4)
            | self.states[Color::BLUE as usize] as u8;
        let b7 = (self.states[Color::WHITE as usize] as u8).rotate_left(4);
        vec![b5, b6, b7]
    }
}

impl Default for LightSetting {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn set_red() {
    let mut setting = LightSetting::new();
    setting.set(Color::RED, LightState::ON);
    assert_eq!(setting.get(Color::RED) as u8, LightState::ON as u8);
}

#[test]
fn check_binary_conversion() {
    let mut setting = LightSetting::new();
    setting.set(Color::RED, LightState::ON);
    setting.set(Color::YELLOW, LightState::BLINK_FAST);
    setting.set(Color::GREEN, LightState::BLINK_SLOW);
    setting.set(Color::BLUE, LightState::FLASH_DOUBLE);
    setting.set(Color::WHITE, LightState::FLASH);
    assert_eq!(setting.binary(), vec![0x12, 0x34, 0x50]);
}
