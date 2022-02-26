/***********************************************************
* Copyright 2022 Yuki Onishi
*
* This software is released under the MIT license.
* http://opensource.org/licenses/mit-license.php
***********************************************************/

extern crate hidapi;

use hidapi::HidApi;
use std::{thread, time};

use patlite::{buzzer::*, light::*, utils::*};

fn main() {
    let device_info = find_device().unwrap();
    println!(
        "Patlite (USB-connected) is detected in {}",
        device_info.path().to_str().unwrap()
    );

    let api = HidApi::new().expect("Failed to create API instance");
    let device = api
        .open(device_info.vendor_id(), device_info.product_id())
        .expect("Failed to open the device");


    // turns on both the lights and buzzer
    let mut light_setting = LightSetting::new();
    light_setting.set_all(LightState::ON);
    control(
        &device,
        Repeat::TWICE,
        Mode::PATTERN2,
        (Scale::A6, Scale::B6),
        Some(&light_setting),
    )
    .expect("Failed to send a command");
    thread::sleep(time::Duration::from_secs(1));

    // turns off the buzzer but keeps the lights on
    control(
        &device,
        Repeat::FOREVER,
        Mode::STOP,
        (Scale::OFF, Scale::OFF),
        None,
    )
    .expect("Failed to send a command");
    thread::sleep(time::Duration::from_secs(3));

    // turns off the lights
    turn_off(&device)
    .expect("Failed to turn off");
}
