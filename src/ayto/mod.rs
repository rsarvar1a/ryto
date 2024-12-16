use crate::prelude::*;

mod accessors;
mod constructor;
mod io;
mod mutators;
mod solvers;
pub mod types;
pub mod worldview;

#[derive(Clone, Debug)]
pub struct Season<'a> {
    m: Vec<String>,
    f: Vec<String>,
    ms: HashMap<String, usize>,
    fs: HashMap<String, usize>,

    n: usize,
    turn: usize,
    found: usize,

    distribution: Array2<f32>,
    worlds: Vec<usize>,
    worldview: &'a Vec<Vec<usize>>,
}
