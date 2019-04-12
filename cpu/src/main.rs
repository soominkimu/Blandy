extern crate num_cpus;

use std::mem;

fn main() {
    println!("size_of f32  : {}", mem::size_of::<f32>());
    println!("size_of f64  : {}", mem::size_of::<f64>());
    println!("size_of usize: {}", mem::size_of::<usize>());
    println!("CPUs         : {}", num_cpus::get());
}
