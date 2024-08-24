use std::path::PathBuf;

use anyhow::anyhow;
use clap::{ArgMatches, Command};
use geo_nd::Vector;
use star_catalog::{cmdline, Catalog, CatalogIndex, Star, Subcube};

#[cfg(feature = "image")]
use geo_nd::Quaternion;
#[cfg(feature = "image")]
use star_catalog::{ImageView, Quat};

fn find_id_or_name(
    catalog: &Catalog,
    s: Option<&str>,
) -> Result<Option<CatalogIndex>, anyhow::Error> {
    let Some(s) = s else {
        return Ok(None);
    };
    Ok(Some(catalog.find_id_or_name(s)?))
}

fn main() -> Result<(), anyhow::Error> {
    let cmd = Command::new("star_catalog")
        .about("Star catlog")
        .version("0.1.0");

    let cmd = cmdline::add_catalog_arg(
        cmd,
        "Which star catalog to load

This can be a filename with a '.json' extension, or (if enabled with
feature csv) a '.csv' extension, or (if enabled with feature postcard)
a '.pst' extension.

Alterrnatively it can be a built-in catalog if no filename extensionis
provide (if enabled with feature hipp_bright) 'hipp_bright'",
    );

    let cmd = cmdline::add_names_arg(
        cmd,
        "Mapping of star names to ids in the catalog

This can be a filename with a '.json' extension or a built-in names
description.

If a JSON file is specified, it is a list of pairs of (name, id)

Built-in name lists provided are 'hipp' and 'collated'; the former is
the list of common Hipparcos star names from the ESA website, the
latter is a collation of various lists that is much larger
",
    );

    let cmd = cmdline::add_magnitude_arg(
        cmd,
        "Maximum brightess of stars to include

Once a catalog has been loaded, any stars with a magnitude more than
this value are discarded from the catalog

The default value is 12.0
",
    );

    let cmd = cmdline::add_angle_arg(
        cmd,
        "Angle for the star selection.

If this option is provided then any star that is separated from the
center of the selected region by more than this angle is discarded.

If this option is not provided then stars are not discarded based on
the selected region.
",
    );

    let cmd = cmdline::add_right_ascension_arg(
        cmd,
        "Right ascension to center the star selection on.

If the 'angle' option is provided then this specifies the right
ascension (in degrees) of the center of the region of the catalog to keep; stars
outside the region specified are discarded from the catalog.

The default value is 0.
",
    );

    let cmd = cmdline::add_declination_arg(
        cmd,
        "Declination to center the star selection on.

If the 'angle' option is provided then this specifies the declination
(in degrees) of the center of the region of the catalog to keep; stars
outside the region specified are discarded from the catalog.

The default value is 0.
",
    );

    let cmd = cmdline::add_star_arg(
        cmd,
        "Star to center the star selection region on

If the 'angle' option is provided then this specifies (by id or name)
the star at the center of the region of the catalog to keep; stars
outside the region specified are discarded from the catalog.

If this option is not provided then the right-ascension and
declination arguments will be used, if required.
",
    );

    let list_subcmd = Command::new("list").about("Lists the stars in the catalog");

    let find_subcmd = Command::new("find").about("Find stars in the catalog and display them");
    let find_subcmd = cmdline::add_stars_arg(
        find_subcmd,
        "An arbitrary list of star names/ids.

The catalog will be seached for the stars, and the data for each written out
",
    );

    let angle_subcmd = Command::new("angle_between").about("Find angle betwen stars");
    let angle_subcmd = cmdline::add_stars_arg(
        angle_subcmd,
        "An arbitrary list of star names/ids.

The angle between every pair of stars is determined and written out
",
    );

    let triangle_subcmd = Command::new("triangle").about(
        "Find sets of triangles of stars from three angles between them

The catalog is searched for all triplets of stars that match the
specified angular separation to the tolerance provided.
",
    );

    let triangle_subcmd = cmdline::add_angle_arg(
        triangle_subcmd,
        "Maximum angular separation to search within

This provides the angle, in degrees, within which the actual angular
separation of the stars must match that provided by the 'angles'
option.

The default is 0.1 degrees
",
    );

    let triangle_subcmd = cmdline::add_angles_arg(
        triangle_subcmd,
        "Three angles expected between the three stars

Three angles *must* be provided, in degrees.

The catalog is searched for sets of three stars A, B and C where the
angle between A and B is the first angle provided to the command; the
angle between A and C is the second angle provided; and the angle
between B and C is the third angle.

The tolerance (in absolute difference in degrees) for the search is
provided by the 'angle' option to the triangle command.
",
    );

    let write_subcmd =
        Command::new("write").about("Write out the catalog (after star region selection)");
    let write_subcmd = cmdline::add_output_arg(
        write_subcmd,
        "Filename to write the contentst of the catalog to

If the filename has a '.json' extension then a JSON format file is written.

If the filename has a '.pst' extension then a Postcard format file is
written (if the 'postcard' feature is enabled).
",
    );

    let image_subcmd = Command::new("image").about("Generate an image of part of the sky");
    let image_subcmd = cmdline::add_output_arg(image_subcmd, "Specify the output image filaname.

This can have either a '.png' or a '.jpg' extension; PNG files are better for star images as jpeg compression artefacts are considerable for star fields.
");

    let image_subcmd = cmdline::add_width_arg(
        image_subcmd,
        "The width in pixels of the image to produce.

The default value is 512 pixels
",
    );

    let image_subcmd = cmdline::add_height_arg(
        image_subcmd,
        "The height in pixels of the image to produce.

The default value is 512 pixels
",
    );

    let image_subcmd = cmdline::add_right_ascension_arg(
        image_subcmd,
        "The right ascension of the center of the image

Specifies the right ascension in degrees that of the direction of the
center of the image. This is not used if the 'star' option is
provided.

The default value is 0.
",
    );

    let image_subcmd = cmdline::add_declination_arg(
        image_subcmd,
        "The declination of the center of the image

Specifies the declination in degrees that of the direction of the
center of the image. This is not used if the 'star' option is
provided.

The default value is 0.
",
    );

    let image_subcmd = cmdline::add_star_arg(
        image_subcmd,
        "Star name or id to center the image on

If this option is not provided then the right ascension and
declination are used.
",
    );

    let image_subcmd = cmdline::add_up_arg(
        image_subcmd,
        "Star name or id of the default 'up' of the image

If this is not specified then the default 'up' is in the direction of increasing
declination without changing right ascension (north)

Note that 'angle' is applied *AFTER* this.
",
    );

    let image_subcmd = cmdline::add_angle_arg(
        image_subcmd,
        "Angle to rotate 'up' by.

After selecting the default 'up' direction for the image, rotate anticlockwise by this angle.

This is in degrees, and defaults to 0.
",
    );

    let image_subcmd = cmdline::add_fov_arg(
        image_subcmd,
        "The horizontal field of view of the image

This is in degrees, and defaults to 60.
",
    );

    let cubemap_subcmd = Command::new("cubemap").about("Generate an cubemap of part of the sky");
    let cubemap_subcmd = cmdline::add_output_arg(
        cubemap_subcmd,
        "Specify the output cubemap filaname.

This can have either a '.png' or a '.jpg' extension; PNG files are
better for star cubemaps as jpeg compression artefacts are
considerable for star fields.
",
    );

    let cubemap_subcmd = cmdline::add_width_arg(
        cubemap_subcmd,
        "The width in pixels of each face of the cubemap to produce.

The cubemap is 4 times this width.

The default value is 512 pixels
",
    );

    let cubemap_subcmd = cmdline::add_height_arg(
        cubemap_subcmd,
        "The height in pixels of each face of the cubemap to produce.

The cubemap is 3 times this height.

The default value is 512 pixels
",
    );
    let cubemap_subcmd = cmdline::add_right_ascension_arg(
        cubemap_subcmd,
        "The right ascension of the center of the cubemap

Specifies the right ascension in degrees that of the direction of the
center of the cubemap. This is not used if the 'star' option is
provided.

The default value is 0.
",
    );

    let cubemap_subcmd = cmdline::add_declination_arg(
        cubemap_subcmd,
        "The declination of the center of the cubemap

Specifies the declination in degrees that of the direction of the
center of the cubemaop. This is not used if the 'star' option is
provided.

The default value is 0.
",
    );

    let cubemap_subcmd = cmdline::add_star_arg(
        cubemap_subcmd,
        "Star name or id to center the cubemap on

If this option is not provided then the right ascension and
declination are used.
",
    );

    let cubemap_subcmd = cmdline::add_up_arg(
        cubemap_subcmd,
        "Star name or id of the default 'up' of the cubemap

If this is not specified then the default 'up' is in the direction of increasing
declination without changing right ascension (north)

Note that 'angle' is applied *AFTER* this.
",
    );

    let cubemap_subcmd = cmdline::add_angle_arg(
        cubemap_subcmd,
        "Angle to rotate 'up' by.

After selecting the default 'up' direction for the cubemap, rotate anticlockwise by this angle.

This is in degrees, and defaults to 0.
",
    );

    let cmd = cmd.subcommand(list_subcmd);
    let cmd = cmd.subcommand(find_subcmd);
    let cmd = cmd.subcommand(angle_subcmd);
    let cmd = cmd.subcommand(triangle_subcmd);
    let cmd = cmd.subcommand(write_subcmd);

    #[cfg(feature = "image")]
    let cmd = { cmd.subcommand(image_subcmd) };
    #[cfg(feature = "image")]
    let cmd = { cmd.subcommand(cubemap_subcmd) };

    #[cfg(not(feature = "image"))]
    let _ = image_subcmd;
    #[cfg(not(feature = "image"))]
    let _ = cubemap_subcmd;

    let matches = cmd.get_matches();

    let magnitude = cmdline::magnitude(&matches, 12.0);
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
                let _ = catalog;
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
                    catalog = postcard::from_bytes(star_catalog::hipparcos::HIPP_BRIGHT_PST)?;
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

    if let Some(names_filename) = cmdline::names(&matches) {
        let names_filename: PathBuf = names_filename.into();
        match names_filename.extension().and_then(|x| x.to_str()) {
            Some("json") => {
                let s = std::fs::read_to_string(names_filename)?;
                let id_names: Vec<(usize, String)> = serde_json::from_str(&s)?;
                catalog.add_names(&id_names, true)?;
            }
            None => {
                if names_filename.as_os_str().as_encoded_bytes() == b"hipp" {
                    catalog.add_names(star_catalog::hipparcos::HIP_ALIASES, true)?;
                } else if names_filename.as_os_str().as_encoded_bytes() == b"collated" {
                    catalog.add_names(star_catalog::hipparcos::HIP_COLLATED_ALIASES, true)?;
                } else {
                    Err(anyhow!("Unknown builtin file {}", names_filename.display()))?
                }
            }
            _ => Err(anyhow!(
                "Unknown extension for names filename {}",
                names_filename.display()
            ))?,
        }
    }

    let angle = cmdline::angle(&matches, 0.0);
    if angle > 0. {
        catalog.derive_data();
        let mut ids: Vec<usize> = vec![];
        let mut v = Star::vec_of_ra_de(
            cmdline::right_ascension(&matches, 0.),
            cmdline::declination(&matches, 0.),
        );
        if let Some(index) = find_id_or_name(&catalog, cmdline::star(&matches).map(|a| a.as_str()))?
        {
            v = catalog[index].vector;
        }

        let cos_angle = angle.cos();
        for s in catalog.iter_stars() {
            if s.vector.dot(&v) >= cos_angle {
                ids.push(s.id);
            }
        }
        catalog.retain(|s| ids.binary_search(&s.id).is_ok());
        catalog.sort();
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
        Some(("cubemap", sub_matches)) => {
            cubemap(catalog, sub_matches)?;
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
            match find_id_or_name(&catalog, Some(s)) {
                Ok(Some(index)) => {
                    display_star(&catalog[index]);
                }
                Err(e) => {
                    eprintln!("{e}");
                }
                _ => (),
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

    let max_angle_delta = cmdline::angle(matches, 0.1);

    let subcube_iter = Subcube::iter_all();
    let r = catalog.find_star_triangles(subcube_iter, &angles_to_find, max_angle_delta);
    for (a, b, c) in &r {
        let a01 = catalog[*a].cos_angle_between(&catalog[*b]).acos() * 180.0 / std::f64::consts::PI;
        let a02 = catalog[*a].cos_angle_between(&catalog[*c]).acos() * 180.0 / std::f64::consts::PI;
        let a12 = catalog[*b].cos_angle_between(&catalog[*c]).acos() * 180.0 / std::f64::consts::PI;
        println!(
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
    let output_filename: PathBuf = cmdline::output(matches).into();
    match output_filename.extension().and_then(|x| x.to_str()) {
        Some("json") => {
            let mut f = std::fs::File::create(output_filename)?;
            let s = serde_json::to_string_pretty(&catalog)?.replace(" ", "");
            f.write_all(s.as_bytes())?;
        }
        #[cfg(feature = "postcard")]
        Some("pst") => {
            let mut f = std::fs::File::create(output_filename)?;
            let s = postcard::to_allocvec(&catalog)?;
            f.write_all(&s)?;
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
    let _ = matches;
    #[cfg(feature = "image")]
    {
        let tan_fov = (cmdline::fov(matches, 60.0) / 2.0).tan();
        let mut v = Star::vec_of_ra_de(
            cmdline::right_ascension(matches, 0.),
            cmdline::declination(matches, 0.),
        );
        if let Some(index) = find_id_or_name(&catalog, cmdline::star(matches).map(|a| a.as_str()))?
        {
            v = catalog[index].vector;
        }

        let mut up = [0., 0., 1.].into();
        let angle = cmdline::angle(matches, 0.0);
        if let Some(index) = find_id_or_name(&catalog, cmdline::up(matches).map(|a| a.as_str()))? {
            up = catalog[index].vector - v;
        }
        let orient = Quat::look_at(&v, &up);
        let orient = Quat::of_axis_angle(&[0., 0., 1.].into(), angle) * orient;

        let width = cmdline::width(matches, 512) as u32;
        let height = cmdline::height(matches, 512) as u32;
        let image = image::DynamicImage::new_rgb8(width, height);
        let mut image_view = ImageView::new(image);
        image_view
            .set_tan_hfov(tan_fov)
            .set_orient(orient)
            .set_star_size(width / 200);
        let output_filename: PathBuf = cmdline::output(matches).into();

        if true {
            image_view.draw_grid();
        }

        let subcubes = Subcube::iter_all();
        let star_iter = catalog.iter_within_subcubes(subcubes);

        for s in star_iter {
            image_view.draw_star(s);
        }
        let image = image_view.take_image();
        image.save(output_filename)?;
    }
    Ok(())
}

fn cubemap(catalog: Catalog, matches: &ArgMatches) -> Result<(), anyhow::Error> {
    let _ = &catalog;
    let _ = matches;
    #[cfg(feature = "image")]
    {
        let mut v = Star::vec_of_ra_de(
            cmdline::right_ascension(matches, 0.),
            cmdline::declination(matches, 0.),
        );
        if let Some(index) = find_id_or_name(&catalog, cmdline::star(matches).map(|a| a.as_str()))?
        {
            v = catalog[index].vector;
        }
        let mut up = [0., 0., 1.].into();
        let angle = cmdline::angle(matches, 0.0);
        if let Some(index) = find_id_or_name(&catalog, cmdline::up(matches).map(|a| a.as_str()))? {
            up = catalog[index].vector - v;
        }
        let orient = Quat::look_at(&v, &up);
        let orient = Quat::of_axis_angle(&[0., 0., 1.].into(), angle) * orient;

        let width = cmdline::width(matches, 512) as u32;
        let height = cmdline::height(matches, 512) as u32;

        let image = image::DynamicImage::new_rgb8(width * 4, height * 3);
        let mut image_view = ImageView::new(image);
        image_view.set_tan_hfov(1.0).set_star_size(width / 200);
        let output_filename: PathBuf = cmdline::output(matches).into();

        for quadrant in 0..6 {
            let (x_ofs, y_ofs, face_orient) = match quadrant {
                0 => (
                    0,
                    1,
                    Quat::look_at(&[-1., 0., 0.].into(), &[0., 1., 0.].into()),
                ),
                1 => (
                    1,
                    1,
                    Quat::look_at(&[0., 0., -1.].into(), &[0., 1., 0.].into()),
                ),
                2 => (
                    2,
                    1,
                    Quat::look_at(&[1., 0., 0.].into(), &[0., 1., 0.].into()),
                ),
                3 => (
                    3,
                    1,
                    Quat::look_at(&[0., 0., 1.].into(), &[0., 1., 0.].into()),
                ),
                4 => (
                    1,
                    0,
                    Quat::look_at(&[0., 1., 0.].into(), &[0., 0., 1.].into()),
                ),
                _ => (
                    1,
                    2,
                    Quat::look_at(&[0., -1., 0.].into(), &[0., 0., -1.].into()),
                ),
            };
            image_view.set_window((x_ofs * width, y_ofs * height), width, height);
            image_view.set_orient(face_orient * orient);
            if true {
                image_view.draw_grid();
            }

            let subcubes = Subcube::iter_all();
            let star_iter = catalog.iter_within_subcubes(subcubes);

            for s in star_iter {
                image_view.draw_star(s);
            }
            for (_c, s) in star_catalog::constellations::NORTHERN_HEMISPHERE {
                let mut last = None;
                for id in s.iter() {
                    if *id == 0 {
                        last = None;
                        continue;
                    }
                    if let Some(index) = catalog.find_sorted(*id) {
                        if let Some(l) = last {
                            image_view.draw_line_between_stars(
                                [155, 255, 255, 0].into(),
                                &catalog[l],
                                &catalog[index],
                            );
                        }
                        last = Some(index);
                    } else {
                        last = last;
                    }
                }
            }
        }
        let image = image_view.take_image();
        image.save(output_filename)?;
    }
    Ok(())
}
