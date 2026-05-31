use crate::simulation::runtime::{Action, Context};

pub mod fifo;

pub trait Scheduler {
    fn decide(&mut self, cx: &Context) -> Action;
}
