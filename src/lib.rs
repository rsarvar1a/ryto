mod ayto;

pub mod prelude {
    use anyhow::Result as AnyhowResult;

    pub type Result<T> = AnyhowResult<T, Error>;

    pub use super::ayto::types::*;
    pub use super::ayto::worldview;
    pub use super::ayto::Season;

    pub use anyhow::{anyhow, Context, Error};
    pub use factorial::Factorial;
    pub use indicatif::ParallelProgressIterator;
    pub use itertools::Itertools;
    pub use log::{debug, error, info, trace, warn};
    pub use ndarray::prelude::*;
    pub use rand::{seq::IteratorRandom, thread_rng};
    pub use rayon::prelude::*;
    pub use tabled::{settings::*, Table};

    pub use std::collections::HashMap;
    pub use std::sync::Arc;
    pub use std::time::{Duration, Instant};
}
