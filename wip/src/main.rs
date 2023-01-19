use rhai::{Engine, Scope, EvalAltResult};
use std::io;

fn main() {
    env_logger::init();
    run_app();
}



fn run_app() {
    pollster::block_on(wip::window::run());
}