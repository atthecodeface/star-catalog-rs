//! # Hipparcos data and catalog reading
//!
//! The Hipparcos catalog is available as 'hipparcos-voidmain.csv';
//! this library provides a means for reading that in and creating a
//! [Catalog] from it.
//!
//! This also provides a small HIP_ALIASES constant that maps some
//! Hipparcos stars to common names.
//!

//a Imports
#[cfg(feature = "csv")]
use crate::{Catalog, Star};

//ti Record
///
/// Fields in hipparcos main CSV file
///
/// Catalog, HIP, Proxy, RAhms, DEdms
/// Vmag, VarFlag, r_Vmag, RAdeg, DEdeg,
/// AstroRef, Plx, pmRA, pmDE,
/// e_RAdeg, e_DEdeg, e_Plx, e_pmRA, e_pmDE,
/// DE:RA, Plx:RA, Plx:DE,
/// pmRA:RA, pmRA:DE, pmRA:Plx,
/// pmDE:RA, pmDE:DE, pmDE:Plx, pmDE:pmRA,
/// F1, F2, ---,
/// BTmag, e_BTmag, VTmag, e_VTmag, m_BTmag,
/// B-V, e_B-V, r_B-V, V-I, e_V-I, r_V-I,
/// CombMag, Hpmag, e_Hpmag,
/// Hpscat, o_Hpmag, m_Hpmag, Hpmax, HPmin,
/// Period, HvarType, moreVar, morePhoto,
/// CCDM, n_CCDM, Nsys, Ncomp, MultFlag,
/// Source, Qual, m_HIP, theta, rho, e_rho,
/// dHp, e_dHp, Survey, Chart, Notes,
/// HD, BD, CoD, CPD, (V-I)red, SpType, r_SpType
///
/// For example, Polaris (HIP111767) has;
///
/// H,11767,,02 31 47.08,+89 15 50.9,1.97,1,H,
///   037.94614689,+89.26413805,,7.56,44.22,
///   -11.74,0.39,0.45,0.48,0.47,0.55,-0.16,
///   0.05,0.27,-0.01,0.08,0.05,0.04,-0.12,
///   -0.09,-0.36,1,1.22,11767,2.756,0.003,
///   2.067,0.003,,0.636,0.003,
///   T,0.70,0.00,
///   L,,2.1077,0.0021,0.014,102,,2.09,2.13,
///   3.97,P,1,A,02319+8915,I,1,1,,,,,,,,,,
///   S,,P,8890,B+88    8,,,0.68,F7:Ib-IIv SB,G
///
/// And Dubhe (in Ursa Major) has:
///
/// H,54061,H,11 03 43.84,+61 45 04.0,1.81,,H,
///   165.93265365,+61.75111888,A,26.38,
///   -136.46,-35.25,0.40,0.46,0.53,0.43,
///   0.45,0.19,0.08,-0.17,-0.13,0.00,-0.08,
///   -0.02,-0.30,0.00,0.14,4,2.15,54061,
///   3.185,0.002,1.934,0.003,*,1.061,0.003,
///   T,1.03,0.00,
///   L,*,1.9519,0.0004,0.005,158,*,1.94,1.96,,
///   D,,,11037+6145,I,1,2,C,,A,AB,270,
///   0.672,0.006,2.92,0.03,
///   S,,,95689,B+62 1161,,,1.04,F7V comp,G
///
/// light years = 3.26156 / parallax in arc-seconds
///
/// Polaris = 3.26156 / 7.56E-3 = 431 light years (3s.f.)
/// Dubhe = 3.26156 / 26.38E-3 = 124 light years (3s.f.)
#[cfg(feature = "csv")]
#[derive(Debug, serde::Deserialize)]
struct Record {
    #[serde(rename = "HIP")]
    hip: Option<usize>,
    /// Right ascension in degrees
    #[serde(rename = "RAdeg")]
    ra: Option<f64>,
    /// Declination in degrees
    #[serde(rename = "DEdeg")]
    de: Option<f64>,
    /// Parallax
    #[serde(rename = "Plx")]
    plx: Option<f32>,
    /// Visual magnitude
    #[serde(rename = "Vmag")]
    mag: Option<f32>,
    /// Blue-violet luminance delta
    #[serde(rename = "B-V")]
    b_v: Option<f32>,
}

//fp read_to_catalog
/// Read stars from a Hipparcos CSV file (or anything that supports
/// std::io::Read) and add them to a [Catalog]
///
/// Only include those with a visual magnitude lower than a certain value
///
/// This is designed to read from 'hipparcos-voidmain.csv' (118,218 stars)
///
/// This ignores stars from the catalog that do not have all of the
/// required records; there are then 116,812 valid stars of any visual
/// magnitude
///
/// This requires the 'csv' feature
#[cfg(feature = "csv")]
pub fn read_to_catalog<R: std::io::Read>(
    catalog: &mut Catalog,
    reader: R,
    max_mag: f32,
) -> Result<(), csv::Error> {
    let mut csv_reader = csv::Reader::from_reader(reader);
    for result in csv_reader.deserialize() {
        let record: Record = result?;
        if record.hip.is_some()
            && record.ra.is_some()
            && record.de.is_some()
            && record.plx.is_some()
            && record.mag.is_some()
            && record.b_v.is_some()
        {
            let mag = record.mag.unwrap();
            if mag > max_mag {
                continue;
            }
            let hip = record.hip.unwrap();
            let ra = record.ra.unwrap() / 180.0 * std::f64::consts::PI;
            let de = record.de.unwrap() / 180.0 * std::f64::consts::PI;
            let ly = 3.26156E3 / record.plx.unwrap();
            let ly = if ly.is_normal() { ly } else { 0.0 };
            let b_v = record.b_v.unwrap();
            let star = Star::new(hip, ra, de, ly, mag, b_v);
            catalog.add_star(star);
        }
    }
    Ok(())
}

//cp HIP_ALIASES
/// Aliases of HIP identifiers to common names of stars
pub const HIP_ALIASES: &[(usize, &str)] = &[
    (677, "Alpheratz"),
    (746, "Caph"),
    (1067, "Algenib"),
    (2081, "Ankaa"),
    (3179, "Shedir"),
    (3419, "Diphda"),
    (3829, "Van Maanen 2"),
    (5447, "Mirach"),
    (7588, "Achernar"),
    (9640, "Almaak"),
    (9884, "Hamal"),
    (10826, "Mira"),
    (11767, "Polaris"),
    (13847, "Acamar"),
    (14135, "Menkar"),
    (14576, "Algol"),
    (15863, "Mirphak"),
    (17702, "Alcyone"),
    (17851, "Pleione"),
    (18543, "Zaurak"),
    (21421, "Aldebaran"),
    (24186, "Kapteyn's star"),
    (24436, "Rigel"),
    (24608, "Capella"),
    (25336, "Bellatrix"),
    (25428, "Alnath"),
    (25606, "Nihal"),
    (25930, "Mintaka"),
    (25985, "Arneb"),
    (26311, "Alnilam"),
    (26727, "Alnitak"),
    (27366, "Saiph"),
    (27989, "Betelgeuse"),
    (30089, "Red Rectangle"),
    (30438, "Canopus"),
    (31681, "Alhena"),
    (32349, "Sirius"),
    (33579, "Adhara"),
    (36208, "Luyten's star"),
    (36850, "Castor"),
    (37279, "Procyon"),
    (37826, "Pollux"),
    (46390, "Alphard"),
    (49669, "Regulus"),
    (50583, "Algieba"),
    (53910, "Merak"),
    (54061, "Dubhe"),
    (57632, "Denebola"),
    (57939, "Groombridge 1830"),
    (58001, "Phad"),
    (59774, "Megrez"),
    (60718, "Acrux"),
    (60936, "3C 273"),
    (62956, "Alioth"),
    (63125, "Cor Caroli"),
    (63608, "Vindemiatrix"),
    (65378, "Mizar"),
    (65474, "Spica"),
    (65477, "Alcor"),
    (67301, "Alkaid"),
    (68702, "Agena"),
    (68702, "Hadar"),
    (68756, "Thuban"),
    (69673, "Arcturus"),
    (70890, "Proxima"),
    (71683, "Rigil Kent"),
    (72105, "Izar"),
    (72607, "Kocab"),
    (76267, "Alphekka"),
    (77070, "Unukalhai"),
    (80763, "Antares"),
    (84345, "Rasalgethi"),
    (85927, "Shaula"),
    (86032, "Rasalhague"),
    (87833, "Etamin"),
    (87937, "Barnard's star"),
    (90185, "Kaus Australis"),
    (91262, "Vega"),
    (92420, "Sheliak"),
    (92855, "Nunki"),
    (95947, "Albireo"),
    (96295, "Campbell's star"),
    (97278, "Tarazed"),
    (97649, "Altair"),
    (98036, "Alshain"),
    (98298, "Cyg X-1"),
    (102098, "Deneb"),
    (105199, "Alderamin"),
    (107315, "Enif"),
    (109074, "Sadalmelik"),
    (109268, "Alnair"),
    (110893, "Kruger 60"),
    (112247, "Babcock's star"),
    (113368, "Fomalhaut"),
    (113881, "Scheat"),
    (113963, "Markab"),
];
