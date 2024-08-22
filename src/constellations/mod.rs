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
        // Megrez -> Dubhe -> Merak -> Phecda -> Megrez ->  Alioth -> Mizar -> Alkaid
        //
        // Dubhe -> ? Muscida -> not-nu -> theta -> Talitha; Merak -> not-nu
        //
        // Phecda -> xi -> Alula Borealis (nu) -> ?Alula Australis (eta?)
        // xi -> upsilon -> Tania Australis (mu) -> Tania Borealis (lambda)
        "Ursa Major",
        &[
            41704, 54061, 53910, 58001, 59774, 62956, 65378, 67301, 0, 54061, 41704, 55219, 46853,
            44127, 0, 53910, 0, 58001, 57399, 55219, 55203, 0, 57399, 48319, 50801, 50372,
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
            49669, 49583, 54879, 57632, 54872, 50583, 50335, 48455, 47908, 46705, 46146, 48455, 0,
            54872, 54879, 55642, 55434, 0, 47908, 50335,
        ],
    ),
];
