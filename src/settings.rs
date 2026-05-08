use std::{env::current_exe, fs, path::PathBuf};

use crate::settings::{
    appearance_settings::AppearanceSettings, controls_settings::ControlsSettings,
    handling_settings::HandlingSettings, rules_settings::RulesSettings,
};

pub mod appearance_settings;
mod controls_settings;
mod handling_settings;
pub mod rules_settings;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Settings {
    pub help: String,
    pub controls: ControlsSettings,
    pub handling: HandlingSettings,
    pub rules: RulesSettings,
    pub appearance: AppearanceSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            help: "\
Settings file.
You can change values here. If you break something this file will
be copied with an 'old' suffix and the default one will replace this.
"
            .to_string(),
            controls: Default::default(),
            handling: Default::default(),
            rules: Default::default(),
            appearance: Default::default(),
        }
    }
}

impl Settings {
    const SETTINGS_FILE_NAME: &str = "ttetris_settings.toml";
    const OLD_SETTINGS_FILE_NAME: &str = "ttetris_settings.old.toml";

    fn get_settings_file_path() -> PathBuf {
        let mut current_exe_path = current_exe().unwrap();
        current_exe_path.pop();
        current_exe_path.push(Self::SETTINGS_FILE_NAME);
        current_exe_path
    }

    fn get_old_settings_file_path() -> PathBuf {
        let mut current_exe_path = current_exe().unwrap();
        current_exe_path.pop();
        current_exe_path.push(Self::OLD_SETTINGS_FILE_NAME);
        current_exe_path
    }

    pub fn load_from_file() -> Option<Settings> {
        let settings_str = fs::read_to_string(Self::get_settings_file_path()).ok()?;
        let settings_result = toml::from_str(&settings_str) as Result<Settings, _>;
        if settings_result.is_err() {
            dbg!(settings_result.err());
        }
        let settings = toml::from_str(&settings_str).ok();
        if settings.is_none() {
            let _ = fs::copy(
                Self::get_settings_file_path(),
                Self::get_old_settings_file_path(),
            );
        }
        settings
    }
    pub fn save_to_file(&self) {
        let settings_str = toml::to_string_pretty(self).unwrap();
        let _ = fs::write(Self::get_settings_file_path(), settings_str);
    }
}
