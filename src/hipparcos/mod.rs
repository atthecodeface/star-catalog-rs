//! # Hipparcos data and catalog reading
//!
//! The Hipparcos catalog is available as 'hipparcos-voidmain.csv';
//! this library provides a means for reading that in and creating a
//! [crate::Catalog] from it.
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
///
/// Source: <https://www.cosmos.esa.int/web/hipparcos/common-star-names>
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

pub const HIP_COLLATED_ALIASES: &[(usize, &str)] = &[
    (60936, "3C 273"),
    (13847, "Acamar"),
    (7588, "Achernar"),
    (3821, "Achird"),
    (78820, "Acrab"),
    (60718, "Acrux"),
    (44066, "Acubens"),
    (50335, "Adhafera"),
    (33579, "Adhara"),
    (6411, "Adhil"),
    (68702, "Agena"),
    (20889, "Ain"),
    (92761, "Ainalrami"),
    (94481, "Aladfar"),
    (90004, "Alasia"),
    (94141, "Albaldah"),
    (102618, "Albali"),
    (95947, "Albireo"),
    (59199, "Alchiba"),
    (65477, "Alcor"),
    (17702, "Alcyone"),
    (21421, "Aldebaran"),
    (105199, "Alderamin"),
    (108085, "Aldhanab"),
    (83895, "Aldhibah"),
    (101421, "Aldulfin"),
    (106032, "Alfirk"),
    (100064, "Algedi"),
    (1067, "Algenib"),
    (50583, "Algieba"),
    (14576, "Algol"),
    (60965, "Algorab"),
    (31681, "Alhena"),
    (62956, "Alioth"),
    (102488, "Aljanah"),
    (67301, "Alkaid"),
    (75411, "Alkalurops"),
    (44471, "Alkaphrah"),
    (115623, "Alkarab"),
    (53740, "Alkes"),
    (9640, "Almaak"),
    (23416, "Almaaz"),
    (9640, "Almach"),
    (109268, "Alnair"),
    (88635, "Alnasl"),
    (25428, "Alnath"),
    (26311, "Alnilam"),
    (26727, "Alnitak"),
    (80112, "Alniyat"),
    (46390, "Alphard"),
    (76267, "Alphecca"),
    (76267, "Alphekka"),
    (677, "Alpheratz"),
    (7097, "Alpherg"),
    (83608, "Alrakis"),
    (9487, "Alrescha"),
    (86782, "Alruba"),
    (96100, "Alsafi"),
    (41075, "Alsciaukat"),
    (42913, "Alsephina"),
    (98036, "Alshain"),
    (100310, "Alshat"),
    (97649, "Altair"),
    (94376, "Altais"),
    (46750, "Alterf"),
    (35904, "Aludra"),
    (55219, "Alula Borealis"),
    (92946, "Alya"),
    (32362, "Alzirr"),
    (29550, "Amadioha"),
    (110003, "Ancha"),
    (13288, "Angetenar"),
    (57820, "Aniara"),
    (2081, "Ankaa"),
    (95771, "Anser"),
    (80763, "Antares"),
    (72845, "Arcalís"),
    (69673, "Arcturus"),
    (95294, "Arkab Posterior"),
    (95241, "Arkab Prior"),
    (25985, "Arneb"),
    (93506, "Ascella"),
    (42911, "Asellus Australis"),
    (42806, "Asellus Borealis"),
    (43109, "Ashlesha"),
    (45556, "Aspidiske"),
    (17579, "Asterope"),
    (80331, "Athebyne"),
    (17448, "Atik"),
    (17847, "Atlas"),
    (82273, "Atria"),
    (41037, "Avior"),
    (118319, "Axólotl"),
    (13993, "Ayeyarwady"),
    (107136, "Azelfafage"),
    (13701, "Azha"),
    (38170, "Azmidi"),
    (112247, "Babcock's star"),
    (73136, "Baekdu"),
    (87937, "Barnard's Star"),
    (8645, "Baten Kaitos"),
    (20535, "Beemim"),
    (19587, "Beid"),
    (25336, "Bellatrix"),
    (27989, "Betelgeuse"),
    (13209, "Bharani"),
    (48711, "Bibhā"),
    (109427, "Biham"),
    (107251, "Bosona"),
    (14838, "Botein"),
    (73714, "Brachium"),
    (26380, "Bubup"),
    (12191, "Buna"),
    (106786, "Bunda"),
    (6643, "Bélénos"),
    (96295, "Campbell's star"),
    (30438, "Canopus"),
    (24608, "Capella"),
    (746, "Caph"),
    (36850, "Castor"),
    (4422, "Castula"),
    (86742, "Cebalrai"),
    (37284, "Ceibo"),
    (17489, "Celaeno"),
    (86796, "Cervantes"),
    (53721, "Chalawan"),
    (20894, "Chamukuy"),
    (61317, "Chara"),
    (99894, "Chechia"),
    (54879, "Chertan"),
    (1547, "Citadelle"),
    (33719, "Citalá"),
    (3479, "Cocibolca"),
    (43587, "Copernicus"),
    // Note tthat Cor Caroli is a binary star with HIP 63121
    (63125, "Cor Caroli"),
    (63121, "Cor Caroli 2"),
    (80463, "Cujam"),
    (23875, "Cursa"),
    (98298, "Cyg X-1"),
    (100345, "Dabih"),
    (14879, "Dalim"),
    (107556, "Deneb Algedi"),
    (102098, "Deneb"),
    (57632, "Denebola"),
    (64241, "Diadem"),
    (54158, "Dingolay"),
    (3419, "Diphda"),
    (66047, "Dofida"),
    (78401, "Dschubba"),
    (54061, "Dubhe"),
    (86614, "Dziban"),
    (114322, "Ebla"),
    (75458, "Edasich"),
    (17499, "Electra"),
    (70755, "Elgafar"),
    (29034, "Elkurud"),
    (25428, "Elnath"),
    (87833, "Eltanin"),
    (5529, "Emiw"),
    (107315, "Enif"),
    (116727, "Errai"),
    (87833, "Etamin"),
    (90344, "Fafnir"),
    (78265, "Fang"),
    (97165, "Fawaris"),
    (48615, "Felis"),
    (2247, "Felixvarela"),
    (113368, "Fomalhaut"),
    (56508, "Formosa"),
    (84832, "Franz"),
    (2920, "Fulu"),
    (113889, "Fumalsamakah"),
    (61177, "Funi"),
    (30122, "Furud"),
    (87261, "Fuyue"),
    (61084, "Gacrux"),
    (42446, "Gakyid"),
    (56211, "Giausar"),
    (59803, "Gienah"),
    (60260, "Ginan"),
    (36188, "Gomeisa"),
    (57939, "Groombridge 1830"),
    (87585, "Grumium"),
    (77450, "Gudja"),
    (84405, "Guniibuu"),
    (68702, "Hadar"),
    (23767, "Haedus"),
    (9884, "Hamal"),
    (23015, "Hassaleh"),
    (26241, "Hatysa"),
    (113357, "Helvetios"),
    (66249, "Heze"),
    (21109, "Hoggar"),
    (112029, "Homam"),
    (55174, "Hunahpú"),
    (80076, "Hunor"),
    (78104, "Iklil"),
    (47087, "Illyrian"),
    (59747, "Imai"),
    (84787, "Inquill"),
    (15578, "Intan"),
    (46471, "Intercrus"),
    (108375, "Itonda"),
    (72105, "Izar"),
    (79374, "Jabbah"),
    (37265, "Jishui"),
    (12706, "Kaffaljidhma"),
    (47202, "Kalausi"),
    (79219, "Kamuy"),
    (69427, "Kang"),
    (24186, "Kapteyn's star"),
    (76351, "Karaka"),
    (90185, "Kaus Australis"),
    (90496, "Kaus Borealis"),
    (89931, "Kaus Media"),
    (92895, "Kaveh"),
    (19849, "Keid"),
    (69974, "Khambalia"),
    (104987, "Kitalpha"),
    (72607, "Kocab"),
    (72607, "Kochab"),
    (12961, "Koeia"),
    (80816, "Kornephoros"),
    (61359, "Kraz"),
    (110893, "Kruger 60"),
    (108917, "Kurhah"),
    (62223, "La Superba"),
    (82396, "Larawag"),
    (85696, "Lesath"),
    (97938, "Libertas"),
    (66192, "Liesma"),
    (13061, "Lilii Borea"),
    (110813, "Lionrock"),
    (30860, "Lucilinburhuc"),
    (30905, "Lusitânia"),
    (36208, "Luyten's star"),
    (85693, "Maasym"),
    (52521, "Macondo"),
    (24003, "Mago"),
    (28380, "Mahasim"),
    (82651, "Mahsati"),
    (17573, "Maia"),
    (80883, "Marfik"),
    (113963, "Markab"),
    (45941, "Markeb"),
    (79043, "Marsic"),
    (112158, "Matar"),
    (32246, "Mebsuta"),
    (59774, "Megrez"),
    (26207, "Meissa"),
    (34088, "Mekbuda"),
    (42556, "Meleph"),
    (28360, "Menkalinan"),
    (14135, "Menkar"),
    (68933, "Menkent"),
    (18614, "Menkib"),
    (53910, "Merak"),
    (72487, "Merga"),
    (94114, "Meridiana"),
    (17608, "Merope"),
    (8832, "Mesarthim"),
    (45238, "Miaplacidus"),
    (62434, "Mimosa"),
    (42402, "Minchir"),
    (63090, "Minelauva"),
    (25930, "Mintaka"),
    (10826, "Mira"),
    (5447, "Mirach"),
    (13268, "Miram"),
    (15863, "Mirfak"),
    (15863, "Mirphak"),
    (30324, "Mirzam"),
    (14668, "Misam"),
    (65378, "Mizar"),
    (117291, "Morava"),
    (8796, "Mothallah"),
    (22491, "Mouhoun"),
    (34045, "Muliphein"),
    (67927, "Muphrid"),
    (41704, "Muscida"),
    (103527, "Musica"),
    (72339, "Mönch"),
    (44946, "Nahn"),
    (39429, "Naos"),
    (106985, "Nashira"),
    (48235, "Natasha"),
    (73555, "Nekkar"),
    (7607, "Nembus"),
    (5054, "Nenque"),
    (32916, "Nervia"),
    (25606, "Nihal"),
    (74961, "Nikawiy"),
    (31895, "Nosaxa"),
    (92855, "Nunki"),
    (75695, "Nusakan"),
    (13192, "Nushagak"),
    (40687, "Násti"),
    (80838, "Ogma"),
    (93747, "Okab"),
    (81266, "Paikauhale"),
    (100751, "Peacock"),
    (26634, "Phact"),
    (58001, "Phad"),
    (58001, "Phecda"),
    (75097, "Pherkad"),
    (99711, "Phoenicia"),
    (40881, "Piautos"),
    (88414, "Pincoya"),
    (82545, "Pipirima"),
    (17851, "Pleione"),
    (116084, "Poerava"),
    (104382, "Polaris Australis"),
    (11767, "Polaris"),
    (89341, "Polis"),
    (37826, "Pollux"),
    (61941, "Porrima"),
    (53229, "Praecipua"),
    (20205, "Prima Hyadum"),
    (37279, "Procyon"),
    (29655, "Propus"),
    (70890, "Proxima Centauri"),
    (70890, "Proxima"),
    (16537, "Ran"),
    (17378, "Rana"),
    (83547, "Rapeto"),
    (48455, "Rasalas"),
    (84345, "Rasalgethi"),
    (86032, "Rasalhague"),
    (85670, "Rastaban"),
    (30089, "Red Rectangle"),
    (49669, "Regulus"),
    (5737, "Revati"),
    (24436, "Rigel"),
    (71683, "Rigil Kent"),
    (71681, "Rigil Kentaurus"),
    (81022, "Rosaliadecastro"),
    (101769, "Rotanev"),
    (6686, "Ruchbah"),
    (95347, "Rukbat"),
    (84012, "Sabik"),
    (23453, "Saclateni"),
    (110395, "Sadachbia"),
    (112748, "Sadalbari"),
    (109074, "Sadalmelik"),
    (109074, "Sadalmelik"),
    (106278, "Sadalsuud"),
    (100453, "Sadr"),
    (56572, "Sagarmatha"),
    (27366, "Saiph"),
    (115250, "Salm"),
    (86228, "Sargas"),
    (84379, "Sarin"),
    (21594, "Sceptrum"),
    (113881, "Scheat"),
    (3179, "Schedar"),
    (20455, "Secunda Hyadum"),
    (8886, "Segin"),
    (71075, "Seginus"),
    (96757, "Sham"),
    (55664, "Shama"),
    (79431, "Sharjah"),
    (85927, "Shaula"),
    (3179, "Shedir"),
    (92420, "Sheliak"),
    (8903, "Sheratan"),
    (95262, "Sika"),
    (32349, "Sirius"),
    (111710, "Situla"),
    (113136, "Skat"),
    (104780, "Solaris"),
    (65474, "Spica"),
    (43674, "Stribor"),
    (101958, "Sualocin"),
    (47508, "Subra"),
    (44816, "Suhail"),
    (93194, "Sulafat"),
    (69701, "Syrma"),
    (106824, "Sāmaya"),
    (22449, "Tabit"),
    (110458, "Taika"),
    (57399, "Taiyangshou"),
    (63076, "Taiyi"),
    (44127, "Talitha"),
    (50801, "Tania Australis"),
    (50372, "Tania Borealis"),
    (38041, "Tapecue"),
    (97278, "Tarazed"),
    (40526, "Tarf"),
    (17531, "Taygeta"),
    (40167, "Tegmine"),
    (30343, "Tejat"),
    (98066, "Terebellum"),
    (21393, "Theemin"),
    (68756, "Thuban"),
    (112122, "Tiaki"),
    (26451, "Tianguan"),
    (62423, "Tianyi"),
    (80687, "Timir"),
    (7513, "Titawin"),
    (71681, "Toliman"),
    (58952, "Tonatiuh"),
    (8198, "Torcular"),
    (17096, "Tupi"),
    (60644, "Tupã"),
    (39757, "Tureis"),
    (47431, "Ukdah"),
    (57291, "Uklun"),
    (77070, "Unukalhai"),
    (33856, "Unurgunite"),
    (96078, "Uruk"),
    (3829, "Van Maanen 2"),
    (91262, "Vega"),
    (116076, "Veritate"),
    (63608, "Vindemiatrix"),
    (63608, "Vindemiatrix"),
    (35550, "Wasat"),
    (27628, "Wazn"),
    (34444, "Wezen"),
    (5348, "Wurren"),
    (82514, "Xamidimura"),
    (91852, "Xihe"),
    (69732, "Xuange"),
    (79882, "Yed Posterior"),
    (79593, "Yed Prior"),
    (85822, "Yildun"),
    (60129, "Zaniah"),
    (18543, "Zaurak"),
    (57757, "Zavijava"),
    (48356, "Zhang"),
    (15197, "Zibal"),
    (54872, "Zosma"),
    (72622, "Zubenelgenubi"),
    (76333, "Zubenelhakrabi"),
    (74785, "Zubeneschamali"),
];

#[cfg(feature = "hipp_bright")]
pub const HIPP_BRIGHT_PST: &[u8] = include_bytes!("hipp_bright.pst");

/// Constellations in the norhern hemisphere (Hipparcos ids)
use crate::constellations::DrawingInstruction::*;
use crate::constellations::{Constellation, Drawing, PtIndex};
pub const NORTHERN_HEMISPHERE: &[Constellation] = &[
    Constellation {
        name: "Orion",
        points: &[27366, 26727, 27989, 26207, 25336, 25930, 26311, 24436],

        drawings: &[Drawing {
            // Phecda -> xi -> Alula Borealis (nu) -> ?Alula Australis (eta?)
            // xi -> upsilon -> Tania Australis (mu) -> Tania Borealis (lambda)
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(4)),
                DrawTo(PtIndex(5)),
                DrawTo(PtIndex(6)),
                DrawTo(PtIndex(7)),
                /* */
                MoveTo(PtIndex(1)),
                DrawTo(PtIndex(6)),
                DrawTo(PtIndex(5)),
                /* */
                MoveTo(PtIndex(2)),
                DrawTo(PtIndex(4)),
            ],
        }],
    },
    Constellation {
        name: "Ursa Major",
        points: &[
            59774, 54061, 53910, 58001, 62956, 65378, 67301, 46733, 41704, 48319, 46853, 44471,
            54539, 50801, 50372,
        ],
        drawings: &[Drawing {
            // Megrez -> Dubhe -> Merak -> Phecda -> Megrez ->  Alioth -> Mizar -> Alkaid
            // Dubhe -> ? Muscida -> not-nu -> theta -> Talitha; Merak -> not-nu
            // front leg
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(0)),
                DrawTo(PtIndex(4)),
                DrawTo(PtIndex(5)),
                DrawTo(PtIndex(6)),
                /* */
                MoveTo(PtIndex(1)),
                DrawTo(PtIndex(7)),
                DrawTo(PtIndex(8)),
                DrawTo(PtIndex(9)),
                DrawTo(PtIndex(2)),
                /* */
                MoveTo(PtIndex(9)),
                DrawTo(PtIndex(10)),
                DrawTo(PtIndex(11)),
                /* */
                MoveTo(PtIndex(3)),
                DrawTo(PtIndex(12)),
                DrawTo(PtIndex(13)),
                /* */
                MoveTo(PtIndex(12)),
                DrawTo(PtIndex(14)),
            ],
        }],
    },
    Constellation {
        name: "Cassiopeia",
        points: &[746, 3179, 4427, 6686, 8886],
        drawings: &[Drawing {
            // Caph, Schedar, Tsih, Ruchbah, Segin
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(4)),
            ],
        }],
    },
    Constellation {
        name: "Ursa Minor",
        points: &[11767, 85822, 82080, 77055, 72607, 75097, 79822],
        drawings: &[Drawing {
            // alpha, delta, epsilon, zeta, beta, gamma, eta
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(4)),
                DrawTo(PtIndex(5)),
                DrawTo(PtIndex(6)),
            ],
        }],
    },
    Constellation {
        name: "Leo",
        points: &[
            49669, 54879, 57632, 54872, 50583, 49583, 50335, 48455, 47908,
        ],
        drawings: &[Drawing {
            // Regulus alpha, eta, theta, Denebola beta, delta, gamma, zeta, mu, epsilon, lambda, kappa, mu
            // delta, theta iota, sigma
            // epsilon, eta
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(4)),
                DrawTo(PtIndex(5)),
                DrawTo(PtIndex(0)),
                /* */
                MoveTo(PtIndex(4)),
                DrawTo(PtIndex(6)),
                DrawTo(PtIndex(7)),
                DrawTo(PtIndex(8)),
                /* */
                MoveTo(PtIndex(3)),
                DrawTo(PtIndex(1)),
            ],
        }],
    },
    Constellation {
        name: "Gemini",
        // Note 28437 is not valid in the HIP catalog as we have it
        points: &[
            32362, 35350, 35550, 36962, 36046, 34693, 32246, 30883, 37740, 37826, 36850, 33018,
            34088, 31681, 30343, 28734,
        ],
        drawings: &[Drawing {
            // Regulus alpha, eta, theta, Denebola beta, delta, gamma, zeta, mu, epsilon, lambda, kappa, mu
            // delta, theta iota, sigma
            // epsilon, eta
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(4)),
                DrawTo(PtIndex(5)),
                DrawTo(PtIndex(6)),
                DrawTo(PtIndex(7)),
                /* */
                MoveTo(PtIndex(8)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(9)),
                /* */
                MoveTo(PtIndex(10)),
                DrawTo(PtIndex(5)),
                DrawTo(PtIndex(11)),
                /* */
                MoveTo(PtIndex(2)),
                DrawTo(PtIndex(12)),
                DrawTo(PtIndex(13)),
                /* */
                MoveTo(PtIndex(6)),
                DrawTo(PtIndex(14)),
                DrawTo(PtIndex(15)),
            ],
        }],
    },
    Constellation {
        name: "Bootes",
        points: &[71795, 69673, 74666, 73555, 71075, 71053, 67927, 67459],
        drawings: &[Drawing {
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(4)),
                DrawTo(PtIndex(5)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(6)),
                DrawTo(PtIndex(7)),
            ],
        }],
    },
    Constellation {
        name: "Corona Borealis",
        points: &[78493, 78159, 77512, 76267, 75695, 76127],
        drawings: &[Drawing {
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(4)),
                DrawTo(PtIndex(5)),
            ],
        }],
    },
    Constellation {
        name: "Cygnus",
        points: &[
            102098, 100453, 98110, 95947, 107310, 104732, 102488, 97165, 95853, 94779,
        ],
        drawings: &[Drawing {
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(3)),
                /* */
                MoveTo(PtIndex(4)),
                DrawTo(PtIndex(5)),
                DrawTo(PtIndex(6)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(7)),
                DrawTo(PtIndex(8)),
                DrawTo(PtIndex(9)),
            ],
        }],
    },
    Constellation {
        name: "Draco",
        points: &[
            56211, 61281, 68756, 75458, 78527, 80331, 83895, 89937, 94648, 97433, 94376, 87585,
            87833, 85670, 85819,
        ],
        drawings: &[Drawing {
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(4)),
                DrawTo(PtIndex(5)),
                DrawTo(PtIndex(6)),
                DrawTo(PtIndex(7)),
                DrawTo(PtIndex(8)),
                DrawTo(PtIndex(9)),
                DrawTo(PtIndex(10)),
                DrawTo(PtIndex(11)),
                DrawTo(PtIndex(12)),
                DrawTo(PtIndex(13)),
                DrawTo(PtIndex(14)),
                DrawTo(PtIndex(11)),
            ],
        }],
    },
    Constellation {
        name: "Cepheus",
        points: &[106032, 112724, 116727, 105199, 109492],
        drawings: &[Drawing {
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(0)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(4)),
                DrawTo(PtIndex(1)),
            ],
        }],
    },
    Constellation {
        name: "Lacerta",
        points: &[109937, 111104, 111022, 111169, 110538, 110609],
        drawings: &[Drawing {
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(4)),
                DrawTo(PtIndex(5)),
                DrawTo(PtIndex(2)),
            ],
        }],
    },
    Constellation {
        name: "Auriga",
        points: &[24608, 23453, 23015, 25428, 28380, 28360],
        drawings: &[Drawing {
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(4)),
                DrawTo(PtIndex(5)),
                DrawTo(PtIndex(0)),
            ],
        }],
    },
    Constellation {
        name: "Taurus",
        points: &[
            15900, 18724, 20205, 20455, 20889, 21881, 25428, //
            26451, 21421, 20894, 17847,
        ],
        drawings: &[Drawing {
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(4)),
                DrawTo(PtIndex(5)),
                DrawTo(PtIndex(6)),
                /* */
                MoveTo(PtIndex(7)),
                DrawTo(PtIndex(8)),
                DrawTo(PtIndex(9)),
                DrawTo(PtIndex(2)),
                /* */
                MoveTo(PtIndex(8)),
                DrawTo(PtIndex(4)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(10)),
            ],
        }],
    },
    Constellation {
        name: "Camelopardalis",
        points: &[25110, 17959, 16228, 18505, 22783],
        drawings: &[Drawing {
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(4)),
                DrawTo(PtIndex(1)),
            ],
        }],
    },
    Constellation {
        name: "Triangulum",
        points: &[10064, 8796, 10670],
        drawings: &[Drawing {
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(0)),
            ],
        }],
    },
    Constellation {
        name: "Aries",
        points: &[13209, 9884, 8903, 8832],
        drawings: &[Drawing {
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(3)),
            ],
        }],
    },
    Constellation {
        name: "Perseus",
        points: &[
            13268, 14328, 15863, 17358, 18532, 18614, 18246, 17448, 13254, 14354, 14576,
        ],
        drawings: &[Drawing {
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(4)),
                DrawTo(PtIndex(5)),
                DrawTo(PtIndex(6)),
                DrawTo(PtIndex(7)),
                /* */
                MoveTo(PtIndex(8)),
                DrawTo(PtIndex(9)),
                DrawTo(PtIndex(10)),
                DrawTo(PtIndex(2)),
            ],
        }],
    },
    Constellation {
        name: "Equuleus",
        points: &[104987, 105570, 104858, 104521],
        drawings: &[Drawing {
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(0)),
            ],
        }],
    },
    Constellation {
        name: "Sagitta",
        points: &[98920, 98337, 97365, 96837, 96757],
        drawings: &[Drawing {
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(3)),
                /* */
                MoveTo(PtIndex(2)),
                DrawTo(PtIndex(4)),
            ],
        }],
    },
    Constellation {
        name: "Delphinus",
        points: &[101769, 102281, 102532, 101958, 101421],
        drawings: &[Drawing {
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(0)),
                DrawTo(PtIndex(4)),
            ],
        }],
    },
    Constellation {
        name: "Capricornus",
        points: &[
            107556, 106985, 105515, 104139, 100345, 100027, 105881, 102978, 102485,
        ],
        drawings: &[Drawing {
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(4)),
                DrawTo(PtIndex(5)),
                /* */
                MoveTo(PtIndex(2)),
                DrawTo(PtIndex(6)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(7)),
                /* */
                MoveTo(PtIndex(4)),
                DrawTo(PtIndex(8)),
            ],
        }],
    },
    Constellation {
        name: "Cancer",
        points: &[43105, 42806, 42911, 44066, 40843, 40526],
        drawings: &[Drawing {
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(3)),
                /* */
                MoveTo(PtIndex(4)),
                DrawTo(PtIndex(1)),
                /* */
                MoveTo(PtIndex(5)),
                DrawTo(PtIndex(2)),
            ],
        }],
    },
    Constellation {
        name: "Pisces",
        points: &[
            5742, 6193, 4889, 7097, 8198, 9487, 8833, 7884, 7007, 4906, 1645, 118268, 116771,
            115830, 114971, 115738, 116928, 116771,
        ],
        drawings: &[Drawing {
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(0)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(4)),
                DrawTo(PtIndex(5)),
                DrawTo(PtIndex(6)),
                DrawTo(PtIndex(7)),
                DrawTo(PtIndex(8)),
                DrawTo(PtIndex(9)),
                DrawTo(PtIndex(10)),
                DrawTo(PtIndex(11)),
                DrawTo(PtIndex(12)),
                DrawTo(PtIndex(13)),
                DrawTo(PtIndex(14)),
                DrawTo(PtIndex(15)),
                DrawTo(PtIndex(16)),
                DrawTo(PtIndex(12)),
            ],
        }],
    },
    Constellation {
        name: "Pegasus",
        points: &[
            107315, 109427, 112029, 113963, 1067, 677, 3092, 5447, 9640, 3881, 4436, 109410,
            112158, 113881, 107354, 109176, 112440, 112748,
        ],
        drawings: &[Drawing {
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(4)),
                DrawTo(PtIndex(5)),
                DrawTo(PtIndex(6)),
                DrawTo(PtIndex(7)),
                DrawTo(PtIndex(8)),
                /* */
                MoveTo(PtIndex(9)),
                DrawTo(PtIndex(10)),
                DrawTo(PtIndex(7)),
                /* */
                MoveTo(PtIndex(11)),
                DrawTo(PtIndex(12)),
                DrawTo(PtIndex(13)),
                DrawTo(PtIndex(5)),
                /* */
                MoveTo(PtIndex(14)),
                DrawTo(PtIndex(15)),
                DrawTo(PtIndex(16)),
                DrawTo(PtIndex(17)),
                DrawTo(PtIndex(13)),
                DrawTo(PtIndex(3)),
            ],
        }],
    },
    Constellation {
        name: "Aquarius",
        points: &[
            115438, 114855, 112961, 111497, 110960, 110395, 109074, 106278, 102618, 114341, 113136,
            112716, 111123, 110003, 109139, 109472, 110003,
        ],
        drawings: &[Drawing {
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(4)),
                DrawTo(PtIndex(5)),
                DrawTo(PtIndex(6)),
                DrawTo(PtIndex(7)),
                DrawTo(PtIndex(8)),
                /* */
                MoveTo(PtIndex(9)),
                DrawTo(PtIndex(10)),
                DrawTo(PtIndex(11)),
                DrawTo(PtIndex(12)),
                DrawTo(PtIndex(13)),
                DrawTo(PtIndex(6)),
                /* */
                MoveTo(PtIndex(14)),
                DrawTo(PtIndex(15)),
                DrawTo(PtIndex(13)),
            ],
        }],
    },
    Constellation {
        name: "Hercules",
        // kappa 79043/5
        // -95 88267
        // alpha 84345
        // -30 80704
        // -68 84573
        // nu 87998
        // omega 80463
        // eta 81833
        // xi 87933
        // epsilon 83207
        points: &[
            88794, 87933, 86974, 85693, 83207, 84380, 85112, 87808, 86414, 84379, 80170,
            80816, // omicron xi mu lambda epsilon pi rho theta iota delta gamma beta
            81693, 81833, 81126, 79992, 79101, 77760, // zeta eta sigma tau phi _ zeta
        ],
        drawings: &[Drawing {
            instructions: &[
                MoveTo(PtIndex(0)),
                DrawTo(PtIndex(1)),
                DrawTo(PtIndex(2)),
                DrawTo(PtIndex(3)),
                DrawTo(PtIndex(4)),
                DrawTo(PtIndex(5)),
                DrawTo(PtIndex(6)),
                DrawTo(PtIndex(7)),
                DrawTo(PtIndex(8)),
                /* */
                MoveTo(PtIndex(9)),
                DrawTo(PtIndex(3)),
                /* */
                MoveTo(PtIndex(10)),
                DrawTo(PtIndex(11)),
                DrawTo(PtIndex(12)),
                DrawTo(PtIndex(13)),
                DrawTo(PtIndex(14)),
                DrawTo(PtIndex(15)),
                DrawTo(PtIndex(16)),
                DrawTo(PtIndex(17)),
                /* */
                MoveTo(PtIndex(5)),
                DrawTo(PtIndex(13)),
                /* */
                MoveTo(PtIndex(4)),
                DrawTo(PtIndex(12)),
            ],
        }],
    },
];
