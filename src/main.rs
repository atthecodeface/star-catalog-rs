use std::path::PathBuf;

use anyhow::anyhow;
use clap::{ArgMatches, Command};
use star_catalog::{hipparcos, Catalog, Star};

mod cmdline {
    use clap::{parser::ValuesRef, value_parser, Arg, ArgAction, ArgMatches, Command};

    //fp add_catalog_arg
    pub fn add_catalog_arg(cmd: Command) -> Command {
        cmd.arg(
            Arg::new("catalog")
                .long("catalog")
                .short('c')
                .required(true)
                .help("Which star catalog to load")
                .action(ArgAction::Set),
        )
    }
    pub fn catalog(matches: &ArgMatches) -> String {
        matches.get_one::<String>("catalog").unwrap().to_string()
    }

    //fp add_magnitude_arg
    pub fn add_magnitude_arg(cmd: Command) -> Command {
        cmd.arg(
            Arg::new("magnitude")
                .long("magnitude")
                .short('m')
                .help("Maximum magnitude")
                .value_parser(value_parser!(f32))
                .action(ArgAction::Set),
        )
    }
    pub fn magnitude(matches: &ArgMatches) -> f32 {
        *matches.get_one::<f32>("magnitude").unwrap_or(&6.0)
    }

    //fp add_names_arg
    pub fn add_names_arg(cmd: Command) -> Command {
        cmd.arg(
            Arg::new("names")
                .long("names")
                .short('n')
                .help("File containing names of id")
                .action(ArgAction::Set),
        )
    }
    pub fn names(matches: &ArgMatches) -> Option<String> {
        matches.get_one::<String>("names").map(|s| s.to_string())
    }

    //fp add_output_arg
    pub fn add_output_arg(cmd: Command) -> Command {
        cmd.arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .required(true)
                .help("Which star output to load")
                .action(ArgAction::Set),
        )
    }
    pub fn output(matches: &ArgMatches) -> String {
        matches.get_one::<String>("output").unwrap().to_string()
    }
    //fp add_stars_arg
    pub fn add_stars_arg(cmd: Command) -> Command {
        cmd.arg(
            Arg::new("stars")
                .help("Stars to interrogate")
                .action(ArgAction::Append),
        )
    }
    pub fn stars(matches: &ArgMatches) -> Option<ValuesRef<'_, String>> {
        matches.get_many::<String>("stars")
    }
}

fn main() -> Result<(), anyhow::Error> {
    let cmd = Command::new("star_catalog")
        .about("Star catlog")
        .version("0.1.0");

    let list_subcmd = Command::new("list").about("Lists the stars in the catalog");
    let find_subcmd = Command::new("find").about("Find stars in the catalog and display them");
    let find_subcmd = cmdline::add_stars_arg(find_subcmd);
    let write_subcmd = Command::new("write").about("Write out the catalog as JSON");
    let write_subcmd = cmdline::add_output_arg(write_subcmd);

    let cmd = cmdline::add_catalog_arg(cmd);
    let cmd = cmdline::add_magnitude_arg(cmd);
    let cmd = cmdline::add_names_arg(cmd);

    let cmd = cmd.subcommand(list_subcmd);
    let cmd = cmd.subcommand(find_subcmd);
    let cmd = cmd.subcommand(write_subcmd);

    let matches = cmd.get_matches();

    let magnitude = cmdline::magnitude(&matches);
    let catalog_filename: PathBuf = cmdline::catalog(&matches).into();

    let mut has_csv = false;
    #[cfg(feature = "csv")]
    {
        has_csv = true;
    }

    let mut catalog = {
        match catalog_filename.extension().and_then(|x| x.to_str()) {
            Some("json") => {
                let s = std::fs::read_to_string(catalog_filename)?;
                let mut catalog: Catalog = serde_json::from_str(&s)?;
                catalog.retain(|s| s.brighter_than(magnitude));
                Ok(catalog)
            }
            Some("csv") => {
                if has_csv {
                    let mut catalog = Catalog::default();
                    let f = std::fs::File::open(catalog_filename)?;
                    #[cfg(feature = "csv")]
                    hipparcos::read_to_catalog(&mut catalog, &f, magnitude)?;
                    Ok(catalog)
                } else {
                    Err(anyhow!("CSV support not provided; star_catalog must be compiled with feature 'csv'"))
                }
            }
            Some(_) => Err(anyhow!(
                "Unknown extension on catalog {}",
                catalog_filename.display()
            )),

            None => Err(anyhow!(
                "Unknown extension on catalog {}",
                catalog_filename.display()
            )),
        }
    }?;

    catalog.sort();

    if let Some(names_filename) = cmdline::names(&matches) {
        let s = std::fs::read_to_string(names_filename)?;
        let id_names: Vec<(usize, String)> = serde_json::from_str(&s)?;
        catalog.add_names(&id_names, true)?;
    }

    match matches.subcommand() {
        Some(("list", sub_matches)) => {
            list(catalog, sub_matches)?;
        }
        // Some(("list", sub_matches)) => {
        // list(catalog, sub_matches)?;
        // }
        Some(("write", sub_matches)) => {
            write(catalog, sub_matches)?;
        }
        Some(("find", sub_matches)) => {
            find(catalog, sub_matches)?;
        }
        _ => {
            println!("Catalog has {} stars", catalog.len());
        }
    }
    Ok(())
}

fn display_star(s: &Star) {
    let id = s.id;
    let ra = s.ra * 180.0 / std::f64::consts::PI;
    let de = s.de * 180.0 / std::f64::consts::PI;
    let ly = s.ly;
    let mag = s.mag;
    println!("{id:8} : {ra}, {de} : {ly} :{mag}");
}

fn find(catalog: Catalog, matches: &ArgMatches) -> Result<(), anyhow::Error> {
    if let Some(stars) = cmdline::stars(matches) {
        for s in stars {
            match s.parse::<usize>() {
                Err(_) => {
                    if let Some(s) = catalog.find_name(s) {
                        display_star(&catalog[s]);
                    } else {
                        eprintln!("Could not find star with name {s}");
                    }
                }
                Ok(id) => {
                    if let Some(s) = catalog.find_sorted(id) {
                        display_star(&catalog[s]);
                    } else {
                        eprintln!("Could not find star with id {id}");
                    }
                }
            }
        }
        // display_star(s);
    }
    Ok(())
}

fn list(catalog: Catalog, matches: &ArgMatches) -> Result<(), anyhow::Error> {
    for s in catalog.iter_stars() {
        display_star(s);
    }
    Ok(())
}

fn write(catalog: Catalog, matches: &ArgMatches) -> Result<(), anyhow::Error> {
    use std::io::Write;
    let output_filename: PathBuf = cmdline::output(&matches).into();
    let mut f = std::fs::File::create(output_filename)?;
    let s = serde_json::to_string_pretty(&catalog)?.replace(" ", "");
    f.write(s.as_bytes())?;
    Ok(())
}
