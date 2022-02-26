/***********************************************************
* Copyright 2022 Yuki Onishi
*
* This software is released under the MIT license.
* http://opensource.org/licenses/mit-license.php
***********************************************************/

extern crate hidapi;

use crate::*;
use hidapi::{DeviceInfo, HidApi, HidDevice, HidError, HidResult};

pub fn find_device() -> Result<DeviceInfo, HidError> {
    match HidApi::new() {
        Ok(api) => {
            for device in api.device_list() {
                if device.vendor_id() == 0x191a && device.product_id() == 0x8003 {
                    return Ok((*device).clone());
                }
            }
            Err(HidError::HidApiError {
                message: "Patlite (USB-connected) is not found".to_string(),
            })
        }
        Err(error) => Err(error),
    }
}

pub fn control(
    device: &HidDevice,
    buzzer_repeat: buzzer::Repeat,
    buzzer_mode: buzzer::Mode,
    buzzer_scales: (buzzer::Scale, buzzer::Scale),
    light_setting: Option<&light::LightSetting>,
) -> HidResult<usize> {
    let report_id = vec![0x00];
    let header = vec![0x00, 0x00];
    let buzzer_command = vec![
        (buzzer_repeat as u8).rotate_left(4) | buzzer_mode as u8,
        (buzzer_scales.0 as u8).rotate_left(4) | buzzer_scales.0 as u8,
    ];
    let light_command = match light_setting {
        Some(v) => v.binary(),
        None => vec![0xFF, 0xFF, 0xF0], // keep current states
    };

    device.write(
        [report_id, header, buzzer_command, light_command]
            .concat()
            .as_slice(),
    )
}

pub fn turn_off(device: &HidDevice) -> HidResult<usize> {
    control(
        device,
        buzzer::Repeat::FOREVER,
        buzzer::Mode::STOP,
        (buzzer::Scale::OFF, buzzer::Scale::OFF),
        Some(&light::LightSetting::new()),
    )
}
