use serde::{Deserialize, Serialize};
use windows::Win32::{
    Foundation::{POINT, RECT},
    UI::WindowsAndMessaging::{WINDOWPLACEMENT, WINDOWPLACEMENT_FLAGS},
};

#[derive(Serialize, Deserialize)]
#[serde(remote = "WINDOWPLACEMENT")]
#[allow(non_snake_case)]
pub struct WINDOWPLACEMENTdef {
    pub length: u32,
    #[serde(with = "WINDOWPLACEMENT_FLAGSdef")]
    pub flags: WINDOWPLACEMENT_FLAGS,
    pub showCmd: u32,
    #[serde(with = "POINTdef")]
    pub ptMinPosition: POINT,
    #[serde(with = "POINTdef")]
    pub ptMaxPosition: POINT,
    #[serde(with = "RECTdef")]
    pub rcNormalPosition: RECT,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "POINT")]
pub struct POINTdef {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "WINDOWPLACEMENT_FLAGS")]
#[allow(non_camel_case_types)]
pub struct WINDOWPLACEMENT_FLAGSdef(pub u32);

#[derive(Serialize, Deserialize)]
#[serde(remote = "RECT")]
pub struct RECTdef {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
