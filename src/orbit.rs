use num_traits::FloatConst;

use crate::{Quat, Vec3};
use geo_nd::Quaternion;

pub const fn unix_time(
    year: u32,
    month: u32,
    day: u32,
    hours: u32,
    minutes: u32,
    seconds: u32,
) -> u64 {
    const START_OF_MONTH: &[u32] = &[0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    const fn is_leap_year(year: u32) -> bool {
        year.is_multiple_of(1000) || (year.is_multiple_of(4) && !year.is_multiple_of(100))
    }
    const fn leap_years_since_0(year: u32) -> u32 {
        year / 4 - (year / 100) + (year / 1000)
    }
    const fn days_of_year_since_0(year: u32) -> u32 {
        year * 365 + leap_years_since_0(year)
    }
    // Note 1970 was not a leap year
    //
    // days_of_years is *1* more than we want for a leap year for Jan and Feb
    let days_of_years = days_of_year_since_0(year) - days_of_year_since_0(1970);
    let days_of_months = START_OF_MONTH[(month - 1) as usize];
    let mut days = days_of_years + days_of_months + (day - 1);
    if month <= 2 && is_leap_year(year) {
        days -= 1;
    }
    let time_of_day =
        ((((days as u64) * 24 + (hours as u64)) * 60) + (minutes as u64)) * 60 + (seconds as u64);
    time_of_day
}

/// An elliptical orbit is fully described by the plane in which it sits (a
/// normal, the line of apsides, and the perpendicular to these).
///
/// If the normal is different to the XY of the plane of reference, then the
/// angle between the two normals is the inclination.
///
/// If the normal is different to the XY of the plane of reference, then the
/// orbital plane interesects the XY plane of reference in a line; this line will be
/// a rotation of the parent X axis around Z by some angle, known as the
/// longitude of the ascending node.
///
/// The line of apsides is the line connecting the point around which the orbit,
/// the point of closest approach (or perigee) and the point of furthest
/// distance (apogee).
///
/// The argument of periapsis is the angle of rotation required around the Z
/// axis to move the periapsis from the X axis to its required point, after the
/// inclination and longitude of ascending node have been applied.
///
/// When the object is at true_anomaly of 0 it is at perigee; it is at (X,0,0) and has velocity (0,+,0).
///
#[derive(Debug, Clone)]
pub struct KeplerianElements {
    /// Inclination of the orbit - angle between parent 'Z' axis and normal to the orbital plane (one of the degrees of freedom of the plane)
    inclination: f64,
    /// Longitude of ascending node - After inclination, rotation around parent Z axis of the orbital plane
    longitude_of_ascending_node: f64,
    /// Argument of periapsis - After longitude, rotation around the *orbital* Z to the perihelion (0 indicating the line of apsides is the parent XY=0 plane)
    argument_of_periapsis: f64,
    /// Eccentricity of the orbit (0 being circular)
    eccentricity: f64,
    /// Semimajor axis (half of the distance between the points of apogee and perigee)
    semimajor_axis: f64,
    /// Period of the orbit in seconds
    period_of_orbit: f64,
    /// True anomaly at epoch (effectively the phase); the angle between the
    /// direction of periapsis and the body at the epoch as seen by the main
    /// focus of the ellipse.
    true_anomaly_at_epoch: f64,
    /// Epoch in seconds since Jan 1 1970
    epoch: u64,
}

impl KeplerianElements {
    fn orbit_to_parent(&self) -> Quat {
        Quat::default()
            .rotate_z(self.argument_of_periapsis)
            .rotate_y(self.inclination)
            .rotate_z(self.longitude_of_ascending_node)
    }
}

pub const MERCURY_SOLAR_J2000: KeplerianElements = KeplerianElements {
    inclination: (3.38_f64).to_radians(), // to Sun's equator
    longitude_of_ascending_node: (48.331_f64).to_radians(),
    argument_of_periapsis: (29.124_f64).to_radians(),
    eccentricity: 0.205630,
    semimajor_axis: 57.91E6,
    period_of_orbit: 87.9691 * (24.0 * 60.0 * 60.0),
    true_anomaly_at_epoch: 0.0, // Perihelion at ? Jan 9, 1883
    epoch: 946814400,
};

pub const VENUS_SOLAR_J2000: KeplerianElements = KeplerianElements {
    inclination: (3.86_f64).to_radians(), // to Sun's equator
    longitude_of_ascending_node: (76.680_f64).to_radians(),
    argument_of_periapsis: (54.884_f64).to_radians(),
    eccentricity: 0.006772,
    semimajor_axis: 108.21E6,
    period_of_orbit: 224.701 * (24.0 * 60.0 * 60.0),
    true_anomaly_at_epoch: 360.0 - 308.95, // Perihelion at 19 Feb 2025
    epoch: 946814400,
};

pub const EARTH_SOLAR_J2000: KeplerianElements = KeplerianElements {
    inclination: (7.155_f64).to_radians(),
    longitude_of_ascending_node: (174.9_f64).to_radians(),
    argument_of_periapsis: (288.1_f64).to_radians(),
    eccentricity: 0.0167086,
    semimajor_axis: 149.69E6,
    period_of_orbit: 365.256363004 * (24.0 * 60.0 * 60.0),
    true_anomaly_at_epoch: 360.0 - 1.98, // 0.0, //360.0 - 1.98,
    epoch: 946814400,
};

pub const MARS_SOLAR_J2000: KeplerianElements = KeplerianElements {
    inclination: (5.65_f64).to_radians(), // to Sun's equator
    longitude_of_ascending_node: (49.57854_f64).to_radians(),
    argument_of_periapsis: (286.5_f64).to_radians(),
    eccentricity: 0.0934,
    semimajor_axis: 227.939_366E6,
    period_of_orbit: 686.980 * (24.0 * 60.0 * 60.0),
    true_anomaly_at_epoch: 0.0, // Perihelion at 11 February 2028
    epoch: 946814400,
};

pub const JUPITER_SOLAR_J2000: KeplerianElements = KeplerianElements {
    inclination: (6.09_f64).to_radians(), // to Sun's equator
    longitude_of_ascending_node: (100.464_f64).to_radians(),
    argument_of_periapsis: (273.867_f64).to_radians(),
    eccentricity: 0.0489,
    semimajor_axis: 778.479E6,
    period_of_orbit: 4_332.59 * (24.0 * 60.0 * 60.0),
    true_anomaly_at_epoch: 0.0, // Perihelion at January 21, 2023
    epoch: 946814400,
};

pub const SATURN_SOLAR_J2000: KeplerianElements = KeplerianElements {
    inclination: (5.51_f64).to_radians(), // to Sun's equator
    longitude_of_ascending_node: (113.665_f64).to_radians(),
    argument_of_periapsis: (339.392_f64).to_radians(),
    eccentricity: 0.0565,
    semimajor_axis: 1_433.53E6,
    period_of_orbit: 10_755.70 * (24.0 * 60.0 * 60.0),
    true_anomaly_at_epoch: 0.0, // Perihelion at 2032-Nov-29
    epoch: 946814400,
};

pub const URANUS_SOLAR_J2000: KeplerianElements = KeplerianElements {
    inclination: (6.48_f64).to_radians(), // to Sun's equator
    longitude_of_ascending_node: (74.006_f64).to_radians(),
    argument_of_periapsis: (96.998_857_f64).to_radians(),
    eccentricity: 0.04717,
    semimajor_axis: 2.870_972E9,
    period_of_orbit: 30_688.5 * (24.0 * 60.0 * 60.0),
    true_anomaly_at_epoch: 0.0, // Perihelion at 17–19 August 2050
    epoch: 946814400,
};

pub const NEPTUNE_SOLAR_J2000: KeplerianElements = KeplerianElements {
    inclination: (6.43_f64).to_radians(), // to Sun's equator
    longitude_of_ascending_node: (131.783_f64).to_radians(),
    argument_of_periapsis: (273.187_f64).to_radians(),
    eccentricity: 0.008_678,
    semimajor_axis: 4.503E9,
    period_of_orbit: 60_195.0 * (24.0 * 60.0 * 60.0),
    true_anomaly_at_epoch: 0.0, // Perihelion at 2042-Sep-04
    epoch: 946814400,
};

pub const SOLAR_SYSTEM: [(&str, &KeplerianElements); 8] = [
    ("Mercury", &MERCURY_SOLAR_J2000),
    ("Venus", &VENUS_SOLAR_J2000),
    ("Earth", &EARTH_SOLAR_J2000),
    ("Mars", &MARS_SOLAR_J2000),
    ("Jupiter", &JUPITER_SOLAR_J2000),
    ("Saturn", &SATURN_SOLAR_J2000),
    ("Uranus", &URANUS_SOLAR_J2000),
    ("Nepture", &NEPTUNE_SOLAR_J2000),
];

/// This is a simple model of an elliptical orbit within a plane
///
/// The plane is given by a quaternion that maps the frame (major axis, minor
/// axis, normal) to the parent; for an orbit that is not specifically embedded
/// in a parent this can be the identity (or ignored)
///
/// The ellipse is specified by the length of the major axis (actually the semimajor_axis length) and the eccentricity.
///
/// The time-related aspects are the period of the orbit, and then the phase at
/// the epoch or the time relative to the epoch of the object being at the
/// perigee of the orbit
#[derive(Debug, Clone)]
pub struct Orbit {
    /// Rotation to be applied to an orbital XYZ to place it in the parent
    orbit_to_parent: Quat,
    /// Eccentricity of the orbit (0 being circular)
    eccentricity: f64,
    /// Semimajor axis (half of the distance between the points of apogee and perigee)
    semimajor_axis: f64,
    /// True anomaly at epoch (effectively the phase); the angle between the
    /// direction of periapsis and the body at the epoch as seen by the main
    /// focus of the ellipse.
    true_anomaly_at_epoch: f64,
    /// Time of perige - i.e. when true_anomaly was 0 - relative to the epoch - another way to represent the phase
    time_of_perigee: f64,
    /// Period of orbit
    period_of_orbit: f64,
    /// Epoch in seconds since Jan 1 1970
    epoch: u64,
}

fn newton_raphson<F: Fn(f64) -> f64>(init: f64, f_m_xdf_div_df: F) -> f64 {
    let mut guess = init;
    let mut last_guess;
    for _ in 0..10 {
        last_guess = guess;
        guess = f_m_xdf_div_df(guess);
        if (last_guess - guess).abs() < 1E-12 {
            return guess;
        }
    }
    // todo!();
    guess
}

impl std::convert::From<&KeplerianElements> for Orbit {
    fn from(kepler: &KeplerianElements) -> Self {
        let orbit_to_parent = kepler.orbit_to_parent();
        let mut s = Self {
            orbit_to_parent,
            eccentricity: kepler.eccentricity,
            semimajor_axis: kepler.semimajor_axis,
            true_anomaly_at_epoch: kepler.true_anomaly_at_epoch / 180.0 * f64::PI(),
            period_of_orbit: kepler.period_of_orbit,
            time_of_perigee: 0.0,
            epoch: kepler.epoch,
        };
        s.time_of_perigee = -s.relative_time_of_true_anomaly(s.true_anomaly_at_epoch);
        s
    }
}

impl Orbit {
    ///
    /// Calculate the mean_anomaly for time t since the epoch
    ///
    fn true_anomaly_of_relative_time(&self, t: f64) -> f64 {
        let mut time_relative = (t / self.period_of_orbit).fract();
        if time_relative < 0.0 {
            time_relative += 1.0;
        }
        // Calculate the mean anomaly M, which is the angle around a *circle* that the object in orbit would have progressed to if the orbit were circular
        let mean_anomaly = time_relative * f64::TAU();

        // The eccentric_anomaly E is given by M = E - e.sin(E); this cannot be done analytically...
        //
        // Consider f(E) = E - e.sin(E) - M; f'(E) = 1 - e.cos(E); then f(E) = 0 can be evaluated with Newton-Raphson
        //
        // E.n+1 = E.n - (E.n - e.sin(E.n) - M)/(1 - e.cos(E.n))
        //
        // E.n+1 = (E.n - E.n.e.cos(E.n) - E.n + e.sin(E.n) + M ) /(1 - e.cos(E.n))
        //
        // E.n+1 = (M + e.sin(E.n) - E.n . e . cos(E.n)) /(1 - e.cos(E.n))
        let e_guess_x = (self.eccentricity * mean_anomaly.sin())
            / (1.0 - self.eccentricity * mean_anomaly.cos());
        let init = mean_anomaly + e_guess_x * (1.0 - 0.5 * e_guess_x * e_guess_x);

        let eccentric_anomaly = newton_raphson(init, |e_a: f64| {
            let e_times_s = self.eccentricity * e_a.sin();
            let e_times_c = self.eccentricity * e_a.cos();
            (mean_anomaly + e_times_s - e_a * e_times_c) / (1.0 - e_times_c)
        });

        if true {
            let test_mean_anomaly = eccentric_anomaly - self.eccentricity * eccentric_anomaly.sin();
            assert!(
                (test_mean_anomaly - mean_anomaly).abs() < 1E-4,
                "{test_mean_anomaly}, {mean_anomaly}, {eccentric_anomaly}"
            );
        }
        self.true_of_eccentric_anomaly(eccentric_anomaly, time_relative > 0.5)
    }

    fn true_of_eccentric_anomaly(&self, eccentric_anomaly: f64, is_second_half: bool) -> f64 {
        let cos_e_a = eccentric_anomaly.cos();
        let cos_t_a = (cos_e_a - self.eccentricity) / (1.0 - self.eccentricity * cos_e_a);
        // Get true_anomaly in 0..PI - which half of the orbit is a guess at present!
        let mut true_anomaly = cos_t_a.acos();
        if is_second_half {
            true_anomaly = f64::TAU() - true_anomaly;
        }
        true_anomaly
    }

    fn eccentric_of_true_anomaly(&self, true_anomaly: f64) -> f64 {
        let relative_eccentricty = (1.0 - self.eccentricity * self.eccentricity).sqrt();
        let sin_t_a = true_anomaly.sin();
        let cos_t_a = true_anomaly.cos();
        let eccentric_anomaly = (relative_eccentricty * sin_t_a).atan2(self.eccentricity + cos_t_a);
        eccentric_anomaly
    }

    /// Calculate the time in seconds relative to the start of the epoch of a particular true_anomaly
    ///
    /// Uses the eccentricity and period_of_orbit *ONLY*
    ///
    /// From wikipedia
    ///
    /// https://en.wikipedia.org/wiki/Mean_anomaly
    fn relative_time_of_true_anomaly(&self, true_anomaly: f64) -> f64 {
        let eccentric_anomaly = self.eccentric_of_true_anomaly(true_anomaly);
        let mean_anomaly = eccentric_anomaly - self.eccentricity * eccentric_anomaly.sin();
        mean_anomaly / f64::TAU() * self.period_of_orbit
    }

    fn distance_of_true_anomaly(&self, true_anomaly: f64) -> f64 {
        let relative_eccentricty = 1.0 - self.eccentricity * self.eccentricity;
        self.semimajor_axis * relative_eccentricty / (1.0 + self.eccentricity * true_anomaly.cos())
    }

    pub fn perigee_distance(&self) -> f64 {
        self.semimajor_axis * (1.0 - self.eccentricity)
    }

    pub fn apogee_distance(&self) -> f64 {
        self.semimajor_axis * (1.0 + self.eccentricity)
    }

    fn relative_time_of_unix_time(&self, time_secs: u64) -> f64 {
        let secs_since_epoch = (time_secs - self.epoch) as f64;
        let secs_since_perigee = secs_since_epoch - self.time_of_perigee;
        secs_since_perigee
    }

    /// Calculate the (x,y)-in-plane coordinates of the object in orbit with
    /// respect to its parent for a time in seconds relative to the UNIX epoch
    pub fn vec_of_unix_time(&self, time_secs: u64) -> [f64; 2] {
        let secs_since_perigee = self.relative_time_of_unix_time(time_secs);
        let true_anomaly = self.true_anomaly_of_relative_time(secs_since_perigee);
        let distance = self.distance_of_true_anomaly(true_anomaly);
        let c_ta = true_anomaly.cos();
        let s_ta = true_anomaly.sin();
        [distance * c_ta, distance * s_ta]
    }
}

// Test true anomaly to / from eccentric and to/from relative time
#[test]
fn test_ta() {
    let earth: Orbit = (&EARTH_SOLAR_J2000).into();
    for i in 0..100 {
        let true_anomaly = (i as f64) * f64::TAU() / 100.0;
        let e = earth.eccentric_of_true_anomaly(true_anomaly);
        let true_anomaly_3 = earth.true_of_eccentric_anomaly(e, i >= 50);

        assert!(
            (true_anomaly_3 - true_anomaly).abs() < 1E-9,
            "{i} {true_anomaly} {true_anomaly_3} {e}",
        );
    }
    for i in 0..100 {
        let true_anomaly = (i as f64) * f64::TAU() / 100.0;
        let t = earth.relative_time_of_true_anomaly(true_anomaly);
        let d = earth.distance_of_true_anomaly(true_anomaly);
        let true_anomaly_2 = earth.true_anomaly_of_relative_time(t);
        let mut dt = (true_anomaly - true_anomaly_2) / f64::TAU();
        if dt > 0.5 {
            dt = 1.0 - dt;
        }
        eprintln!(
            "{i} {true_anomaly} {true_anomaly_2} {} {d:e}",
            t / earth.period_of_orbit,
        );
        assert!(
            dt.abs() < 1E-9,
            "{i} {true_anomaly} {true_anomaly_2} {} {d:e}",
            t / earth.period_of_orbit,
        );
    }
}

// Test solar system orbita / perihelion dates
#[test]
fn test_solar_system() {
    const TEST_DATA: &[(&str, &KeplerianElements, f64, f64, &[u64])] = &[
        (
            "earth",
            &EARTH_SOLAR_J2000,
            147_098_291.0_f64,
            152_098_233.0_f64,
            &[
                unix_time(2026, 1, 3, 02, 32, 0),
                unix_time(2010, 1, 3, 00, 09, 0),
                unix_time(2012, 1, 5, 00, 32, 0),
                unix_time(2035, 1, 3, 00, 54, 0),
            ],
        ),
        (
            "venus",
            &VENUS_SOLAR_J2000,
            107_476_170.0,
            108_942_780.0,
            &[
                unix_time(2025, 2, 19, 18, 00, 0),
                unix_time(2025, 10, 1, 18, 00, 0),
            ],
        ),
    ];
    for (name, planet, perihelion, aphelion, perihelion_times) in TEST_DATA {
        let orbit: Orbit = (*planet).into();
        eprintln!("{orbit:?} {perihelion_times:?}");
        eprintln!("Planet: {name}");
        eprintln!("   Perihelion: {:e}", orbit.perigee_distance());
        eprintln!("   Aphelion: {:e}", orbit.apogee_distance());
        eprintln!("   Solar orbit period: {:.2}", orbit.period_of_orbit);
        eprintln!(
            "   True anomaly of epoch: {:.2} {} {}",
            orbit.true_anomaly_of_relative_time(orbit.relative_time_of_unix_time(orbit.epoch)),
            orbit.relative_time_of_unix_time(orbit.epoch),
            orbit.epoch
        );
        eprintln!(
            "   True anomaly of first perihelion time: {:.2} {} {}",
            orbit.true_anomaly_of_relative_time(
                orbit.relative_time_of_unix_time(perihelion_times[0])
            ) * 180.0
                / f64::PI(),
            orbit.relative_time_of_unix_time(perihelion_times[0]),
            perihelion_times[0]
        );

        assert!(
            ((orbit.perigee_distance() / *perihelion) - 1.0).abs() < 0.001,
            "Perihelion should be correct to 1 part in 1000"
        );
        assert!(
            ((orbit.apogee_distance() / *aphelion) - 1.0).abs() < 0.001,
            "Aphelion should be corect to 1 part in 1000"
        );
        for p in *perihelion_times {
            let ta = orbit.true_anomaly_of_relative_time(orbit.relative_time_of_unix_time(*p));

            assert!(
                ((ta / f64::TAU() - 0.5).abs().fract() - 0.5).abs() < 1E-2,
                "True anmoaly at perihelion time {p} should be approx 0, got {ta}",
            );
        }
    }
}

// Test true anomaly to / from eccentric and to/from relative time
#[test]
fn test_known_good_values() {
    let earth: Orbit = (&EARTH_SOLAR_J2000).into();
    const TEST_DATA: &[(u64, u32)] = &[
        (unix_time(2026, 1, 3, 02, 32, 00), 147_098),
        (unix_time(2026, 3, 12, 15, 30, 00), 148_643),
        (unix_time(2026, 8, 22, 15, 30, 00), 151_314),
        (unix_time(2026, 6, 28, 15, 30, 00), 152_080),
        (unix_time(2026, 10, 11, 15, 30, 00), 149_351),
        (unix_time(2099, 10, 11, 15, 30, 00), 149_351),
    ];

    for (t1, d) in TEST_DATA {
        let ta =
            earth.true_anomaly_of_relative_time((t1 - earth.epoch) as f64 - earth.time_of_perigee);
        let earth_d = earth.distance_of_true_anomaly(ta);
        let closeness = ((*d) as f64) * 1000.0 / earth_d;
        eprintln!("{earth_d} {d} {closeness}",);
        assert!(closeness > 0.999 && closeness < 1.001, "{closeness}");
    }
    // assert!(false);
}

/// A type that describes the orientation of an object that is rotating around an axis that is itself rotating (such as the Earth and its precession of equinoxes)
///
/// This is not a full celestial mechanical model; its accuracy is bounded as
/// this does not attempt to model the gravitational impacts of all the bodies
/// on the solar system on each other. However, it is the commonly used simple
/// approximation that holds up for human timescales.
pub struct PrecessionalRotation {
    /// Period of precession in seconds
    precession_period: f64,
    /// Axis of precession; this is in the terms of the orbital frame of reference
    precession_axis: Vec3,
    /// Axis of rotation at time 't=0' in the orbital frame of reference
    axis: Vec3,
    /// Period of rotation around 'axis' in seconds
    period: f64,
    /// Phase of rotation around 'axis' at time 0
    phase: f64,
}
impl PrecessionalRotation {
    /// The quaternion to be applied to the axis due to precession, at time t
    fn precessional_rotation_at_time(&self, t: f64) -> Quat {
        Quat::of_axis_angle(&self.precession_axis, t / self.precession_period)
    }
    /// The quaternion describing the orbital-frame to object-fixed rotation for a time t
    fn orbital_to_object_fixed(&self, t: f64) -> Quat {
        let precessional_rotation = self.precessional_rotation_at_time(t);
        let axis = precessional_rotation.apply3(&self.axis);
        Quat::of_axis_angle(&axis, self.phase + t / self.period)
    }
}
