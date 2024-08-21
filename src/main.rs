use std::path::PathBuf;

use anyhow::anyhow;
use clap::{ArgMatches, Command};
use geo_nd::{Quaternion, Vector};
use star_catalog::{Catalog, Quat, Star, Subcube, Vec3};

mod cmdline {
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
}

fn main() -> Result<(), anyhow::Error> {
    let cmd = Command::new("star_catalog")
        .about("Star catlog")
        .version("0.1.0");

    #[allow(unused_assignments)]
    let mut has_image = false;
    #[cfg(feature = "image")]
    {
        has_image = true;
    }
    has_image = has_image;

    let list_subcmd = Command::new("list").about("Lists the stars in the catalog");
    let find_subcmd = Command::new("find").about("Find stars in the catalog and display them");
    let find_subcmd = cmdline::add_stars_arg(find_subcmd);
    let angle_subcmd = Command::new("angle_between").about("Find angle betwen stars");
    let angle_subcmd = cmdline::add_stars_arg(angle_subcmd);
    let triangle_subcmd =
        Command::new("triangle").about("Find a triangle of stars from three angles between them");
    let triangle_subcmd = cmdline::add_angle_arg(triangle_subcmd);
    let triangle_subcmd = cmdline::add_angles_arg(triangle_subcmd);
    let write_subcmd = Command::new("write").about("Write out the catalog");
    let write_subcmd = cmdline::add_output_arg(write_subcmd);
    let image_subcmd = Command::new("image").about("Generate an image");
    let image_subcmd = cmdline::add_output_arg(image_subcmd);
    let image_subcmd = cmdline::add_width_arg(image_subcmd);
    let image_subcmd = cmdline::add_height_arg(image_subcmd);
    let image_subcmd = cmdline::add_right_ascension_arg(image_subcmd);
    let image_subcmd = cmdline::add_declination_arg(image_subcmd);
    let image_subcmd = cmdline::add_angle_arg(image_subcmd);
    let image_subcmd = cmdline::add_fov_arg(image_subcmd);

    let cmd = cmdline::add_catalog_arg(cmd);
    let cmd = cmdline::add_magnitude_arg(cmd);
    let cmd = cmdline::add_names_arg(cmd);
    let cmd = cmdline::add_right_ascension_arg(cmd);
    let cmd = cmdline::add_declination_arg(cmd);
    let cmd = cmdline::add_angle_arg(cmd);

    let cmd = cmd.subcommand(list_subcmd);
    let cmd = cmd.subcommand(find_subcmd);
    let cmd = cmd.subcommand(angle_subcmd);
    let cmd = cmd.subcommand(triangle_subcmd);
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
                catalog
            }
            #[cfg(feature = "postcard")]
            Some("pst") => {
                let data = std::fs::read(catalog_filename)?;
                let mut catalog: Catalog = postcard::from_bytes(&data)?;
                catalog.retain(|s| s.brighter_than(magnitude));
                catalog
            }
            #[cfg(feature = "csv")]
            Some("csv") => {
                let mut catalog = Catalog::default();
                catalog = catalog;
                {
                    let f = std::fs::File::open(catalog_filename)?;
                    star_catalog::hipparcos::read_to_catalog(&mut catalog, &f, magnitude)?;
                }
                catalog
            }
            None => {
                #[allow(unused_mut)]
                let mut catalog = Catalog::default();
                #[cfg(feature = "hipp_bright")]
                if catalog_filename.as_os_str().as_encoded_bytes() == b"hipp_bright" {
                    catalog = postcard::from_bytes(&star_catalog::hipparcos::HIPP_BRIGHT_PST)?;
                    catalog.retain(|s| s.brighter_than(magnitude));
                }
                if catalog.is_empty() {
                    Err(anyhow!(
                        "Unknown builtin catalog {} (use feature hipp_bright)",
                        catalog_filename.display()
                    ))?
                }
                catalog
            }
            _ => Err(anyhow!(
                "Unknown extension on catalog {} (note that CSV, postcard etc support must be compiled in with appropriate features)",
                catalog_filename.display()
            ))?,
        }
    };

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

    catalog.sort();
    catalog.derive_data();
    match matches.subcommand() {
        Some(("list", sub_matches)) => {
            list(catalog, sub_matches)?;
        }
        Some(("image", sub_matches)) => {
            image(catalog, sub_matches)?;
        }
        Some(("triangle", sub_matches)) => {
            find_triangle(catalog, sub_matches)?;
        }
        Some(("write", sub_matches)) => {
            write(catalog, sub_matches)?;
        }
        Some(("find", sub_matches)) => {
            find(catalog, sub_matches)?;
        }
        Some(("angle_between", sub_matches)) => {
            angle_between(catalog, sub_matches)?;
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

fn angle_between(catalog: Catalog, matches: &ArgMatches) -> Result<(), anyhow::Error> {
    if let Some(stars) = cmdline::stars(matches) {
        let mut star_indices = vec![];
        for s in stars {
            match s.parse::<usize>() {
                Err(_) => {
                    if let Some(index) = catalog.find_name(s) {
                        star_indices.push((s.to_owned(), index));
                    } else {
                        eprintln!("Could not find star with name {s}");
                    }
                }
                Ok(id) => {
                    if let Some(index) = catalog.find_sorted(id) {
                        star_indices.push((s.to_owned(), index));
                    } else {
                        eprintln!("Could not find star with id {id}");
                    }
                }
            }
        }
        for i in 0..star_indices.len() {
            for j in i + 1..star_indices.len() {
                let angle = catalog[star_indices[i].1]
                    .cos_angle_between(&catalog[star_indices[j].1])
                    .acos()
                    * 180.0
                    / std::f64::consts::PI;
                println!(
                    "{} to {} is {} degrees",
                    star_indices[i].0, star_indices[j].0, angle
                );
            }
        }
    }
    Ok(())
}

fn find_triangle(catalog: Catalog, matches: &ArgMatches) -> Result<(), anyhow::Error> {
    let Some(angles) = cmdline::angles(matches) else {
        return Err(anyhow!(
            "Exactly three angles must be specified to find a triangle"
        ));
    };

    if angles.len() != 3 {
        return Err(anyhow!(
            "Exactly three angles must be specified to find a triangle"
        ));
    }

    let mut angles_to_find = [0.; 3];
    for (i, a) in angles.enumerate() {
        angles_to_find[i] = a / 180.0 * std::f64::consts::PI;
    }

    // let max_angle_delta = 0.15 / 180.0 * std::f64::consts::PI;
    let max_angle_delta = cmdline::angle(&matches);

    let subcube_iter = Subcube::iter_all();
    let r = catalog.find_star_triangles(subcube_iter, &angles_to_find, max_angle_delta);
    for (a, b, c) in &r {
        let a01 = catalog[*a].cos_angle_between(&catalog[*b]).acos() * 180.0 / std::f64::consts::PI;
        let a02 = catalog[*a].cos_angle_between(&catalog[*c]).acos() * 180.0 / std::f64::consts::PI;
        let a12 = catalog[*b].cos_angle_between(&catalog[*c]).acos() * 180.0 / std::f64::consts::PI;
        eprintln!(
            "{}, {}, {} : {} {} {}",
            catalog[*a].id, catalog[*b].id, catalog[*c].id, a01, a02, a12,
        );
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
    match output_filename.extension().and_then(|x| x.to_str()) {
        Some("json") => {
            let mut f = std::fs::File::create(output_filename)?;
            let s = serde_json::to_string_pretty(&catalog)?.replace(" ", "");
            f.write(s.as_bytes())?;
        }
        #[cfg(feature = "postcard")]
        Some("pst") => {
            let mut f = std::fs::File::create(output_filename)?;
            let s = postcard::to_allocvec(&catalog)?;
            f.write(&s)?;
        }
        _ => Err(anyhow!(
            "Unknown extension on catalog {} (note that CSV, postcard etc support must be compiled in with appropriate features)",
            output_filename.display()
        ))?,
    }
    Ok(())
}

fn image(catalog: Catalog, matches: &ArgMatches) -> Result<(), anyhow::Error> {
    let _ = &catalog;
    #[cfg(feature = "image")]
    {
        let tan_fov = (cmdline::fov(&matches) / 2.0).tan();
        let v = Star::vec_of_ra_de(
            cmdline::right_ascension(&matches),
            cmdline::declination(&matches),
        );
        let angle = cmdline::angle(&matches);
        let width = cmdline::width(&matches) as u32;
        let height = cmdline::height(&matches) as u32;
        let output_filename: PathBuf = cmdline::output(&matches).into();

        // tan_fov is frame mm width / focal length in mm
        fn pxy_of_vec(width: u32, height: u32, tan_fov: f64, v: &Vec3) -> Option<(u32, u32)> {
            if v[2] > 0. {
                return None;
            }
            let tx = v[0];
            let ty = v[1];
            let x = (width as f64) * (0.5 + tx / tan_fov);
            let y = (height as f64) * (0.5 - ty / tan_fov);
            if x < 0. || x >= width as f64 {
                return None;
            }
            if y < 0. || y >= height as f64 {
                return None;
            }
            Some((x as u32, y as u32))
        }

        let up = [0., 0., 1.].into();
        let avg = Quat::look_at(&v, &up);
        let avg = Quat::of_axis_angle(&[0., 0., 1.].into(), angle) * avg;

        let mut image = image::DynamicImage::new_rgb8(width, height);

        if true {
            use image::GenericImage;
            let color_0 = [100, 10, 10, 0].into();
            let color_1 = [10, 100, 10, 0].into();
            for de_i in 0..180 {
                let de = ((de_i as f64) / 90.0 - 1.0) * std::f64::consts::PI;
                let color = {
                    if de_i % 10 == 0 {
                        color_1
                    } else {
                        color_0
                    }
                };
                for ra_i in 0..3600 {
                    let ra = (ra_i as f64) / 1800.0 * std::f64::consts::PI;
                    let v = Star::vec_of_ra_de(ra, de);
                    if let Some((x, y)) = pxy_of_vec(width, height, tan_fov, &avg.apply3(&v)) {
                        image.put_pixel(x, y, color);
                    }
                }
            }
            for ra_i in 0..360 {
                let ra = (ra_i as f64) / 180.0 * std::f64::consts::PI;
                let color = {
                    if ra_i % 10 == 0 {
                        color_1
                    } else {
                        color_0
                    }
                };
                for de_i in 0..1800 {
                    let de = ((de_i as f64) / 900.0 - 1.0) * std::f64::consts::PI;
                    let v = Star::vec_of_ra_de(ra, de);
                    if let Some((x, y)) = pxy_of_vec(width, height, tan_fov, &avg.apply3(&v)) {
                        image.put_pixel(x, y, color);
                    }
                }
            }
        }

        let subcubes = Subcube::iter_all();
        let subcubes = subcubes.filter(|s| s.may_be_on_sphere());
        let star_iter = catalog.iter_within_subcubes(subcubes);

        for s in star_iter {
            if !s.brighter_than(7.0) {
                continue;
            }
            let v = avg.apply3(&s.vector);
            if let Some(xy) = pxy_of_vec(width, height, tan_fov, &v) {
                let (r, g, b) = Star::temp_to_rgb(s.temp());
                let color = [
                    (r.clamp(0., 1.) * 255.9).floor() as u8,
                    (g.clamp(0., 1.) * 255.9).floor() as u8,
                    (b.clamp(0., 1.) * 255.9).floor() as u8,
                    0,
                ]
                .into();
                draw_star(
                    &mut image,
                    width,
                    height,
                    xy.0 as u32,
                    xy.1 as u32,
                    color,
                    s.mag,
                );
            }
        }
        image.save(output_filename)?;
    }
    Ok(())
}

#[cfg(feature = "image")]
fn draw_star(
    image: &mut image::DynamicImage,
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    color: image::Rgba<u8>,
    mag: f32,
) {
    use image::GenericImage;
    let size = ((7.0 - mag).powi(2) / 8.0).max(0.) as u32;
    if false {
        // draw a cross
        for dx in 0..(2 * size + 1) {
            if x as u32 + dx >= size && x as u32 + dx - size < width {
                image.put_pixel(x as u32 + dx - size, y as u32, color);
            }
        }
        for dy in 0..(2 * size + 1) {
            if y as u32 + dy >= size && y as u32 + dy - size < height {
                image.put_pixel(x as u32, y as u32 + dy - size, color);
            }
        }
    } else {
        // draw a circle
        for dx in 0..size + 1 {
            for dy in 0..size + 1 {
                if dx * dx + dy * dy > size * size {
                    continue;
                }
                if x + dx < width {
                    if y + dy < height {
                        image.put_pixel(x + dx, y + dy, color);
                    }
                    if y > dy {
                        image.put_pixel(x + dx, y - dy, color);
                    }
                }
                if x > dx {
                    if y + dy < height {
                        image.put_pixel(x - dx, y + dy, color);
                    }
                    if y > dy {
                        image.put_pixel(x - dx, y - dy, color);
                    }
                }
            }
        }
    }
}
