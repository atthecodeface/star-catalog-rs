use std::error::Error;

use star_catalog::{hipparcos, Catalog};

// This test is used to create hipparcos.json from hipparcos-voidmain.csv
// #[test]
// fn test_create_hipparcoss_json() -> Result<(), Box<dyn Error>> {
//     use std::io::Write;
//     let mut catalog = Catalog::default();
//     let f = std::fs::File::open("hipparcos-voidmain.csv")?;
//     hipparcos::read_to_catalog(&mut catalog, &f, 116.0)?;
//     drop(f);
//     let mut f = std::fs::File::create("hipparcos.json")?;
//     let s = serde_json::to_string(&catalog)?;
//     f.write(s.as_bytes())?;
//     Ok(())
// }

#[test]
fn test_read_hipparcos_json() -> Result<(), Box<dyn Error>> {
    let s = std::fs::read_to_string("hipparcos.json")?;
    let mut catalog: Catalog = serde_json::from_str(&s)?;
    catalog.sort();
    eprintln!("Loaded {} stars", catalog.len());
    catalog.add_names(hipparcos::HIP_ALIASES)?;
    Ok(())
}
