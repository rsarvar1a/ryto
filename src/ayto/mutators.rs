use std::usize;

use crate::prelude::*;

impl<'a> Season<'a> {
    /// Applies a given list of couples and a number of correct couples as a matching ceremony.
    pub fn apply_ceremony(
        &mut self,
        couples: Vec<CoupleInput>,
        beams: usize,
        recompute: bool,
    ) -> Result<&mut Self> {
        let mapped: Vec<usize> = couples
            .into_iter()
            // Validate the input to ensure we can convert from names to ids.
            .map(|(m, f)| {
                Ok::<(usize, usize), Error>((self.id(m, &self.ms)?, self.id(f, &self.fs)?))
            })
            .try_collect::<(usize, usize), Vec<(usize, usize)>, Error>()?
            // Process the couples, and convert the list into a single world representation.
            .into_iter()
            .sorted_by_key(|(m, _)| *m)
            .map(|(_, f)| f)
            .collect();

        self.apply_ceremony_impl(&mapped, beams)?;

        match recompute {
            true => self.recalculate(),
            _ => Ok(self),
        }
    }

    /// Applies a given list of couples and a number of correct couples as a matching ceremony.
    ///
    /// Here, `couples` is a match strip of the form `m = [f_1, f_2, ..., f_n]` where `match_of(m_i) = m[m_i]`.
    /// However, couples that are already found are set to n + 1 instead, to simulate a non-match. This is necessary
    /// for blackout detection.
    pub(super) fn apply_ceremony_impl(
        &mut self,
        couples: &Vec<usize>,
        beams: usize,
    ) -> Result<&mut Self> {
        self.worlds
            // Keep only those indices for which the corresponding world matches the ceremonial world in exactly `beams` places.
            .retain(|&i| {
                unsafe { self.worldview.get_unchecked(i) }
                    .iter()
                    .zip(couples)
                    .filter(|(&lhs, &rhs)| lhs == rhs)
                    .count()
                    == beams
            });

        Ok(self)
    }

    /// Sends a given couple to the truth booth and applies the given outcome.
    pub fn apply_truth(
        &mut self,
        couple: CoupleInput,
        correct: bool,
        recompute: bool,
    ) -> Result<&mut Self> {
        let (m, f) = couple;

        let couple = (self.id(m, &self.ms)?, self.id(f, &self.fs)?);

        self.apply_truth_impl(couple, correct)?;

        match recompute {
            true => self.recalculate(),
            _ => Ok(self),
        }
    }

    // Sends a given couple to the truth booth and applies the given outcome.
    pub(super) fn apply_truth_impl(&mut self, couple: Couple, correct: bool) -> Result<&mut Self> {
        let (m, f) = couple;

        self.worlds
            .retain(|&i| { 
                let check = unsafe { *self.worldview.get_unchecked(i).get_unchecked(m) };
                (check == f) == correct
            });

        Ok(self)
    }

    /// Ticks over to the next turn, and errors to signal that you have lost.
    pub fn next_turn(&mut self) -> Result<&mut Self> {
        self.turn += 1;
        match self.turn > self.n {
            true => Err(anyhow!("out of turns!")),
            _ => Ok(self),
        }
    }

    /// Recomputes the bipartite probability table for this season.
    pub fn recalculate(&mut self) -> Result<&mut Self> {
        let nw = self.worlds.len() as f32;
        
        let mut view = self.distribution.view_mut();
        (0..self.n).for_each(|m| {
            view.slice_mut(s![m, ..]).fill(0.0);
            self.worlds.iter().for_each(|&k| {
                let f = unsafe { *self.worldview.get_unchecked(k).get_unchecked(m) };
                view[(m, f)] += 1.0;
            });
        });
        view /= nw;

        self.found = self.distribution.iter().filter(|&e| *e == 1.0).count();

        Ok(self)
    }
}
