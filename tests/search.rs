use std::error::Error;

use geo_nd::{FArray, Vector};

use star_catalog::{Catalog, StarTriangleMatch, StarTriangleSearch, Subcube, Vec3, hipparcos};

#[test]
fn test_find_stars() -> Result<(), Box<dyn Error>> {
    // Probably we should be testing with mag 5; we can test with 7 though
    let magnitude = 5.0;
    let (is_hipparcos, s) = match std::fs::read_to_string("hipparcos.json") {
        Ok(s) => (true, s),
        Err(_s) => (false, std::fs::read_to_string("hipparcos_mag7.json")?),
    };
    let mut catalog: Catalog = serde_json::from_str(&s)?;
    catalog.retain(move |s, _n| s.brighter_than(magnitude));
    catalog.sort();
    eprintln!("Loaded {} stars", catalog.len());
    if is_hipparcos {
        catalog.add_names(hipparcos::HIP_ALIASES, true)?;
    }
    catalog.derive_data();

    // Dubhe, Polaris, Megrez (54061, 11767, 59774)
    let mut angles_to_find = [28.71, 10.22, 33.58];
    // Dubhe, Megrez, Alkaid
    // let angles_to_find = [25.71, 10.22, 15.71];
    // Mizar, Megrez, Alkaid
    // let angles_to_find = [9.782, 6.676, 15.71];
    for a in &mut angles_to_find {
        *a = *a / 180.0 * std::f64::consts::PI;
    }

    // let max_angle_delta = 0.15 / 180.0 * std::f64::consts::PI;
    let max_angle_delta = 0.06 / 180.0 * std::f64::consts::PI;

    let subcube_iter = Subcube::iter_all();
    let search = StarTriangleSearch::of_angles(angles_to_find, max_angle_delta).unwrap();
    let (_finished, mut r) = catalog.find_star_triangles(subcube_iter, &search, 1000);
    r.sort_by(StarTriangleMatch::compare_angle_sum);
    let mut errs = 0;
    for tm in &r {
        let t = tm.triangle();
        let a01 = catalog[t.0].cos_angle_between(&catalog[t.1]).acos();
        let a12 = catalog[t.1].cos_angle_between(&catalog[t.2]).acos();
        let a20 = catalog[t.2].cos_angle_between(&catalog[t.0]).acos();
        let d12 = (a12 - angles_to_find[0]).abs();
        let d20 = (a20 - angles_to_find[1]).abs();
        let d01 = (a01 - angles_to_find[2]).abs();
        eprintln!(
            "{}, {}, {} : {} {} {}",
            catalog[t.0].id(),
            catalog[t.1].id(),
            catalog[t.2].id(),
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

/*
 * On a  5184 by 3456 that is rectilinear(?) with focal lengrh 24.1mm
 *
 * View to ECEF is rijk 0.7464623291490128, 0.5184452038309155, -0.3327355431837139,  0.2515862080647313
 *
 * 4427 to 746 is 6.147565179199756 degrees
 * 4427 to 112158 is 37.649680798045566 degrees
 * 746 to 112158 is 32.406063021307645 degrees

 * Mag 3 or brighter stars are HI6686,
 * HI4427 @1175,1778
 * Caph HI746@2026,1486
 * , (HI112158 @4072,1319), (Scheat 113881 @3984, 1006)
 * Schedir@2064,1781
 * HI6686 @ 1677.2645209142268, 1979.636740923519
 * Alpheratz @ 3776.866537717601, 2159.164410058027
 * Mirach @ 3038.2388794590843, 2709.2694372292353
 * Almaak @ 2302.97445201916, 2979.603774075444
 *
  1775.112310638358, 1776.5027160418604 HI4427
​
29: Array [ 1675.626164299013, 1978.6915504053422 ]
9: Array [ 2026.790212632814, 1486.551562888918 ] Caph

1: Array [ 3278.0984711515625, 2101.7363607401962 ]
2: Array [ 2744.0131303397543, 741.4831578035513 ]
3: Array [ 2623.556301560086, 663.5877418593657 ]
4: Array [ 2797.0141350028084, 675.6334247373326 ]
5: Array [ 2660.4963957191844, 796.0902535170011 ]
6: Array [ 2756.0588132177213, 993.6394527156574 ]
7: Array [ 2907.031371954906, 731.8466115011778 ]
8: Array [ 2157.7898969453677, 1195.2038795403028 ]
10: Array [ 3788.072160250552, 1668.7084067303113 ]
11: Array [ 3768.38880993417, 2159.1321611844974 ]
12: Array [ 2969.354741620304, 2479.8118138032387 ]
13: Array [ 2869.3800887802986, 2329.8498345432304 ]
14: Array [ 2410.3221645011904, 2548.1431132214075 ]
15: Array [ 2438.7553226483483, 2496.3213895015883 ]
16: Array [ 1751.4663267099875, 2448.325325765358 ]
17: Array [ 2045.7532882082467, 2477.9184280389254 ]
18: Array [ 2182.8137962211354, 2513.5994445149945 ]
19: Array [ 3863.8068532531424, 1836.256076544863 ]
20: Array [ 2870.492724486179, 1452.4758915567604 ]
21: Array [ 3059.5619285986245, 1513.5206570101059 ]
22: Array [ 2988.3430355697215, 1519.4555647625143 ]
23: Array [ 2846.753093476545, 1542.347351807519 ]
24: Array [ 2577.294186604003, 2093.026531464874 ]
25: Array [ 2501.138907906271, 2067.0955962588723 ]
26: Array [ 2391.426729551264, 1978.515602901052 ]
27: Array [ 2334.753024562189, 2023.8814309168151 ]
28: Array [ 3175.2975749023262, 2041.3488949845691 ]
30: Array [ 1686.4967383943324, 2068.1408458182564 ]
31: Array [ 1802.6565872986025, 2017.5150293171973
*/

#[allow(dead_code)]
const STARS_ON_IMAGE: &[(usize, &str, u32, u32)] = &[
    (4427, "HI4427", 1775, 1776), // 4151 ?
    (746, "Caph HI746", 2026, 1486),
    (113881, "Scheat HI113881", 4069, 1314),
    (112158, "HI112158", 3982, 1006),
    (3179, "Schedir", 2064, 1781),
    (6686, "HI6686", 1677, 1979),
    (677, "Alpheratz", 3776, 2159),
    (5447, "Mirach", 3038, 2709),
    (9640, "Almaak", 2302, 2979),
    (3278, "HI1473", 3278, 2101),
    (111169, "HI111169", 2744, 741),
    (110538, "HI110538", 2623, 663),
    (110609, "HI110609", 2797, 675),
    (111674, "HI111674", 2660, 796),
    (113288, "HI113288", 2756, 993),
    (111022, "HI111022", 2907, 731),
    (115990, "HI115990", 2157, 1195),
    (116310, "HI116310", 3788, 1668),
    (4436, "HI4436", 2969, 2479),
    (3881, "HI3881", 2869, 2329),
    (6813, "HI6813", 2410, 2548),
    (6411, "HI6411", 2438, 2496),
    (9505, "HI9505", 1751, 2448),
    (8068, "HI8068", 2045, 2477),
    (7607, "HI7607", 2182, 2513),
    (117073, "HI117073", 3863, 1836),
    (116584, "HI116584", 2870, 1452),
    (116631, "HI116631", 3059, 1513),
    (116805, "HI116805", 2988, 1519),
    (117221, "HI117221", 2846, 1542),
    (3414, "HI3414", 2577, 2093),
    (3504, "HI3504", 2501, 2067),
    (3300, "HI3300", 2391, 1978),
    (3801, "HI3801", 2334, 2023),
    (1366, "HI1366", 3175, 2041),
    (7294, "HI7294", 1686, 2068),
    (6242, "HI6242", 1802, 2017),
    (2599, "HI2599", 1741, 1557),
    (3821, "HI3821", 1962, 1815),
    /**/
];

fn vector_of_img(x: u32, y: u32) -> FArray<f64, 3> {
    let w: f64 = 5184.0;
    let h: f64 = 3456.0;
    let mm_equiv: f64 = 23.5;

    //  tan_hfovh = 18 / mm_equiv;
    //  fovh = 2 * Math.atan(tan_hfovh);
    //  tan_hfovh = Math.tan(this.fovh / 2);

    // -h/w < yf < h/w, for yf in the height of the image
    let xf = ((x as f64) - w / 2.0) / (w / 2.0);
    let yf = ((y as f64) - h / 2.0) / (w / 2.0);

    // rectilinear:
    //  sensor_rf = R tan (world_yaw)
    //
    //  sensor_rf of 1 occurs at xf=1.0 for half field-of-view
    //  world_yaw at xf=1.0 is atan(half-36-mm-frame / 36mm_equiv focal distance)
    //  hence tan(world_yaw) = 18.0 / mm_equiv = sensor_rf / R = 1.0 / R
    //  hence R = mm_equiv / 18.0
    //  and hence
    //  world_yaw = atan(sensor_rf * 18 / mm_equiv)
    let sensor_rf = (xf * xf + yf * yf).sqrt();
    let roll = yf.atan2(xf);

    let world_yaw = (sensor_rf * 18.0 / mm_equiv).atan();
    [
        world_yaw.cos(),
        -world_yaw.sin() * roll.cos(),
        -world_yaw.sin() * roll.sin(),
    ]
    .into()
}

// * On a  5184 by 3456 that is rectilinear(?) with focal lengrh 24.1mm

#[cfg(feature = "hipp_bright")]
#[test]
fn test_find_caph() -> Result<(), Box<dyn Error>> {
    let magnitude = 5.0;
    let mut catalog = Catalog::load_catalog("hipp_bright", magnitude)?;
    catalog.sort();
    catalog.derive_data();
    eprintln!("Loaded {} stars", catalog.len());

    let mut img_vectors: Vec<[f64; 3]> = vec![];
    for (_id, _name, x, y) in STARS_ON_IMAGE {
        img_vectors.push(vector_of_img(*x, *y).into());
        eprintln!("{:?}", img_vectors.last().unwrap());
    }

    // 6.190846898801532 32.306172778472444 36.946847293065176
    //
    // 4427 to 746 is 6.147565179199756 degrees
    // 746 to 112158 is 32.406063021307645 degrees
    // 4427 to 112158 is 37.649680798045566 degrees
    //
    let a01 = geo_nd::vector::dot(&img_vectors[0], &img_vectors[1]).acos() * 180.0 / 3.1415926;
    let a12 = geo_nd::vector::dot(&img_vectors[1], &img_vectors[2]).acos() * 180.0 / 3.1415926;
    let a20 = geo_nd::vector::dot(&img_vectors[2], &img_vectors[0]).acos() * 180.0 / 3.1415926;
    eprintln!("Angles between the search vectors (in degres) {a01} {a12} {a20}");

    let s = Subcube::iter_sphere();

    let max_angle_delta = 0.3 / 180.0 * 3.14159;
    let (finished, mut results) =
        catalog.find_best_star_mappings(s, &img_vectors, max_angle_delta, usize::MAX);
    assert!(finished, "Expected to have tried all candidates");

    // View to ECEF is rijk 0.7464623291490128, 0.5184452038309155, -0.3327355431837139,  0.2515862080647313

    let mut found = false;
    results.sort_by(|a, b| a.quality.partial_cmp(&b.quality).unwrap());
    for r in results {
        eprintln!(
            "{} {:?}",
            r.initial_match.angle_sum(),
            r.initial_match.quaternion(),
        );
        eprintln!(
            "{} {} {:?}",
            r.angle_mean * 180.0 / 3.14159265,
            r.quality * 180.0 / 3.14159265,
            r.quaternion()
        );
        let mut num_incorrect = 0;
        for m in r.mappings {
            eprintln!(
                "   idx:{} '{}' id:{} mag:{} ordering:{} qual:{}",
                m.img_index,
                STARS_ON_IMAGE[m.img_index].1,
                catalog[m.star].id(),
                catalog[m.star].magnitude(),
                m.ordering,
                m.quality
            );
            if STARS_ON_IMAGE[m.img_index].0 == catalog[m.star].id() {
                num_incorrect += 1;
            }
        }
        if num_incorrect == 0 {
            found = true;
        }
        eprintln!();
    }
    eprintln!("Was the desired answer found? {found}");
    assert!(found, "Desired star set was not found");
    Ok(())
}

// Caph is HIP 746, in Cassiopeia; Epsilon Cassiopeia (the furthest from Capg) is HIP 8886
//
// For magnitude 5, there are 33 stars as close or closer to Caph (including itself...) as HIP 8886
#[cfg(feature = "hipp_bright")]
#[test]
fn test_find_stars_around_caph() -> Result<(), Box<dyn Error>> {
    let magnitude = 5.0;
    let mut catalog = Catalog::load_catalog("hipp_bright", magnitude)?;
    catalog.sort();
    catalog.derive_data();

    let caph = &catalog[catalog.find_id_or_name("746")?];
    assert_eq!(caph.id(), 746);
    let caph_v: Vec3 = caph.vector().into();

    // Look for all stars with 13.5 degrees of Caph (angle between Caph and HIP8886 is 13.261+ degrees)
    let max_angle = 0.23561944901923448;
    let mut found_hip_8886 = false;
    for s in catalog.find_stars_around(caph.vector(), max_angle) {
        found_hip_8886 |= catalog[s].id() == 8886;
        let s_vec: FArray<f64, 3> = catalog[s].vector().into();
        assert!(caph_v.dot(s_vec) >= max_angle.cos());
    }
    assert!(found_hip_8886);
    assert_eq!(
        catalog
            .find_stars_around(caph.vector(), max_angle)
            .iter()
            .count(),
        33
    );

    // Should not find HIP 8886 with tighter angle delta
    let max_angle = max_angle * 0.95;
    let mut found_hip_8886 = false;
    for s in catalog.find_stars_around(caph.vector(), max_angle) {
        found_hip_8886 |= catalog[s].id() == 8886;
        let s_vec: FArray<f64, 3> = catalog[s].vector().into();
        assert!(caph_v.dot(s_vec) >= max_angle.cos());
    }
    assert!(!found_hip_8886);
    assert_eq!(
        catalog
            .find_stars_around(caph.vector(), max_angle)
            .iter()
            .count(),
        28
    );

    Ok(())
}

#[cfg(feature = "hipp_bright")]
#[test]
fn test_find_stars_around_caph_all_bright() -> Result<(), Box<dyn Error>> {
    let magnitude = 15.0;
    let mut catalog = Catalog::load_catalog("hipp_bright", magnitude)?;
    catalog.sort();
    catalog.derive_data();

    let caph = &catalog[catalog.find_id_or_name("746")?];
    assert_eq!(caph.id(), 746);
    let caph_v: Vec3 = caph.vector().into();

    // Look for all stars with 13.5 degrees of Caph (angle between Caph and HIP8886 is 13.261+ degrees)
    let max_angle = 0.23561944901923448;
    let mut found_hip_8886 = false;
    for s in catalog.find_stars_around(caph.vector(), max_angle) {
        found_hip_8886 |= catalog[s].id() == 8886;
        let s_vec: FArray<f64, 3> = catalog[s].vector().into();
        assert!(caph_v.dot(s_vec) >= max_angle.cos());
    }
    assert!(found_hip_8886);
    assert_eq!(
        catalog
            .find_stars_around(caph.vector(), max_angle)
            .iter()
            .count(),
        843
    );

    // Should not find HIP 8886 with tighter angle delta
    let max_angle = max_angle * 0.95;
    let mut found_hip_8886 = false;
    for s in catalog.find_stars_around(caph.vector(), max_angle) {
        found_hip_8886 |= catalog[s].id() == 8886;
        let s_vec: FArray<f64, 3> = catalog[s].vector().into();
        assert!(caph_v.dot(s_vec) >= max_angle.cos());
    }
    assert!(!found_hip_8886);
    assert_eq!(
        catalog
            .find_stars_around(caph.vector(), max_angle)
            .iter()
            .count(),
        759
    );

    Ok(())
}
