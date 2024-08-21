use std::collections::HashMap;
use std::error::Error;

use geo_nd::{Quaternion, Vector};
use star_catalog::Subcube;
use star_catalog::{hipparcos, Catalog, Quat, Star, Vec3};

#[test]
fn test_find_stars() -> Result<(), Box<dyn Error>> {
    // Probably we should be testing with mag 5; we can test with 7 though
    let magnitude = 7.0;
    let s = std::fs::read_to_string("hipparcos.json")?;
    let mut catalog: Catalog = serde_json::from_str(&s)?;
    catalog.retain(|s| s.brighter_than(magnitude));
    catalog.sort();
    eprintln!("Loaded {} stars", catalog.len());
    catalog.add_names(hipparcos::HIP_ALIASES, true)?;
    catalog.derive_data();

    // Dubhe, Polaris, Megrez (54061, 11767, 59774)
    let angles_to_find = [28.71, 10.22, 33.58];
    // Dubhe, Megrez, Alkaid
    // let angles_to_find = [25.71, 10.22, 15.71];
    // Mizar, Megrez, Alkaid
    // let angles_to_find = [9.782, 6.676, 15.71];
    let angles_to_find: Vec<f64> = angles_to_find
        .iter()
        .map(|x| x / 180.0 * std::f64::consts::PI)
        .collect();
    let max_angle_delta = 0.15 / 180.0 * std::f64::consts::PI;
    let max_angle_delta = 0.06 / 180.0 * std::f64::consts::PI;
    let cos_angle_ranges: Vec<(f64, f64)> = angles_to_find
        .iter()
        .map(|(a)| {
            (
                (a + max_angle_delta).cos(),
                (a - max_angle_delta).max(0.).cos(),
            )
        })
        .collect();

    // With 32 subcubes the subcube_max_angle is 6.18 degrees
    // 2 subcubes = 12.4
    // 3 subcubes = 18.6
    // 4 subcubes = 24.8
    // 5 subcubes = 30.9
    // 6 subcubes = 37.1
    // 7 subcubes = 43.3
    // 8 subcubes = 49.5
    //
    // These are approx the angle of a SubcubeRange
    let subcube_max_angle = 2.0 * (Subcube::SUBCUBE_RADIUS).asin();
    let subcube_angle_ranges: Vec<(f64, f64)> = angles_to_find
        .iter()
        .map(|a| {
            (
                (a - max_angle_delta - subcube_max_angle).max(0.),
                (a + max_angle_delta + subcube_max_angle).min(std::f64::consts::PI / 2.),
            )
        })
        .collect();
    let subcube_cos_angle_ranges: Vec<(f64, f64)> = subcube_angle_ranges
        .iter()
        .map(|(min, max)| (max.cos(), min.cos()))
        .collect();
    eprintln!("{subcube_max_angle} {subcube_cos_angle_ranges:?}");

    // With max mag 7...
    // For max 33.57 degrees (mag 7.0) needs range = 8
    // For max 25.71 degrees (mag 5.0) needs range = 7
    // let range = Subcube::ELE_PER_SIDE / 2;
    // For max 15.71 degrees (mag 5.0)  needs range = 3
    // let range = Subcube::ELE_PER_SIDE / 2;
    let max_angle = angles_to_find.iter().fold(0.0, |acc: f64, b| acc.max(*b));
    let range = (max_angle / subcube_max_angle).trunc() as usize + 3;

    let cos_max_angle = max_angle.cos();
    let cos_max_angle_to_search = (max_angle + subcube_max_angle).cos();
    dbg!(cos_max_angle_to_search);

    let subcube_iter = Subcube::iter_all();
    // Run through all the stars within each subcube that we have to search
    let mut count = 0;
    for sub0 in subcube_iter {
        if catalog[sub0].is_empty() {
            continue;
        }
        // Before we run through the stars in the subcube, find all
        // the subcubes that are close enough to this one for the
        // neighbors we are going to have to look for; this is for both [1] and [2]
        //
        // For large angle this might be doing 50x the work required
        //
        // However, for small angles the subcubes_to_search will only
        // be about 6 things, all relevant,
        let sub0_center = sub0.center().normalize();
        let mut subcubes_to_search = vec![];
        let min_cos = subcube_cos_angle_ranges[0]
            .0
            .min(subcube_cos_angle_ranges[1].0);
        let max_cos = subcube_cos_angle_ranges[0]
            .1
            .max(subcube_cos_angle_ranges[1].1);
        for s12 in sub0.iter_range(range) {
            if catalog[s12].is_empty() {
                continue;
            }
            let Some(c) = s12.cos_angle_on_sphere(&sub0_center) else {
                continue;
            };
            if c < min_cos || c > max_cos {
                continue;
            }
            subcubes_to_search.push(s12)
        }

        for i0 in catalog[sub0].iter() {
            let s0 = &catalog[*i0];
            // iterate through subcubes_to_search, skipping those that are nowhere near angles_to_find[0] away
            let subcubes_for_s0 = subcubes_to_search
                .iter()
                .filter(|s| {
                    let c = s.center().normalize().dot(&sub0_center);
                    c > subcube_cos_angle_ranges[0].0 && c < subcube_cos_angle_ranges[0].1
                })
                .copied();
            for sub1 in subcubes_for_s0 {
                for i1 in catalog[sub1].iter() {
                    if *i0 == *i1 {
                        continue;
                    }
                    let s1 = &catalog[*i1];

                    let c_s01 = s0.cos_angle_between(s1);
                    if c_s01 < cos_angle_ranges[0].0 || c_s01 > cos_angle_ranges[0].1 {
                        continue;
                    }

                    let sub1_center = sub1.center().normalize();
                    let subcubes_for_s1 = subcubes_to_search
                        .iter()
                        .filter(|s| {
                            let c = s.center().normalize().dot(&sub1_center);
                            c > subcube_cos_angle_ranges[2].0 && c < subcube_cos_angle_ranges[2].1
                        })
                        .filter(|s| {
                            let c = s.center().normalize().dot(&sub0_center);
                            c > subcube_cos_angle_ranges[1].0 && c < subcube_cos_angle_ranges[1].1
                        })
                        .copied();
                    for s2 in catalog.iter_within_subcubes(subcubes_for_s1) {
                        if s0.id == s2.id || s1.id == s2.id {
                            continue;
                        }
                        let c_s02 = s0.cos_angle_between(s2);
                        if c_s02 < cos_angle_ranges[1].0 || c_s02 > cos_angle_ranges[1].1 {
                            continue;
                        }
                        let c_s12 = s1.cos_angle_between(s2);
                        if c_s12 < cos_angle_ranges[2].0 || c_s12 > cos_angle_ranges[2].1 {
                            continue;
                        }
                        eprintln!("{}, {}, {}", s0.id, s1.id, s2.id);
                        count += 1;
                    }
                }
            }
        }
    }
    eprintln!("Count {count}");
    //     let stars_to_search : Vec<usize> = catalog.iter_within_subcubes(s).map(|s| s.id).collect();

    assert!(false);

    Ok(())
}
