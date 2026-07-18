use std::collections::HashSet;
use std::error::Error;

use geo_nd::{FArray, Vector};

use star_catalog::{Subcube, Vec3};

#[test]
fn test_on_sphere() -> Result<(), Box<dyn Error>> {
    let s: Subcube = Subcube::of_vector(&[0.01, 0.01, 0.01]);
    for sn in s.iter_range(1) {
        assert!(
            !sn.may_be_on_sphere(),
            "Center and its neighbors cannot be on the sphere"
        );
    }

    let mut deltas: Vec<Vec3> = vec![];
    let delta = Subcube::SUBCUBE_SIZE / 2.0;
    for x in [-1, 1] {
        for y in [-1, 1] {
            for z in [-1, 1] {
                deltas.push([delta * x as f64, delta * y as f64, delta * z as f64].into());
            }
        }
    }

    for sn in Subcube::iter_all() {
        let c: Vec3 = sn.center().into();
        let xyz: (usize, usize, usize) = sn.into();
        let l = c.length();
        let mut l_min = l;
        let mut l_max = l;
        for d in &deltas {
            let l = (c + *d).length();
            l_min = l_min.min(l);
            l_max = l_max.max(l);
        }
        let m = sn.may_be_on_sphere();
        let d = (l - 1.0).abs();

        // may be on sphere does some sandbagging hence 1.001
        if d > 1.001 * (3.0_f64).sqrt() / Subcube::ELE_PER_SIDE as f64 {
            assert!(
                !m,
                "Subcube {xyz:?} {c} {l} with l min/max of {l_min} {l_max} should not possibly be on sphere"
            );
        } else {
            assert!(
                m,
                "Subcube {xyz:?} {c} {l} with l min/max of {l_min} {l_max} might be possibly on sphere"
            );
        }
    }
    // assert!(false);
    Ok(())
}

#[test]
fn test_all() -> Result<(), Box<dyn Error>> {
    let c = Subcube::iter_all().count();
    assert_eq!(
        c,
        Subcube::ELE_PER_SIDE * Subcube::ELE_PER_SIDE * Subcube::ELE_PER_SIDE
    );
    Ok(())
}

#[test]
fn test_iter_sphere() -> Result<(), Box<dyn Error>> {
    let all_subcubes: Vec<Subcube> = Subcube::iter_all().collect();
    let num_maybe_on_sphere = all_subcubes.iter().filter(|s| s.may_be_on_sphere()).count();
    let mut sphere_subcubes = HashSet::new();
    for s in Subcube::iter_sphere() {
        let (x, y, z) = s.into();
        if x < 16 && y < 16 && z < 16 {
            eprintln!("{x},{y},{z}");
        }
        let inserted = sphere_subcubes.insert(s);
        assert!(
            inserted,
            "There should be no duplicates in the sphere subcube iterator"
        );
    }
    // Print out any subcubes that maybe on the sphere (in all) that this has not detected
    //
    // If there are any, then this has failed
    let mut num_on_sphere_missed = 0;
    for s in all_subcubes.iter() {
        if s.may_be_on_sphere() != sphere_subcubes.contains(s) {
            num_on_sphere_missed += 1;
            let (x, y, z) = s.into();
            if x < 16 && y < 16 && z < 16 {
                if s.may_be_on_sphere() {
                    eprintln!("Sphere subcubes does not contain {x},{y},{z} as it should");
                } else {
                    eprintln!("Sphere subcubes contains {x},{y},{z} when it should not");
                }
            }
        }
    }
    // Check that the *number* match
    assert_eq!(
        sphere_subcubes.len(),
        num_maybe_on_sphere,
        "Expected the number of subcubes truly that maybe on the sphere to match the number returned from the iterator"
    );
    assert_eq!(
        num_on_sphere_missed, 0,
        "Expected to have missed no subcubes that are maybe on the sphere in 'all'"
    );
    Ok(())
}

#[test]
fn test_iter_sphere_of_vector() -> Result<(), Box<dyn Error>> {
    let sphere_subcubes: HashSet<Subcube> = Subcube::iter_sphere().collect();
    let tests: &[(Vec3, f64)] = &[
        ([1.0, 0.0, 0.0].into(), 0.5_f64),
        ([1.0, 1.0, 0.0].into(), 0.05_f64),
        ([1.0, 0.0, 1.0].into(), 0.05_f64),
        ([1.0, 0.0, 1.0].into(), 0.75_f64),
        ([0.3, 0.4, 1.0].into(), 0.95_f64),
        ([1.0, 0.0, 1.0].into(), 0.15_f64),
    ];
    for (v, cos) in tests {
        let mut num_outside_area = 0;
        let v_unit = v.normalize();
        for s in Subcube::iter_sphere_within_cos_of_vector(&v_unit, *cos) {
            assert!(
                sphere_subcubes.contains(&s),
                "Subcube {s:?} is apparently not maybe on the sphere"
            );
            let s_vec: FArray<f64, 3> = s.center().into();
            let actual_cos = v.dot(s_vec);
            if actual_cos < *cos {
                num_outside_area += 1;
                eprintln!(
                    "Probably not expecting subcube {s:?} with actual cos {actual_cos} compared to 'min' of {cos}"
                );
            }
        }
        assert_eq!(num_outside_area, 0);
    }
    Ok(())
}
