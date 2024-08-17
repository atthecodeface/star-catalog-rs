use std::error::Error;

use geo_nd::Vector;

use star_catalog::{Subcube, Vec3};

#[test]
fn test_on_sphere() -> Result<(), Box<dyn Error>> {
    let s: Subcube = Subcube::of_vector(&[0.01, 0.01, 0.01].into());
    for sn in s.iter_neighbors() {
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
                deltas.push([delta * x as f32, delta * y as f32, delta * z as f32].into());
            }
        }
    }

    for sn in Subcube::iter_all() {
        let c = sn.center();
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
        if d > 1.001 * (3.0_f32).sqrt() / Subcube::ELE_PER_SIDE as f32 {
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
