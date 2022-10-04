// TODO: Replace standard example with actual domain knowledge

use crate::{
    spaces::{real::Interval, ProductSpace, Surjection},
    Domain, Observation, Reward,
};

const X_MIN: f64 = -1.2;
const X_MAX: f64 = 0.6;

const V_MIN: f64 = -0.07;
const V_MAX: f64 = 0.07;

const FORCE_G: f64 = -0.0025;
const FORCE_CAR: f64 = 0.0015;

const HILL_FREQ: f64 = 3.0;

const REWARD_STEP: f64 = -1.0;
const REWARD_GOAL: f64 = 0.0;

const MIN_ACTION: f64 = -1.0;
const MAX_ACTION: f64 = 1.0;

pub struct EconomyPlayer {
    x: f64,
    v: f64,

    action_space: Interval,
}

impl EconomyPlayer {
    pub fn new(x: f64, v: f64) -> EconomyPlayer {
        EconomyPlayer {
            // TODO: Define state data
        }
    }

    fn update_state(&mut self, a: f64) {
        // TODO: Update state
    }
}

impl Default for EconomyPlayer {
    fn default() -> EconomyPlayer {
        EconomyPlayer::new()
    }
}

impl Domain for EconomyPlayer {
    type StateSpace = ProductSpace<Interval>;
    type ActionSpace = Interval;

    fn emit(&self) -> Observation<Vec<f64>> {
        // TODO: Emit the state vector
        // For MountainCar example:
        // if self.x >= X_MAX {
        //     Observation::Terminal(vec![self.x, self.v])
        // } else {
        //     Observation::Full(vec![self.x, self.v])
        // }
    }

    fn step(&mut self, action: &f64) -> (Observation<Vec<f64>>, Reward) {
        // TODO: define actionspace
        self.update_state(*action);

        let to = self.emit();
        // TODO: Calculate reward
        let reward = if to.is_terminal() {
            REWARD_GOAL
        } else {
            REWARD_STEP
        };

        (to, reward)
    }

    fn state_space(&self) -> Self::StateSpace {
        // TODO: Define state space
        ProductSpace::empty() + Interval::bounded(X_MIN, X_MAX) + Interval::bounded(V_MIN, V_MAX)
    }

    fn action_space(&self) -> Interval {
        // TODO: Define action space
        Interval::bounded(MIN_ACTION, MAX_ACTION)
    }
}
