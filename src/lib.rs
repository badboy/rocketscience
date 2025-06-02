use std::sync::Arc;

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum LaunchError {
    #[error("Failed to launch rocket")]
    RocketLaunch,
}

/// Which direction to point to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum Direction {
    Up,
    Down,
}

/// A vessel consists of several parts,
/// such as the command pod, tanks or the engine.
#[derive(uniffi::Record)]
pub struct Part {
    pub name: String,
    pub cost: i64,
    pub weight: i64,
}

/// A rocket we can launch into orbit.
#[derive(uniffi::Object)]
pub struct Rocket;

#[uniffi::export]
impl Rocket {
    #[uniffi::constructor]
    pub fn new(_name: String) -> Arc<Self> {
        Arc::new(Self)
    }
}

uniffi::setup_scaffolding!();
