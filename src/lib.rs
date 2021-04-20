#[derive(Debug, thiserror::Error)]
enum LaunchError {
    #[error("Failed to launch rocket")]
    RocketLaunch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
}

struct Part {
    name: String,
    cost: u64,
    weight: u64,
}

struct Rocket {
    name: String,
    parts: Vec<Part>,
    total_cost: u64,
    total_weight: u64,
    steering: Option<Direction>,
}

impl Rocket {
    fn new(name: String) -> Self {
        Rocket {
            name,
            parts: vec![],
            total_cost: 0,
            total_weight: 0,
            steering: None,
        }
    }

    fn add(&mut self, part: Part) {
        self.total_cost += part.cost;
        self.total_weight += part.weight;
        self.parts.push(part);
    }

    fn lock_steering(&mut self, dir: Direction) {
        self.steering = Some(dir);
    }

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
