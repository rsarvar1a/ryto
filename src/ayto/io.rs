use std::iter;

use crate::prelude::*;
use tabled::builder::Builder;

impl<'a> Season<'a> {
    /// Pretty-prints a season.
    pub fn pretty_print(&self, as_counts: bool) -> () {
        let (turn, worlds) = (self.turn, self.worlds.len());
        let specifier = if worlds == 1 { "world" } else { "worlds" }; 

        println!(
            "Episode {turn} - {worlds} {specifier} remain\n{}",
            self.table(as_counts).with(Style::rounded()).with(Alignment::right())
        );
    }

    /// Coerces a value into its string representation.
    fn print_value(&self, e: f32, factor: f32, as_counts: bool) -> String {
        if e == factor {
            "♡".into()
        } else if e == 0.0 {
            ".".into()
        } else {
            match as_counts {
                true => (e as usize).to_string(),
                _ => format!("{:.1}%", e * 100.0),
            }
        }
    }

    /// Returns a table representing the current state of the season,
    /// in terms of how many worlds remain for each couple.
    pub fn table(&self, as_counts: bool) -> Table {
        let factor = if as_counts {
            self.worlds.len() as f32
        } else {
            1.0
        };
        let table = self.distribution.clone() * factor;

        let mut builder = Builder::new();
        let header = iter::once("".into()).chain(self.f.clone().into_iter());
        builder.push_record(header);
        self.m.iter().for_each(|m| {
            let index = self.id(m, &self.ms).unwrap();
            let data = table.slice(s![index, ..]).to_vec();
            let row = iter::once(m.into()).chain(
                data.into_iter()
                    .map(|e| self.print_value(e, factor, as_counts)),
            );
            builder.push_record(row);
        });

        builder.build()
    }
}
