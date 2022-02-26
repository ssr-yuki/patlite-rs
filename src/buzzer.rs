/***********************************************************
* Copyright 2022 Yuki Onishi
* 
* This software is released under the MIT license.
* http://opensource.org/licenses/mit-license.php
***********************************************************/

/// The number of times a buzzer blows
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum Repeat {
    FOREVER,
    ONCE,
    TWICE,
    THREE_TIMES,
    FOUR_TIMES,
    FIVE_TIMES,
    SIX_TIMES,
    SEVEN_TIMES,
    EIGHT_TIMES,
    NINE_TIMES,
    TEN_TIMES,
    ELEVEN_TIMES,
    TWELVE_TIMES,
    THIRTEEN_TIMES,
    FOURTEEN_TIMES,
    FIFTEEN_TIMES,
}

/// Buzzer patterns
#[derive(Copy, Clone)]
pub enum Mode {
    STOP,
    BLOWING,
    PATTERN1,
    PATTERN2,
    PATTERN3,
    PATTERN4,
}

/// Buzzer sound scales
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum Scale {
    OFF,
    A6,
    B_FLAT6,
    B6,
    C7,
    D_FLAT7,
    D7,
    E_FLAT7,
    E7,
    F7,
    G_FLAT7,
    G7,
    A_FLAT7,
    A7,
}
