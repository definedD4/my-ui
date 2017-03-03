#![feature(ptr_eq)]

extern crate glutin;

mod primitives;
mod tree;
mod window;
mod element;

use std::fmt::Display;

fn main() {
    let window = window::WindowBuilder::new()
        .with_title("My new window!")
        .build();
}