#![feature(ptr_eq)]

extern crate glutin;

mod primitives;
mod tree;
mod window;
mod element;
mod property;
mod render;

use std::fmt::Display;

fn main() {
    let window = window::WindowBuilder::new()
        .with_title("My new window!")
        .build();
}