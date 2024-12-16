use crate::prelude::*;

impl<'a> Season<'a> {
    /// Fetches the corresponding id of a name from its idmap.
    pub(super) fn id(&self, k: &str, d: &HashMap<String, usize>) -> Result<usize> {
        d.get(k).cloned().context(format!("invalid name {k}"))
    }

    /// Returns a list of couples that are correct in all remaining worlds.
    pub fn known_couples(&self) -> Vec<(String, String)> {
        self.distribution
            // Take all couples in the bipartite table that are confirmed to be true down to f32 effects.
            .mapv(|p| (p - 1.0).abs() <= f32::MIN)
            .indexed_iter()
            // Convert each couple to their named representation.
            .filter_map(|((m, f), &b)| {
                if b {
                    Some((self.m[m].clone(), self.f[f].clone()))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Returns the number of worlds remaining on this season.
    pub fn num_worlds(&self) -> usize {
        self.worlds.len()
    }

    /// Returns the number of couples in this season.
    pub fn size(&self) -> usize {
        self.n
    }

    /// Get the worlds that are currently consistent with this season.
    pub fn worlds(&self) -> Vec<Vec<usize>> {
        self.worlds
            .iter()
            .map(|k| self.worldview[*k].clone())
            .collect()
    }
}
