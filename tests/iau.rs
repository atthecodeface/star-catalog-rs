use std::error::Error;

use star_catalog::{iau, Catalog};

#[test]
fn test_iau() -> Result<(), Box<dyn Error>> {
    let s = std::fs::read_to_string("hipparcos.json")?;
    let mut catalog: Catalog = serde_json::from_str(&s)?;
    catalog.sort();
    catalog.derive_data();
    eprintln!("Loaded {} stars", catalog.len());
    for (name, opt_id, ra, de) in iau::NAMES_AND_RA_DE.iter() {
        let ra = (*ra as f64) / 180.0 * std::f64::consts::PI;
        let de = (*de as f64) / 180.0 * std::f64::consts::PI;
        let (c, star) = catalog.closest_to(ra, de).unwrap();
        let found_id = catalog.star(star).id();
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
            assert!( !(c > 0.999999),
"IAU named star {name} has no HIP ID but Hipparcos catalog entry found to be {found_id} witth cos {c}");
        }
    }
    Ok(())
}
