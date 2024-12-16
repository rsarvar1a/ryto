use crate::prelude::*;

impl<'a> Season<'a> {
    /// Creates a new Season with the given contestants.
    pub fn new(m: &Vec<&str>, f: &Vec<&str>, worldview: &'a Vec<Vec<usize>>) -> Result<Season<'a>> {
        let n = Season::check_lengths(m, f)?;
        let (ms, fs) = Season::create_maps_for(m, f);
        let distribution = Season::create_distribution(n);
        let worlds = (0..n.factorial()).collect();

        Ok(Season {
            m: m.into_iter().map(|&s| s.into()).collect(),
            f: f.into_iter().map(|&s| s.into()).collect(),
            ms,
            fs,
            n,
            turn: 1,
            found: 0,
            distribution,
            worlds,
            worldview,
        })
    }

    /// Returns the size of the season (in terms of contestants per gender), provided the two genders are of equal size.
    fn check_lengths(m: &Vec<&str>, f: &Vec<&str>) -> Result<usize> {
        let (lhs, rhs) = (m.len(), f.len());
        match lhs == rhs {
            true => Ok(lhs),
            _ => Err(anyhow!(
                "Mismatched number of males and females: {} != {}",
                lhs,
                rhs
            )),
        }
    }

    /// Creates a uniform pairwise distribution for a bipartite matching over two sets of n elements each.
    fn create_distribution(n: usize) -> Array2<f32> {
        let initializer = (n as f32).powi(-1);
        Array2::from_elem((n, n), initializer)
    }

    /// Creates name-to-index maps to convert at every public callsite so that all internal operations and state can be represented numerically.
    fn create_maps_for(
        m: &Vec<&str>,
        f: &Vec<&str>,
    ) -> (HashMap<String, usize>, HashMap<String, usize>) {
        let ms = HashMap::from_iter(m.into_iter().enumerate().map(|(i, &name)| (name.into(), i)));
        let fs = HashMap::from_iter(f.into_iter().enumerate().map(|(i, &name)| (name.into(), i)));
        (ms, fs)
    }
}
