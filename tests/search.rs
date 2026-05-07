use std::error::Error;

use geo_nd::FArray;
use star_catalog::{Catalog, hipparcos};
use star_catalog::{StarTriangleMatch, StarTriangleSearch, Subcube};

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
 * 1775.112310638358, 1776.5027160418604 HI4427
​
1: Array [ 3278.0984711515625, 2101.7363607401962 ]
​
2: Array [ 2744.0131303397543, 741.4831578035513 ]
​
3: Array [ 2623.556301560086, 663.5877418593657 ]
​
4: Array [ 2797.0141350028084, 675.6334247373326 ]
​
5: Array [ 2660.4963957191844, 796.0902535170011 ]
​
6: Array [ 2756.0588132177213, 993.6394527156574 ]
​
7: Array [ 2907.031371954906, 731.8466115011778 ]
​
8: Array [ 2157.7898969453677, 1195.2038795403028 ]
​
9: Array [ 2026.790212632814, 1486.551562888918 ] Caph
​
10: Array [ 3788.072160250552, 1668.7084067303113 ]
​
11: Array [ 3768.38880993417, 2159.1321611844974 ]
​
12: Array [ 2969.354741620304, 2479.8118138032387 ]
​
13: Array [ 2869.3800887802986, 2329.8498345432304 ]
​
14: Array [ 2410.3221645011904, 2548.1431132214075 ]
​
15: Array [ 2438.7553226483483, 2496.3213895015883 ]
​
16: Array [ 1751.4663267099875, 2448.325325765358 ]
​
17: Array [ 2045.7532882082467, 2477.9184280389254 ]
​
18: Array [ 2182.8137962211354, 2513.5994445149945 ]
​
19: Array [ 3863.8068532531424, 1836.256076544863 ]
​
20: Array [ 2870.492724486179, 1452.4758915567604 ]
​
21: Array [ 3059.5619285986245, 1513.5206570101059 ]
​
22: Array [ 2988.3430355697215, 1519.4555647625143 ]
​
23: Array [ 2846.753093476545, 1542.347351807519 ]
​
24: Array [ 2577.294186604003, 2093.026531464874 ]
​
25: Array [ 2501.138907906271, 2067.0955962588723 ]
​
26: Array [ 2391.426729551264, 1978.515602901052 ]
​
27: Array [ 2334.753024562189, 2023.8814309168151 ]
​
28: Array [ 3175.2975749023262, 2041.3488949845691 ]
​
29: Array [ 1675.626164299013, 1978.6915504053422 ]
​
30: Array [ 1686.4967383943324, 2068.1408458182564 ]
​
31: Array [ 1802.6565872986025, 2017.5150293171973
*/

#[allow(dead_code)]
const STARS_ON_IMAGE: &[(usize, &str, u32, u32)] = &[
    (0, "HI4427", 1775, 1778),
    (0, "Caph HI746", 2026, 1486),
    (0, "HI112158", 4072, 1319),
    (0, "Scheat HI113881", 3984, 1006),
    (0, "Schedir", 2064, 1781),
    (0, "HI6686", 1677, 1979),
    (0, "Alpheratz", 3776, 2159),
    (0, "Mirach", 3038, 2709),
    (0, "Almaak", 2302, 2979),
    /**/
];

fn vector_of_img(x: u32, y: u32) -> FArray<f64, 3> {
    let w: f64 = 5184.0;
    let h: f64 = 3456.0;
    let mm_equiv: f64 = 24.1;

    //  tan_hfovh = 18 / mm_equiv;
    //  fovh = 2 * Math.atan(tan_hfovh);
    //  tan_hfovh = Math.tan(this.fovh / 2);

    let tan_hvofh = (36.0 / 2.0) / mm_equiv;

    let xf = ((x as f64) - w / 2.0) / w;
    let yf = ((y as f64) - h / 2.0) / w; // Note this should be w

    // sensor_rf is relative distance from the centre, so +-0.5 for the Y=0, X=+-w/2 edges
    let sensor_rf = (xf * xf + yf * yf).sqrt();
    let sensor_yaw = (sensor_rf * 2.0 * tan_hvofh).atan();
    let roll = yf.atan2(xf);
    let world_yaw = sensor_yaw;
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

    // [0.9047149782456791, 0.3694044272055461, 0.2122055072302855]
    // [0.9414851217769051, 0.15355018987025365, 0.3000468374496476]
    // [0.871569089040238, -0.37169267282721086, 0.31970592737097253]
    // [0.8544379323052361, -0.3427206090850877, 0.3904848319029808]
    // [0.9632503106951024, 0.1465526060531968, 0.22510258240367917]
    // [0.9531523200454994, 0.2513065850742371, 0.1683616793994615]
    // [0.9399039619564755, -0.3206680183715039, 0.11727132766457873]
    // [0.9912887491716079, -0.12739591462959227, -0.03342000451045355]
    // [0.990429949696611, 0.08276414402144343, -0.11044732322861589]
    //
    // 6.190846898801532 32.306172778472444 36.946847293065176
    //
    // 4427 to 746 is 6.147565179199756 degrees
    // 746 to 112158 is 32.406063021307645 degrees
    // 4427 to 112158 is 37.649680798045566 degrees
    //
    let a01 = geo_nd::vector::dot(&img_vectors[0], &img_vectors[1]).acos() * 180.0 / 3.1415926;
    let a12 = geo_nd::vector::dot(&img_vectors[1], &img_vectors[2]).acos() * 180.0 / 3.1415926;
    let a20 = geo_nd::vector::dot(&img_vectors[2], &img_vectors[0]).acos() * 180.0 / 3.1415926;
    eprintln!("{a01} {a12} {a20}");

    let s = Subcube::iter_all();

    let max_angle_delta = 0.9 / 180.0 * 3.14159;
    let (finished, mut results) =
        catalog.find_best_star_mappings(s, &img_vectors, max_angle_delta, usize::MAX);
    assert!(finished, "Expected to have tried all candidates");

    // View to ECEF is rijk 0.7464623291490128, 0.5184452038309155, -0.3327355431837139,  0.2515862080647313

    results.sort_by(|a, b| a.angle_sd.partial_cmp(&b.angle_sd).unwrap());
    for r in results {
        eprintln!(
            "{} {:?}",
            r.initial_match.angle_sum(),
            r.initial_match.quaternion(),
        );
        eprintln!(
            "{} {} {:?}",
            r.angle_mean * 180.0 / 3.14159265,
            r.angle_sd * 180.0 / 3.14159265,
            r.quaternion()
        );
        for m in r.mappings {
            eprintln!(
                "   idx:{} id:{} mag:{} ordering:{} qual:{}",
                m.img_index,
                catalog[m.star].id(),
                catalog[m.star].magnitude(),
                m.ordering,
                m.quality
            );
        }
        eprintln!();
    }
    assert!(false);
    Ok(())
}
