use std::error::Error;

use star_catalog::{iau, Catalog, Subcube};

#[test]
fn test_iau() -> Result<(), Box<dyn Error>> {
    const IGNORE_IDS: &[usize] = &[
        90004, 29550, 57820, 72845, 118319, 13993, 6643, 48711, 107251, 12191, 37284, 1547, 3479,
        54158, 66047, 114322, 5529, 2247, 84832, 61177, 42446, 21109, 55174, 80076, 47087, 15578,
        108375, 47202, 76351, 92895, 12961, 66192, 110813, 30860, 30905, 52521, 82651, 72339,
        117291, 22491, 40687, 48235, 5054, 32916, 74961, 31895, 13192, 99711, 88414, 116084, 83547,
        81022, 56572, 106824, 55664, 79431, 95262, 104780, 43674, 110458, 38041, 17096, 57291,
        96078, 87937, 80838, 70890,
    ];
    let (_is_hipparcos, s) = match std::fs::read_to_string("hipparcos.json") {
        Ok(s) => (true, s),
        Err(_s) => (false, std::fs::read_to_string("hipparcos_mag7.json")?),
    };
    // let s = std::fs::read_to_string("hipparcos.json")?;
    let mut catalog: Catalog = serde_json::from_str(&s)?;
    catalog.sort();
    catalog.derive_data();
    eprintln!("Loaded {} stars", catalog.len());
    for (name, opt_id, ra, de) in iau::NAMES_AND_RA_DE.iter() {
        if let Some(id) = opt_id {
            if IGNORE_IDS.contains(id) {
                continue;
            }
        }
        let ra = *ra / 180.0 * std::f64::consts::PI;
        let de = *de / 180.0 * std::f64::consts::PI;
        let subcube_iter = Subcube::iter_all();
        let (c, star) = catalog.closest_to_ra_de(subcube_iter, ra, de).unwrap();
        let found_id = catalog[star].id();
        if let Some(iau_id) = *opt_id {
            assert!(
                iau_id == found_id,
                "Mismatch in IAU id {iau_id} found {found_id}"
            );
            assert!(
                (c > 0.999999),
                "Angle between found star {found_id} and IAU star {name} too big"
            );
        } else {
            assert!( c <= 0.999999,
"IAU named star {name} has no HIP ID but Hipparcos catalog entry found to be {found_id} witth cos {c}");
        }
    }
    Ok(())
}
