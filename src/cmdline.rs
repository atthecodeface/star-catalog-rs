//a Imports
use clap::{parser::ValuesRef, value_parser, Arg, ArgAction, ArgMatches, Command};

//a Catalog
//fp add_catalog_arg
/// Add an argument to a clap [Command] to specify a catalog to load;
/// this is a *rerquired* *positional* argument
pub fn add_catalog_arg(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("catalog")
            .required(true)
            .help("Which star catalog to load")
            .action(ArgAction::Set),
    )
}

//fp catalog
/// Retrieve the value of the catalog to load from the clap [Matches]
///
/// Panics if the command argument was not required and was not given
pub fn catalog(matches: &ArgMatches) -> String {
    matches.get_one::<String>("catalog").unwrap().to_string()
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

//fp names
pub fn names(matches: &ArgMatches) -> Option<String> {
    matches.get_one::<String>("names").map(|s| s.to_string())
}

//a Image specification arguments
//fp add_width_arg
/// Add an optional argument to a clap [Command] to specify the width
/// of an image
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

//fp width
/// Retrieve the value of the width argument
pub fn width(matches: &ArgMatches, default: usize) -> usize {
    *matches.get_one::<usize>("width").unwrap_or(&default)
}

//fp add_height_arg
/// Add an optional argument to a clap [Command] to specify the height
/// of an image
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

//fp height
/// Retrieve the value of the height argument
pub fn height(matches: &ArgMatches, default: usize) -> usize {
    *matches.get_one::<usize>("height").unwrap_or(&default)
}

//fp add_fov_arg
/// Add an optional argument to a clap [Command] to specify the field of view
/// of an image
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

//fp fov
/// Retrieve the value of the field-of-view argument or a default value
///
pub fn fov(matches: &ArgMatches, default: f64) -> f64 {
    matches.get_one::<f64>("fov").unwrap_or(&default) * std::f64::consts::PI / 180.0
}

//fp add_up_arg
/// Add an optional argument to a clap [Command] to specify an up
/// direction - a star name or id
pub fn add_up_arg(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("up")
            .long("up")
            .short('u')
            .help("Up to use for an image")
            .action(ArgAction::Set),
    )
}

//fp up
/// Retrieve the value of the up argument
pub fn up(matches: &ArgMatches) -> Option<&String> {
    matches.get_one::<String>("up")
}

//a Star specification arguments
/// Add an optional argument to a clap [Command] to specify the magnitude
/// of stars
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

//fp magnitude
/// Retrieve the value of the magnitude argument
pub fn magnitude(matches: &ArgMatches, default: f32) -> f32 {
    *matches.get_one::<f32>("magnitude").unwrap_or(&default)
}

//fp add_right_ascension_arg
/// Add an optional argument to a clap [Command] to specify a right
/// ascension for a direction or orientation
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

//fp right_ascension
/// Retrieve the value of the right ascension argument or a default value
///
/// default is in degrees; the result is in radians
pub fn right_ascension(matches: &ArgMatches, default: f64) -> f64 {
    matches
        .get_one::<f64>("right_ascension")
        .unwrap_or(&default)
        * std::f64::consts::PI
        / 180.0
}

//fp add_declination_arg
/// Add an optional argument to a clap [Command] to specify a declination
/// for a direction or orientation
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

//fp declination
/// Retrieve the value of the declination argument or a default value
///
/// default is in degrees; the result is in radians
pub fn declination(matches: &ArgMatches, default: f64) -> f64 {
    matches.get_one::<f64>("declination").unwrap_or(&default) * std::f64::consts::PI / 180.0
}

//fp add_star_arg
/// Add an optional argument to a clap [Command] to specify a single
/// star - by name or id
pub fn add_star_arg(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("star")
            .long("star")
            .short('s')
            .help("Star to use instead of right ascension and declination")
            .action(ArgAction::Set),
    )
}

//fp star
/// Retrieve the value of the star argument
pub fn star(matches: &ArgMatches) -> Option<&String> {
    matches.get_one::<String>("star")
}

//a Angle arguments
//fp add_angle_arg
/// Add an optional argument to a clap [Command] to specify an angle
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

//fp angle
/// Retrieve the value of the angle argument or a default value
///
/// default is in degrees; the result is in radians
pub fn angle(matches: &ArgMatches, default: f64) -> f64 {
    matches.get_one::<f64>("angle").unwrap_or(&default) * std::f64::consts::PI / 180.0
}

//fp add_angles_arg
/// Add an positional argument to a clap [Command] to specify a list of angles
pub fn add_angles_arg(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("angles")
            .help("Angles for the command")
            .value_parser(value_parser!(f64))
            .action(ArgAction::Append),
    )
}

//fp angles
/// Retrieve the value of the angles argument
pub fn angles(matches: &ArgMatches) -> Option<ValuesRef<'_, f64>> {
    matches.get_many::<f64>("angles")
}

//a Image arguments
//fp add_output_arg
/// Add a required argument to a clap [Command] to specify the output file
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

//fp output
/// Retrieve the value of the output argument
pub fn output(matches: &ArgMatches) -> String {
    matches.get_one::<String>("output").unwrap().to_string()
}

//fp add_stars_arg
/// Add a positional argument to a clap [Command] to specify a list of stars
pub fn add_stars_arg(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("stars")
            .help("Stars to interrogate")
            .action(ArgAction::Append),
    )
}

//fp stars
/// Retrieve the value of the stars argument
pub fn stars(matches: &ArgMatches) -> Option<ValuesRef<'_, String>> {
    matches.get_many::<String>("stars")
}
