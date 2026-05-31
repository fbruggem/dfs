use crate::simulation::runtime::{Action, Context, scheduler::Scheduler};

pub struct FiFo {}

impl Scheduler for FiFo {
    fn decide(&mut self, cx: &Context) -> Action {
        let _ = cx;
        Action::Done
    }
}
