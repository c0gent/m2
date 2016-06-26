#![feature(rustc_private)]

#[macro_use] extern crate glium;
extern crate glium_text;
// extern crate time;
extern crate bismit;
// extern crate find_folder;
// extern crate num;
// extern crate vecmath;
// extern crate rand;
// extern crate enamel;
extern crate genmesh;
extern crate obj;
// extern crate nalgebra;
// extern crate tobj;
extern crate cgmath;
extern crate arena;

mod util;
mod sim;
mod win;
mod octree;

fn main() {
    use std::thread;
    // use std::sync::mpsc;

    // let (result_tx, result_rx) = mpsc::channel();
    // let (control_tx, control_rx) = mpsc::channel();

    let th_win = thread::Builder::new().name("win".to_string()).spawn(move || {
        win::Window::open();
        // win::conrod::window_conrod::open(control_tx, result_rx);
    }).expect("Error creating 'win' thread");

    let th_sim = thread::Builder::new().name("sim".to_string()).spawn(move || {
        // cycle::CycleLoop::run(0, control_rx, result_tx);
    }).expect("Error creating 'sim' thread");

    if let Err(e) = th_win.join() { println!("th_win.join(): Error: '{:?}'", e); }
    if let Err(e) = th_sim.join() { println!("th_sim.join(): Error: '{:?}'", e); }
}

