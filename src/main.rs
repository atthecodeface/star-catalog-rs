use std::path::PathBuf;

use anyhow::anyhow;
use clap::{ArgMatches, Command};
use geo_nd::Vector;
use star_catalog::{Catalog, Star};

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

    //fp add_right_ascension_arg
    pub fn add_right_ascension_arg(cmd: Command) -> Command {
        cmd.arg(
            Arg::new("right_ascension")
                .long("right_ascension")
                .short('r')
                .help("Right ascension")
                .value_parser(value_parser!(f64))
                .action(ArgAction::Set),
        )
    }
    pub fn right_ascension(matches: &ArgMatches) -> f64 {
        matches
            .get_one::<f64>("right_ascension")
            .map(|x| *x * std::f64::consts::PI / 180.0)
            .unwrap_or(0.0)
    }

    //fp add_declination_arg
    pub fn add_declination_arg(cmd: Command) -> Command {
        cmd.arg(
            Arg::new("declination")
                .long("declination")
                .short('d')
                .help("Declination")
                .value_parser(value_parser!(f64))
                .action(ArgAction::Set),
        )
    }
    pub fn declination(matches: &ArgMatches) -> f64 {
        matches
            .get_one::<f64>("declination")
            .map(|x| *x * std::f64::consts::PI / 180.0)
            .unwrap_or(0.0)
    }

    //fp add_angle_arg
    pub fn add_angle_arg(cmd: Command) -> Command {
        cmd.arg(
            Arg::new("angle")
                .long("angle")
                .short('a')
                .help("Angle")
                .value_parser(value_parser!(f64))
                .action(ArgAction::Set),
        )
    }
    pub fn angle(matches: &ArgMatches) -> f64 {
        matches
            .get_one::<f64>("angle")
            .map(|x| *x * std::f64::consts::PI / 180.0)
            .unwrap_or(0.0)
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

    let mut has_csv = false;
    #[cfg(feature = "csv")]
    {
        has_csv = true;
    }
    let mut has_image = false;
    #[cfg(feature = "image")]
    {
        has_image = true;
    }
    has_csv = has_csv;
    has_image = has_image;

    let list_subcmd = Command::new("list").about("Lists the stars in the catalog");
    let find_subcmd = Command::new("find").about("Find stars in the catalog and display them");
    let find_subcmd = cmdline::add_stars_arg(find_subcmd);
    let write_subcmd = Command::new("write").about("Write out the catalog as JSON");
    let write_subcmd = cmdline::add_output_arg(write_subcmd);
    let image_subcmd = Command::new("image").about("Generate an image");
    let image_subcmd = cmdline::add_output_arg(image_subcmd);

    let cmd = cmdline::add_catalog_arg(cmd);
    let cmd = cmdline::add_magnitude_arg(cmd);
    let cmd = cmdline::add_names_arg(cmd);
    let cmd = cmdline::add_right_ascension_arg(cmd);
    let cmd = cmdline::add_declination_arg(cmd);
    let cmd = cmdline::add_angle_arg(cmd);

    let cmd = cmd.subcommand(list_subcmd);
    let cmd = cmd.subcommand(find_subcmd);
    let cmd = cmd.subcommand(write_subcmd);
    let cmd = {
        if has_image {
            cmd.subcommand(image_subcmd)
        } else {
            cmd
        }
    };

    let matches = cmd.get_matches();

    let magnitude = cmdline::magnitude(&matches);
    let catalog_filename: PathBuf = cmdline::catalog(&matches).into();

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
                    catalog = catalog;
                    #[cfg(feature = "csv")]
                    {
                        let f = std::fs::File::open(catalog_filename)?;
                        star_catalog::hipparcos::read_to_catalog(&mut catalog, &f, magnitude)?;
                    }
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
    let angle = cmdline::angle(&matches);
    if angle > 0. {
        catalog.derive_data();
        let mut ids: Vec<usize> = vec![];
        dbg!(angle);
        let v = Star::vec_of_ra_de(
            cmdline::right_ascension(&matches),
            cmdline::declination(&matches),
        );
        let cos_angle = angle.cos();
        for s in catalog.iter_stars() {
            if s.vector.dot(&v) >= cos_angle {
                ids.push(s.id);
            }
        }
        catalog.retain(|s| ids.binary_search(&s.id).is_ok());
        catalog.sort();
    }

    if let Some(names_filename) = cmdline::names(&matches) {
        let s = std::fs::read_to_string(names_filename)?;
        let id_names: Vec<(usize, String)> = serde_json::from_str(&s)?;
        catalog.add_names(&id_names, true)?;
    }

    match matches.subcommand() {
        Some(("list", sub_matches)) => {
            list(catalog, sub_matches)?;
        }
        Some(("image", sub_matches)) => {
            image(catalog, sub_matches)?;
        }
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

fn list(catalog: Catalog, _matches: &ArgMatches) -> Result<(), anyhow::Error> {
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

fn image(catalog: Catalog, matches: &ArgMatches) -> Result<(), anyhow::Error> {
    let _ = &catalog;
    let _ = matches;
    #[cfg(feature = "image")]
    {
        let avg = 0.;
        let subcubes = subcube.iter_range(3);
        let subcubes = subcubes.filter(|s| s.may_be_on_sphere());
        let star_iter = catalog.iter_within_subcubes(subcubes);

        use image::GenericImage;
        let mut image =
            image::DynamicImage::new_rgb8(camera.width() as u32, camera.height() as u32);
        for s in star_iter {
            if !s.brighter_than(7.0) {
                continue;
            }
            let v = avg.apply3(&s.vector);
            if let Some(xy) = camera.within_frame(camera.pxy_of_vec(&v)) {
                // eprintln!("{xy:?}");
                if xy.0 < 8 || xy.0 + 8 >= camera.width() {
                    continue;
                }
                if xy.1 < 8 || xy.1 + 8 >= camera.height() {
                    continue;
                }
                for dx in 0..17 {
                    image.put_pixel(xy.0 as u32 + dx - 8, xy.1 as u32, [128, 255, 255, 0].into());
                }
                for dy in 0..17 {
                    image.put_pixel(xy.0 as u32, xy.1 as u32 + dy - 8, [128, 255, 255, 0].into());
                }
            }
        }
        image.save("test.png")?;
    }
    Ok(())
}
