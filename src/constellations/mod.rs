//! Constants that represent some common constellations, using Hipparcos numbers for the stars

/// Constellations in the norhern hemisphere (Hipparcos numbers)
///
/// 0 implies a break in a drawing; the first loop is the most important
pub const NORTHERN_HEMISPHERE: &[(&str, &[usize])] = &[
    (
        //
        "Orion",
        &[
            27366, 26727, 27989, 26207, 25336, 25930, 26311, 24436, 0, 26727, 26311, 25930, 0,
            27989, 25336,
        ],
    ),
    (
        //
        //
        //
        // Phecda -> xi -> Alula Borealis (nu) -> ?Alula Australis (eta?)
        // xi -> upsilon -> Tania Australis (mu) -> Tania Borealis (lambda)
        "Ursa Major",
        &[
            // Megrez -> Dubhe -> Merak -> Phecda -> Megrez ->  Alioth -> Mizar -> Alkaid
            59774, 54061, 53910, 58001, 59774, 62956, 65378, 67301, 0,
            // Dubhe -> ? Muscida -> not-nu -> theta -> Talitha; Merak -> not-nu
            // front leg
            54061, 46733, 41704, 48319, 53910, 0, //
            48319, 46853, 44471, 0, //
            58001, 54539, 50801, 0, //
            54539, 50372, 0, //
        ],
    ),
    (
        // Caph, Schedar, Tsih, Ruchbah, Segin
        "Cassiopeia",
        &[746, 3179, 4427, 6686, 8886],
    ),
    (
        // alpha, delta, epsilon, zeta, beta, gamma, eta
        "Ursa Minor",
        &[11767, 85822, 82080, 77055, 72607, 75097, 79822],
    ),
    (
        // Regulus alpha, eta, theta, Denebola beta, delta, gamma, zeta, mu, epsilon, lambda, kappa, mu
        // delta, theta iota, sigma
        // epsilon, eta
        "Leo",
        &[
        49669, 54879, 57632, 54872, 50583, 49583, 49669, 0,  //
        50583, 50335, 48455, 47908, 0, 
            54872, 54879, 0, //
        ],
    ),
    (
        "Gemini",
        &[32362, 35350, 35550, 36962, 36046, 34693, 32246, 30883, 0, //
        37740, 36962, 37826, 0, //
        36850, 34693, 33018, 0, //
        35550, 34088, 31681, 0, //
        32246, 30343, 28734, // Note 28437 is not valid in the HIP catalog as we have it
        ],
    ),
    (
        "Bootes",
        &[71795, 69673, 74666, 73555, 71075, 71053, 69673, 67927, 67459,
        ],
    ),
    (
        "Corona Borealis",
        &[78493, 78159, 77512, 76267, 75695, 76127,
        ],
    ),
    (
        "Lyra",
        &[91262, 91971, 92791, 93194, 92420, 91971,
        ],
    ),
    (
        "Cygnus",
        &[102098, 100453, 98110, 95947, 0, //
        107310, 104732, 102488, 100453, 97165, 95853, 94779,
        ],
    ),
    (
        "Draco",
        &[56211, 61281, 68756, 75458, 78527, 80331, 83895, 89937, 94648, 97433, 94376, 87585, 87833, 85670, 85819, 87585, 
        ],
    ),
    (
        "Cepheus",
        &[106032, 112724, 116727, 106032, 105199, 109492, 112724,
        ],
    ),
    (
        "Lacerta",
        &[109937, 111104, 111022, 111169, 110538, 110609, 111022
        ],
    ),
    (
        "Pegasus",
        &[107315, 109427, 112029, 113963, 1067, 677, 3092, 5447, 9640, 0,//
        3881, 4436, 5447, 0,//
        109410, 112158, 113881, 677, 0, //
        107354, 109176, 112440, 112748, 113881, 113963, 
        ],
    ),
    (
        "Auriga",
        &[24608, 23453, 23015, 25428, 28380, 28360, 24608, 
        ],
    ),
    (
        "Taurus",
        &[15900, 18724, 20205, 20455, 20889, 21881, 25428, 0, //
        26451, 21421, 20894, 20205, 0, //
        21421, 20889, 20455, 17847, 
        ],
    ),
    (
        "Camelopardalis",
        &[25110, 17959, 16228, 18505, 22783, 17959,
        ],
    ),
    (
        "Perseus",
        &[13268, 14328, 15863, 17358, 18532, 18614, 18246, 17448, 0,
        13254, 14354, 14576, 15863,
        ],
    ),
    (
        "Triangulum",
        &[10064, 8796, 10670, 10064,
        ],
    ),
    (
        "Aries",
        &[13209, 9884, 8903, 8832,
        ],
    ),
    (
        "Pisces",
        &[5742, 6193, 4889, 5742, 7097, 8198, 9487, 8833, 7884, 7007, 4906, 1645, 118268, 116771, 115830, 114971, 115738, 116928, 116771
        ],
    ),
    (
        "Equuleus",
        &[104987, 105570, 104858, 104521, 104987,
        ],
    ),
    (
        "Sagitta",
        &[98920, 98337, 97365, 96837, 0, 97365, 96757,
        ],
    ),
    (
        "Delphinus",
        &[101769, 102281, 102532, 101958, 101769, 101421,
        ],
    ),
    (
        "Aquarius",
        &[115438, 114855, 112961, 111497, 110960, 110395, 109074, 106278, 102618, 0,
        114341, 113136, 112716, 111123, 110003, 109074, 0,
        109139, 109472, 110003
        ],
    ),
    (
        "Hercules",
        &[88794, 87933, 86974, 85693, 83207, 84380, 851122, 87808, 86414, 0,// 
        84379, 85693, 0,// 
        80170, 80816, 150680, 81833, 81126, 79992, 79101, 77760, 0,// 
        84380, 81833, 0,//
        83207, 81693,
        ],
    ),
    (
        "Capricornus",
        &[107556, 106985, 105515, 104139, 100345, 100027, 0,
        105515, 105881, 104139, 102978, 0, //
        100345, 102485,
        ],
    ),
    (
        "Cancer",
        &[43105, 42806, 42911, 44066, 0,//
        40843, 42806, 0,  //
        40526, 42911, 0, 
        ],
    ),
];
