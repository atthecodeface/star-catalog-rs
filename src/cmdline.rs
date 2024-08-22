
use clap::{parser::ValuesRef, value_parser, Arg, ArgAction, ArgMatches, Command};

//fp add_catalog_arg
pub fn add_catalog_arg(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("catalog")
            .required(true)
            .help("Which star catalog to load")
            .action(ArgAction::Set),
    )
}
pub fn catalog(matches: &ArgMatches) -> String {
    matches.get_one::<String>("catalog").unwrap().to_string()
}

//fp add_width_arg
pub fn add_width_arg(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("width")
            .long("width")
            .short('W')
            .help("Width of image to generate ")
            .value_parser(value_parser!(usize))
            .action(ArgAction::Set),
    )
}
pub fn width(matches: &ArgMatches) -> usize {
    *matches.get_one::<usize>("width").unwrap_or(&512)
}

//fp add_height_arg
pub fn add_height_arg(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("height")
            .long("height")
            .short('H')
            .help("Height of image to generate")
            .value_parser(value_parser!(usize))
            .action(ArgAction::Set),
    )
}
pub fn height(matches: &ArgMatches) -> usize {
    *matches.get_one::<usize>("height").unwrap_or(&512)
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
    *matches.get_one::<f32>("magnitude").unwrap_or(&12.0)
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

//fp add_angles_arg
pub fn add_angles_arg(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("angles")
            .help("Angles for the command")
            .value_parser(value_parser!(f64))
            .action(ArgAction::Append),
    )
}
pub fn angles(matches: &ArgMatches) -> Option<ValuesRef<'_, f64>> {
    matches.get_many::<f64>("angles")
}

//fp add_fov_arg
pub fn add_fov_arg(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("fov")
            .long("fov")
            .short('f')
            .help("Field of view")
            .value_parser(value_parser!(f64))
            .action(ArgAction::Set),
    )
}
pub fn fov(matches: &ArgMatches) -> f64 {
    matches
        .get_one::<f64>("fov")
        .map(|x| *x * std::f64::consts::PI / 180.0)
        .unwrap_or(60.0)
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

//fp add_star_arg
pub fn add_star_arg(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("star")
            .long("star")
            .short('s')
            .help("Star to use instead of right ascension and declination")
            .action(ArgAction::Set),
    )
}
pub fn star(matches: &ArgMatches) -> Option<&String> {
    matches.get_one::<String>("star")
}

//fp add_up_arg
pub fn add_up_arg(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("up")
            .long("up")
            .short('u')
            .help("Up to use for an image")
            .action(ArgAction::Set),
    )
}
pub fn up(matches: &ArgMatches) -> Option<&String> {
    matches.get_one::<String>("up")
}
