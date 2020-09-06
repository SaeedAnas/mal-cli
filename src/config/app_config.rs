use super::*;

#[derive(Clone)]
pub struct AppConfig {
    pub keys: KeyBindings,
    pub theme: Theme,
    pub behavior: BehaviorConfig,
    pub nsfw: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct Theme {}

impl Default for Theme {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Clone)]
pub struct KeyBindings {}

#[derive(Clone)]
pub struct BehaviorConfig {
    pub seek_milliseconds: u32,
    pub tick_rate_milliseconds: u64,
    pub show_loading_indicator: bool,
}

// TODO: get app config from file
impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        Ok(Self {
            theme: Theme::default(),
            keys: KeyBindings {},
            behavior: BehaviorConfig {
                seek_milliseconds: 1000,
                tick_rate_milliseconds: 250,
                show_loading_indicator: true,
            },
            nsfw: true,
        })
    }
}
