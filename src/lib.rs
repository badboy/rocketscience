use std::sync::{Arc, RwLock};
use std::time::Duration;

use async_std::future::{pending, timeout};

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


/// Async function that says something after a certain time.
#[uniffi::export]
async fn launch_after(rocket: Arc<Rocket>, ms: u64) -> Result<bool> {
    let never = pending::<()>();
    timeout(Duration::from_millis(ms), never).await.unwrap_err();
    rocket.launch()
}

/// A rocket we can launch into orbit.
#[derive(uniffi::Object)]
pub struct Rocket(RwLock<Inner>);

#[uniffi::export]
impl Rocket {
    #[uniffi::constructor]
    pub fn new(name: String) -> Arc<Self> {
        Arc::new(Self(RwLock::new(Inner::new(name))))
    }

    fn add(&self, part: Part) {
        let mut rocket = self.0.write().unwrap();
        rocket.add(part)
    }

    fn lock_steering(&self, direction: Direction) {
        let mut rocket = self.0.write().unwrap();
        rocket.lock_steering(direction);
    }

    fn launch(&self) -> Result<bool> {
        let rocket = self.0.read().unwrap();
        rocket.launch()
    }

    fn show(&self) -> String {
        let rocket = self.0.read().unwrap();
        rocket.show()
    }
}

struct Inner {
    name: String,
    parts: Vec<Part>,
    total_cost: i64,
    total_weight: i64,
    steering: Option<Direction>,
}

impl Inner {
    /// Construct a new named rocket.
    fn new(name: String) -> Self {
        Inner {
            name,
            parts: vec![],
            total_cost: 0,
            total_weight: 0,
            steering: None,
        }
    }

    /// Add a new part to the rocket.
    fn add(&mut self, part: Part) {
        self.total_cost += part.cost;
        self.total_weight += part.weight;
        self.parts.push(part);
    }

    /// Lock the steering into a single direction.
    fn lock_steering(&mut self, dir: Direction) {
        self.steering = Some(dir);
    }

    /// Try to launch the rocket into orbit.
    ///
    /// This will throw a `LaunchError` if the rocket is not pointing up.
    /// Returns `true` if the launch succeeded
    /// or `false` if the rocket was too heavy or too expensive.
    fn launch(&self) -> Result<bool> {
        match self.steering {
            None | Some(Direction::Down) => return Err(LaunchError::RocketLaunch),
            _ => {}
        }

        if self.total_cost < 2000 && self.total_weight < 5000 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Show the rocket's configuration.
    fn show(&self) -> String {
        format!(
            "Rocket({}) {{ cost: {}, weight: {}, #parts: {}, steering: {:?} }}",
            self.name,
            self.total_cost,
            self.total_weight,
            self.parts.len(),
            self.steering
        )
    }
}

type Result<T, E = LaunchError> = std::result::Result<T, E>;

uniffi::setup_scaffolding!();
