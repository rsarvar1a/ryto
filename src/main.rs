use ayto_rs::prelude::*;
use clap::*;
use std::io::{stdin, stdout, Write};

#[derive(Clone, Debug, Parser)]
#[command(version, about, long_about = None)]
struct Root {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Clone, Debug, Subcommand)]
enum Commands {
    NewGame(NewGameArgs),
    NextTurn,
    Print(PrintArgs),
    Recalculate,
    ApplyCeremony(ApplyCeremonyArgs),
    ApplyTruthBooth(ApplyTruthBoothArgs),
    BestCeremony(BestCeremonyArgs),
    BestTruthBooth(BestTruthBoothArgs),
    Spread(SpreadArgs),
    Worlds,
}

impl std::fmt::Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Commands::NewGame(_) => "new-game",
            Commands::NextTurn => "next-turn",
            Commands::Print(_) => "print",
            Commands::Recalculate => "recalculate",
            Commands::ApplyCeremony(_) => "apply-ceremony",
            Commands::ApplyTruthBooth(_) => "apply-truth-booth",
            Commands::BestCeremony(_) => "best-ceremony",
            Commands::BestTruthBooth(_) => "best-truth-booth",
            Commands::Spread(_) => "spread",
            Commands::Worlds => "worlds",
        };
        write!(f, "{repr}")
    }
}

#[derive(Clone, Debug, Args)]
struct ApplyCeremonyArgs {
    beams: usize,
    names: Vec<String>,

    #[arg(short, long)]
    offline: bool,
}

#[derive(Clone, Debug, Args)]
struct ApplyTruthBoothArgs {
    m: String,
    f: String,

    #[arg(short, long)]
    incorrect: bool,

    #[arg(short, long)]
    offline: bool,
}

#[derive(Clone, Debug, Args)] 
struct BestCeremonyArgs {
    #[arg(short, long)]
    naive: bool,
}

#[derive(Clone, Debug, Args)]
struct BestTruthBoothArgs {
    #[arg(short, long)]
    constrain: bool,
}

#[derive(Clone, Debug, Args)]
struct NewGameArgs {
    n: usize,
    names: Vec<String>,
}

#[derive(Clone, Debug, Args)]
struct PrintArgs {
    #[arg(short, long)]
    probabilities: bool,
}

#[derive(Clone, Debug, Args)]
struct SpreadArgs {
    names: Vec<String>,
}

fn main() -> () {
    pretty_env_logger::init();
    println!();

    if let Err(e) = _main() {
        println!("{e}");
    }
}

fn _main() -> Result<()> {
    let mut season: Option<Season> = None;
    let mut constraint: Option<Vec<CoupleOutput>> = None;
    let mut view: Vec<Vec<usize>>;

    loop {
        let input = _read()?;

        if input.trim().len() == 0 || input.trim().starts_with("#") {
            continue;
        }

        let mut cmdline = vec![""];
        cmdline.extend(input.split_ascii_whitespace());
        let try_parse = Root::try_parse_from(cmdline);
        let Ok(cmd) = try_parse else {
            println!("\n{}", try_parse.err().unwrap().to_string());
            continue;
        };

        'outer: {
            let start = Instant::now();

            match &cmd.cmd {
                Commands::NewGame(NewGameArgs { n, names }) => {
                    if names.len() != 2 * n {
                        println!(
                            "err: expected {} names; received {} instead.",
                            2 * n,
                            names.len()
                        );
                        break 'outer;
                    }

                    let chunks = names.chunks(*n).collect::<Vec<_>>();
                    let (m, f) = (chunks[0], chunks[1]);
                    let (m, f) = (
                        m.iter().map(|n| n.as_str()).sorted().collect(),
                        f.iter().map(|n| n.as_str()).sorted().collect(),
                    );

                    season = None;
                    view = (0..n.factorial())
                        .map(|k| worldview::generate(*n, k))
                        .collect();
                    let r = Season::new(&m, &f, &view);

                    match r {
                        Ok(s) => {
                            season = Some(s);
                        }
                        Err(e) => {
                            println!("err: {e}");
                        }
                    };
                }
                Commands::NextTurn => {
                    let Some(season) = season.as_mut() else {
                        _err_no_season();
                        break 'outer;
                    };

                    if let Err(e) = season.next_turn() {
                        println!("err: {e}");
                    }
                }
                Commands::Print(
                    PrintArgs { probabilities }
                ) => {
                    let Some(season) = season.as_mut() else {
                        _err_no_season();
                        break 'outer;
                    };

                    season.pretty_print(! *probabilities);
                }
                Commands::Recalculate => {
                    let Some(season) = season.as_mut() else {
                        _err_no_season();
                        break 'outer;
                    };

                    if let Err(e) = season.recalculate() {
                        println!("err: {e}");
                        break 'outer;
                    }
                }
                Commands::ApplyCeremony(ApplyCeremonyArgs {
                    beams,
                    names,
                    offline,
                }) => {
                    let Some(season) = season.as_mut() else {
                        _err_no_season();
                        break 'outer;
                    };

                    let (m, f) = (
                        names.iter().step_by(2).cloned().collect::<Vec<String>>(),
                        names
                            .iter()
                            .skip(1)
                            .step_by(2)
                            .cloned()
                            .collect::<Vec<String>>(),
                    );

                    if m.len() != f.len() {
                        println!("err: expected an even number of names.");
                        break 'outer;
                    }

                    let couples_owned: Vec<CoupleOutput> = m.into_iter().zip(f).collect();
                    let couples: Vec<CoupleInput> = couples_owned
                        .iter()
                        .map(|(m, f)| (m.as_str(), f.as_str()))
                        .collect();

                    if let Err(e) = season.apply_ceremony(couples, *beams, !*offline) {
                        println!("err: {e}");
                    }

                    constraint = Some(couples_owned.clone());
                }
                Commands::ApplyTruthBooth(ApplyTruthBoothArgs {
                    m,
                    f,
                    incorrect,
                    offline,
                }) => {
                    let Some(season) = season.as_mut() else {
                        _err_no_season();
                        break 'outer;
                    };

                    let couple = (m.as_str(), f.as_str());

                    if let Err(e) = season.apply_truth(couple, !*incorrect, !*offline) {
                        println!("err: {e}");
                    }
                }
                Commands::BestCeremony(BestCeremonyArgs { naive }) => {
                    let Some(season) = season.as_mut() else {
                        _err_no_season();
                        break 'outer;
                    };

                    let r = season.find_best_ceremony(*naive);

                    match r {
                        Ok(candidates) => {
                            if let Some(best_ceremony) = candidates.get(0) {
                                let ceremony_input: Vec<CoupleInput> = best_ceremony.iter().map(|(m, f)| {
                                    (m.as_str(), f.as_str())
                                }).collect();
                                if let Err(e) = season.speculate(ceremony_input.clone(), "best possible ceremony") {
                                    println!("err: {e}");
                                }
                                println!("");
                                if let Err(e) = season.spread(ceremony_input) {
                                    println!("err: {e}");
                                }
                            } else {
                                println!("There are no worlds; did you enter a contradiction?")
                            }
                        }
                        Err(e) => {
                            println!("err: {e}");
                        }
                    }
                }
                Commands::BestTruthBooth(BestTruthBoothArgs { constrain }) => {
                    let Some(season) = season.as_mut() else {
                        _err_no_season();
                        break 'outer;
                    };

                    let backing = constraint.clone();
                    let this_constraint = backing
                        .as_ref()
                        .map(|v| v.iter().map(|(m, f)| (m.as_str(), f.as_str())).collect());
                    let this_constraint = this_constraint.as_ref();

                    let r = season.find_best_truth(if *constrain { this_constraint } else { None });

                    match r {
                        Ok(candidates) => {
                            if let Some((m, f)) = candidates.get(0) {
                                let couple_input = (m.as_str(), f.as_str()); 
                                if let Err(e) = season.speculate(vec![couple_input], "best possible truth booth") {
                                    println!("err: {e}");
                                }
                            } else {
                                println!("There are no couples; did you enter a contradiction?");
                            }
                        }
                        Err(e) => {
                            println!("err: {e}");
                        }
                    }
                },
                Commands::Spread(SpreadArgs { names }) => {
                    let Some(season) = season.as_mut() else {
                        _err_no_season();
                        break 'outer;
                    };

                    let (m, f) = (
                        names.iter().step_by(2).cloned().collect::<Vec<String>>(),
                        names
                            .iter()
                            .skip(1)
                            .step_by(2)
                            .cloned()
                            .collect::<Vec<String>>(),
                    );

                    let n = season.size();
                    if m.len() != n || f.len() != n {
                        println!("err: wrong number of matches; expected {n}.");
                        break 'outer;
                    }

                    let couples_owned: Vec<CoupleOutput> = m.into_iter().zip(f).collect();
                    let couples: Vec<CoupleInput> = couples_owned
                        .iter()
                        .map(|(m, f)| (m.as_str(), f.as_str()))
                        .collect();

                    if let Err(e) = season.spread(couples) {
                        println!("err: {e}");
                    }
                },
                Commands::Worlds => {
                    let Some(season) = season.as_mut() else {
                        _err_no_season();
                        break 'outer;
                    };

                    let worlds = season.worlds();
                    println!("worlds:");
                    for (i, w) in worlds.iter().enumerate() {
                        println!("{:>3}. {}", i + 1, w.iter().map(|(m, f)| format!("{m} & {f}")).join(", "));
                    }
                }
            }

            let elapsed = (Instant::now() - start).as_secs_f32();
            debug!("command `{}` took {elapsed:.2}s", &cmd.cmd);
            println!();
        }
    }
}

fn _err_no_season() -> () {
    println!("err: no season; use `newgame` to create one!");
}

fn _print(prompt: &str) -> Result<()> {
    print!("{prompt}");
    stdout().flush()?;
    Ok(())
}

fn _read() -> Result<String> {
    let mut iobuf = String::new();
    if stdin().read_line(&mut iobuf)? == 0 {
        return Err(anyhow!("eof"));
    }
    Ok(iobuf.trim().to_owned())
}
