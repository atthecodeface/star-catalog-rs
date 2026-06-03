/// Constellations in the norhern hemisphere (Hipparcos ids)
use crate::constellations::DrawingInstruction::*;
use crate::constellations::{Constellation, Drawing, PtIndex};
pub const NORTHERN_HEMISPHERE: &[Constellation] = &[Constellation {
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
            /* */
            LookAt(PtIndex(0), PtIndex(1), 100),
            SetXY(0, 10, 10),
            DrawXY(2, 50, 10),
            DrawXY(2, 50, 50),
            DrawXY(2, 10, 50),
            DrawXY(2, 10, 10),
        ],
    }],
}];
pub const NOT_NORTHERN_HEMISPHERE: &[Constellation] = &[
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
