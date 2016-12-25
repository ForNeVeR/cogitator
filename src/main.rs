extern crate winapi;

mod fwpuclnt;
mod wfp;

fn main() {
    println!("Setting up the session");
    wfp::open_session();
    println!("Session opened successfully");
}
