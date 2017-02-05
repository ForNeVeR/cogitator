extern crate winapi;

mod windivert;

fn main() {
    let handle = windivert::open("true");
}
