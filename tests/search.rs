use std::error::Error;

use star_catalog::Subcube;
use star_catalog::{hipparcos, Catalog};

#[test]
fn test_find_stars() -> Result<(), Box<dyn Error>> {
    // Probably we should be testing with mag 5; we can test with 7 though
    let magnitude = 5.0;
    let s = std::fs::read_to_string("hipparcos.json")?;
    let mut catalog: Catalog = serde_json::from_str(&s)?;
    catalog.retain(move |s, _n| s.brighter_than(magnitude));
    catalog.sort();
    eprintln!("Loaded {} stars", catalog.len());
    catalog.add_names(hipparcos::HIP_ALIASES, true)?;
    catalog.derive_data();

    // Dubhe, Polaris, Megrez (54061, 11767, 59774)
    let mut angles_to_find = [28.71, 10.22, 33.58];
    // Dubhe, Megrez, Alkaid
    // let angles_to_find = [25.71, 10.22, 15.71];
    // Mizar, Megrez, Alkaid
    // let angles_to_find = [9.782, 6.676, 15.71];
    for i in 0..3 {
        angles_to_find[i] = angles_to_find[i] / 180.0 * std::f64::consts::PI;
    }

    // let max_angle_delta = 0.15 / 180.0 * std::f64::consts::PI;
    let max_angle_delta = 0.06 / 180.0 * std::f64::consts::PI;

    let subcube_iter = Subcube::iter_all();
    let r = catalog.find_star_triangles(subcube_iter, &angles_to_find, max_angle_delta);
    let mut errs = 0;
    for (a, b, c) in &r {
        let a01 = catalog[*a].cos_angle_between(&catalog[*b]).acos();
        let a12 = catalog[*b].cos_angle_between(&catalog[*c]).acos();
        let a20 = catalog[*c].cos_angle_between(&catalog[*a]).acos();
        let d12 = (a12 - angles_to_find[0]).abs();
        let d20 = (a20 - angles_to_find[1]).abs();
        let d01 = (a01 - angles_to_find[2]).abs();
        eprintln!(
            "{}, {}, {} : {} {} {}",
            catalog[*a].id(),
            catalog[*b].id(),
            catalog[*c].id(),
            d01,
            d12,
            d20,
        );
        if d01 > max_angle_delta || d12 > max_angle_delta || d20 > max_angle_delta {
            errs += 1;
            eprintln!(" Angle at A: {a12} {}", angles_to_find[0]);
            eprintln!(" Angle at B: {a20} {}", angles_to_find[1]);
            eprintln!(" Angle at C: {a01} {}", angles_to_find[2]);
        }
    }
    eprintln!("Count {}", r.len());

    assert_eq!(errs, 0);

    Ok(())
}
