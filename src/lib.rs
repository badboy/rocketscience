#[derive(Debug, thiserror::Error)]
enum LaunchError {
    #[error("Failed to launch rocket")]
    RocketLaunch,
}

/// Which direction to point to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
}

/// A vessel consists of several parts,
/// such as the command pod, tanks or the engine.
struct Part {
    name: String,
    cost: u64,
    weight: u64,
}

/// A rocket we can launch into orbit.
struct Rocket {
    name: String,
    parts: Vec<Part>,
    total_cost: u64,
    total_weight: u64,
    steering: Option<Direction>,
}

impl Rocket {
    /// Construct a new named rocket.
    fn new(name: String) -> Self {
        Rocket {
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

uniffi_macros::include_scaffolding!("rocketscience");
