use std::usize;

use indicatif::ProgressStyle;
use ndarray::parallel::prelude::IntoParallelRefIterator;

use crate::prelude::*;

const INDICATIF_TEMPLATE: &'static str =
    " {wide_bar} | {percent}% ({human_pos}/{human_len}) | {eta} remaining ({per_sec}) ";

impl<'a> Season<'a> {
    pub fn find_best_ceremony(&self) -> Result<Vec<Vec<CoupleOutput>>> {
        let best = self
            .find_best_ceremony_impl()
            .0
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|(m, f)| (self.m[m].clone(), self.f[f].clone()))
                    .collect()
            })
            .collect();

        Ok(best)
    }

    fn find_best_ceremony_impl(&self) -> (Vec<Vec<Couple>>, usize) {
        if self.worlds.len() == 0 {
            return (vec![], usize::MAX);
        }

        let results: Vec<(Vec<Couple>, usize)> = self
            .worlds
            .par_iter()
            .progress()
            .with_style(ProgressStyle::with_template(INDICATIF_TEMPLATE).unwrap())
            .map(|k| {
                let candidate = &self.worldview[*k];
                let strip = candidate
                    .clone()
                    .into_iter()
                    .enumerate()
                    .collect::<Vec<Couple>>();

                // We should iterate over beam values starting at the number of already-found couples,
                // because no world left in the worldview couild possibly match on fewer.
                let best_possible_worst_case = (self.found..self.n)
                    .map(|beams| {
                        let mut season = self.clone();
                        season
                            .apply_ceremony_impl(candidate, beams)
                            .and_then(|season| season.recalculate())
                            .map(|season| season.find_best_truth_impl(Some(&strip)).1)
                            .unwrap_or(usize::MAX)
                    })
                    .filter(|s| ![0, usize::MAX].contains(s))
                    .min()
                    .unwrap_or(usize::MAX);

                (strip, best_possible_worst_case)
            })
            .collect();

        let lowest_worst_case = results
            .iter()
            .map(|(_, score)| score)
            .filter(|score| ![0, usize::MAX].contains(score))
            .min()
            .cloned();

        let Some(lowest_score) = lowest_worst_case else {
            return (vec![], usize::MAX);
        };

        let best: Vec<Vec<Couple>> = results
            .into_iter()
            .filter(|(_, score)| ![0, usize::MAX].contains(score))
            .filter_map(|(candidate, score)| {
                if score == lowest_score {
                    Some(candidate)
                } else {
                    None
                }
            })
            .collect();

        (best, lowest_score)
    }

    pub fn find_best_truth(
        &self,
        constraint: Option<&Vec<CoupleInput>>,
    ) -> Result<Vec<CoupleOutput>> {
        let constraint: Option<Vec<Couple>> = constraint
            .map(|o| {
                o.iter()
                    .map(|(m, f)| {
                        Ok::<Couple, Error>((self.id(m, &self.ms)?, self.id(f, &self.fs)?))
                    })
                    .try_collect::<Couple, Vec<Couple>, Error>()
            })
            .transpose()?;

        let result = self
            .find_best_truth_impl(constraint.as_ref())
            .0
            .into_iter()
            .map(|(m, f)| (self.m[m].clone(), self.f[f].clone()))
            .collect();

        Ok(result)
    }

    fn find_best_truth_impl(&self, constraint: Option<&Vec<Couple>>) -> (Vec<Couple>, usize) {
        let all_couples: Vec<Couple> = (0..self.n).cartesian_product(0..self.n).collect();

        let candidates = if let Some(constraint) = constraint {
            constraint
        } else {
            // Just generate all couples, there's not really that many of them.
            &all_couples
        };

        if self.worlds.len() == 0 || candidates.len() == 0 {
            return (vec![], usize::MAX);
        }

        let results = candidates
            .iter()
            .map(|candidate| {
                // If a couple is already determined to either be or not be a couple, give it a marker.
                let prior = self.distribution[*candidate];
                let worlds = (prior * self.num_worlds() as f32) as usize;
                let worst = worlds.max(self.num_worlds() - worlds);

                if worst == usize::MAX {
                    return (candidate, usize::MAX);
                }

                (candidate, worst)
            })
            .collect::<Vec<(&Couple, usize)>>();

        let lowest_worst_case = results
            .iter()
            // Drop incoherent worlds (which either have no worlds or have the marker usize::MAX to indicate explicit contradiction).
            .filter_map(|(_, score)| {
                if ![0, usize::MAX].contains(score) {
                    Some(score)
                } else {
                    None
                }
            })
            .min()
            .cloned();

        let Some(lowest_score) = lowest_worst_case else {
            return (vec![], usize::MAX);
        };

        let best: Vec<Couple> = results
            .into_iter()
            // Keep all those results
            .filter_map(|(candidate, score)| {
                if [0, usize::MAX].contains(&score) || score == lowest_score {
                    Some(candidate)
                } else {
                    None
                }
            })
            .cloned()
            .collect();

        (best, lowest_score)
    }
}
