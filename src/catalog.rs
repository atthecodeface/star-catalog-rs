//a Imports
use std::collections::HashMap;
use std::path::Path;

use geo_nd::{Quaternion, Vector};
use serde::{Deserialize, Serialize};

// hipparcos is used only with some features
#[allow(unused_imports)]
use crate::hipparcos;
use crate::{
    Error, Star, StarFilter, StarFilterFn, StarMatchMapping, StarMatchMappingSet, StarTriangle,
    StarTriangleMatch, StarTriangleSearch, Subcube, Vec3,
};

//a CatalogIndex
//tp CatalogIndex
// The sky above a location with latitude L and longitude M (east)
//
// It is right ascension X + M * 24/360 + days * 360/364.25
//
// days since 18th sept 2024 has X about 0

//tp CatalogIndex
/// An index into the Catalog to identify a particular star
///
/// A [CatalogIndex] becomes invalid if the Catalog is sorted again
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct CatalogIndex(usize);
impl CatalogIndex {
    pub fn as_usize(self) -> usize {
        self.0
    }
}

//ip From<usize> for CatalogIndex
impl From<usize> for CatalogIndex {
    fn from(index: usize) -> Self {
        CatalogIndex(index)
    }
}

//a Catalog
//tp Catalog
/// A catalog of stars
///
/// The catalog contains an indexed (and possibly named) list of
/// stars, which can be searched by id, name, or geometrically
///
/// # Design rationale for `Vec<Subcube>`
///
/// There are roughly 4,000 (3769 in fact) subcubes used out of 32^3 (i.e. 32,768)
/// by a star catalog with an ELE_PER_SIDE of 32
///
/// Purely using a vec of 32^3 vecs is 1MB of storage, plus subcube contents
///
/// A hash map from subcube to vec with 4k entries used is probably
/// 64kB plus 128kB for the vecs themselves; access is slower though.
#[derive(Default, Serialize, Deserialize)]
pub struct Catalog {
    /// Stars in the catalog
    ///
    /// When searching this must be sorted by id; if not sorted by id
    /// then searching will return errors
    stars: Vec<Star>,
    sorted: bool,
    /// Stars dictionary to map name to an index in stars
    named_stars: HashMap<String, CatalogIndex>,
    /// Filter to apply to finding stars
    #[serde(skip)]
    filter: StarFilter,
    /// Star indices within each subcube; filled out by derive_data
    ///
    /// Subcubes are numbered 0 to Subcube::NUM_SUBCUBES-1
    #[serde(skip)]
    subcubes: Vec<Vec<CatalogIndex>>,
}

//ip Catalog - Constructors and builders (e.g. add stars, names)
impl Catalog {
    //cp load_catalog
    pub fn load_catalog<P: AsRef<Path>>(
        catalog_filename: P,
        magnitude: f32,
    ) -> Result<Self, Error> {
        match catalog_filename
            .as_ref()
            .extension()
            .and_then(|x| x.to_str())
        {
            Some("json") => {
                let s = std::fs::read_to_string(catalog_filename.as_ref()).map_err(|e| {
                    (
                        e,
                        format!(
                            "failed to read json catalog file {:?}",
                            catalog_filename.as_ref()
                        ),
                    )
                })?;
                let mut catalog: Self = serde_json::from_str(&s)?;
                catalog.retain(move |s, _n| s.brighter_than(magnitude));
                catalog.sort();
                Ok(catalog)
            }
            #[cfg(feature = "postcard")]
            Some("pst") => {
                let data = std::fs::read(catalog_filename.as_ref()).map_err(|e| {
                    (
                        e,
                        format!(
                            "failed to read postcard catalog file {:?}",
                            catalog_filename.as_ref()
                        ),
                    )
                })?;
                let mut catalog: Self = postcard::from_bytes(&data)?;
                catalog.retain(move |s, _n| s.brighter_than(magnitude));
                catalog.sort();
                Ok(catalog)
            }
            #[cfg(feature = "csv")]
            Some("csv") => {
                let mut catalog = Self::default();
                let _ = catalog;
                {
                    let f = std::fs::File::open(catalog_filename.as_ref()).map_err(|e| {
                        (
                            e,
                            format!(
                                "failed to read postcard catalog file {:?}",
                                catalog_filename.as_ref()
                            ),
                        )
                    })?;
                    hipparcos::read_to_catalog(&mut catalog, &f, magnitude)?;
                }
                catalog.sort();
                Ok(catalog)
            }
            None => {
                let mut catalog = Self::default();
                #[cfg(feature = "hipp_bright")]
                if catalog_filename.as_ref().as_os_str().as_encoded_bytes() == b"hipp_bright" {
                    catalog = postcard::from_bytes(hipparcos::HIPP_BRIGHT_PST)?;
                    catalog.retain(move |s, _n| s.brighter_than(magnitude));
                }
                if catalog.is_empty() {
                    return Err(Error::UnknownCatalog);
                }
                catalog.sort();
                Ok(catalog)
            }
            _ => Err(Error::UnknownCatalogExtension),
        }
    }

    //mp add_star
    /// Add a star to the catalog
    ///
    /// This also clears any derived data and marks the catalog as
    /// usorted
    pub fn add_star(&mut self, star: Star) {
        self.clear_derived_data();
        self.sorted = false;
        self.stars.push(star);
    }

    //mp add_name
    /// Add a name for a single star in the catalog; the star must
    /// have been found by ID
    ///
    /// The catalog must have been sorted beforehand
    pub fn add_name<I: Into<String>>(&mut self, index: CatalogIndex, name: I) {
        self.named_stars.insert(name.into(), index);
    }

    //mp add_names
    /// Add names for a set of stars in the catalog, from their IDs
    ///
    /// The catalog must have been sorted beforehand
    pub fn add_names<I: Into<String> + Clone>(
        &mut self,
        id_names: &[(usize, I)],
        ignore_not_found: bool,
    ) -> Result<(), Error> {
        for (id, name) in id_names {
            let Some(index) = self.find_sorted(*id) else {
                if ignore_not_found {
                    continue;
                }
                return Err(Error::FailedToFindId(*id));
            };
            self.named_stars.insert(name.clone().into(), index);
        }
        Ok(())
    }

    //mp retain
    /// Retain stars that match a certain criterion; the rest are
    /// dropped
    ///
    /// This should be invoked prior to any stars being named; it also
    /// clears the derived data (e.g. geometric searching will not be
    /// allowed until a derive_data() call is invoked)
    pub fn retain<F>(&mut self, f: F)
    where
        F: StarFilterFn,
    {
        self.sorted = false;
        self.clear_derived_data();
        let mut i = 0;
        self.stars.retain(move |s| {
            i += 1;
            f(s, i)
        });
    }
}

//ip Catalog - Accessors
impl Catalog {
    //mp len
    /// Get the number of stars in the catalog
    pub fn len(&self) -> usize {
        self.stars.len()
    }

    //mp is_empty
    /// Returns true if the catalog contains no stars
    pub fn is_empty(&self) -> bool {
        self.stars.is_empty()
    }

    //mp is_sorted
    /// Returns true if the catalog has been sorted (and is thus ready
    /// for names to be added)
    pub fn is_sorted(&self) -> bool {
        self.sorted
    }

    //mi has_derived_data
    /// return true iif the data has been derived
    fn has_derived_data(&self) -> bool {
        !self.subcubes.is_empty()
    }
}

//ip Catalog - filtering
impl Catalog {
    //mp clear_filter
    pub fn clear_filter(&mut self) -> StarFilter {
        std::mem::take(&mut self.filter)
    }

    //ap filter
    pub fn filter(&self) -> &StarFilter {
        &self.filter
    }

    //mp set_filter
    pub fn set_filter(&mut self, f: StarFilter) {
        self.filter = f;
    }

    //mp add_filter
    pub fn add_filter(&mut self, f: StarFilter) {
        self.filter.then(f);
    }
}

//ip Catalog - Derive data
impl Catalog {
    //mi clear_derived_data
    /// Clear the derived data (lists of stars in which subcubes, for
    /// example)
    fn clear_derived_data(&mut self) {
        if self.has_derived_data() {
            self.subcubes.clear();
        }
    }

    //mp derive_data
    /// Derive data from the stars in the catalog - such as what stars
    /// are in which subcubes
    ///
    /// This does not impact the sorting - indeed, usually the catalog
    /// is sorted before the data is derived.
    pub fn derive_data(&mut self) {
        if self.has_derived_data() {
            return;
        }
        self.allocate_subcubes();
    }

    //mi allocate_subcubes
    /// Allocate the subcubes and put the stars in appropriately
    fn allocate_subcubes(&mut self) {
        if self.has_derived_data() {
            return;
        }
        self.subcubes.clear();
        for _ in 0..Subcube::NUM_SUBCUBES {
            self.subcubes.push(vec![]);
        }
        for (i, s) in self.stars.iter().enumerate() {
            self.subcubes[s.subcube.as_usize()].push(CatalogIndex(i));
        }
    }

    //mp sort
    /// Sort the stars so that to create the index (and hence
    /// afterwards they can be searched by id)
    ///
    /// To do: Must remap name identifiers too
    pub fn sort(&mut self) {
        self.stars.sort_by_key(|a| a.id);
        self.clear_derived_data();
        self.sorted = true;
    }
}

//ip Catalog - Searching
impl Catalog {
    //mp is_filtered
    /// Run all the filters
    pub fn is_filtered(&self, star: &Star, n: usize) -> bool {
        self.filter.call(star, n)
    }

    //mp find_sorted
    /// Find a star from its ID
    ///
    /// The catalog must have been sorted beforehand
    pub fn find_sorted(&self, id: usize) -> Option<CatalogIndex> {
        assert!(
            self.is_sorted(),
            "Attempt to find_sorted when Catalog was not sorted"
        );
        match self.stars.binary_search_by(|a| a.id.cmp(&id)) {
            Ok(x) => Some(CatalogIndex(x)),
            Err(_) => None,
        }
    }

    //mp find_name
    /// Find a star from its name
    pub fn find_name(&self, name: &str) -> Option<CatalogIndex> {
        self.named_stars.get(name).copied()
    }

    //mp find_id_or_name
    /// Find a star from a string, which might be an id or a name
    pub fn find_id_or_name(&self, s: &str) -> Result<CatalogIndex, Error> {
        match s.parse::<usize>() {
            Err(_) => {
                if let Some(s) = self.find_name(s) {
                    Ok(s)
                } else {
                    Err(Error::FailedToFindName)
                }
            }
            Ok(id) => {
                if let Some(s) = self.find_sorted(id) {
                    Ok(s)
                } else {
                    Err(Error::FailedToFindId(id))
                }
            }
        }
    }

    //mp closest_to_dir
    /// Find the closest star in the catalog given a direction vector
    ///
    /// This requires the catalog to have had its data derived
    /// beforehand
    #[track_caller]
    pub fn closest_to_dir<I>(
        &self,
        subcube_iter: I,
        vector: &[f64; 3],
    ) -> Option<(f64, CatalogIndex)>
    where
        I: Iterator<Item = Subcube>,
    {
        assert!(
            self.has_derived_data(),
            "Attempt to find a star in the Catalog that has not has its data derived"
        );
        let vector: Vec3 = vector.into();
        let mut closest = None;
        for s in subcube_iter {
            for index in self[s].iter() {
                if !self.filter.call(&self[*index], 0) {
                    continue;
                }
                let cv = &self[*index].vector;
                let c = cv.dot(&vector);
                if let Some((cc, _)) = closest {
                    if c > cc {
                        closest = Some((c, *index));
                    }
                } else {
                    closest = Some((c, *index));
                }
            }
        }
        closest
    }

    //mp closest_to_ra_de
    /// Find the closest star in the catalog given an RA and DE in radians
    ///
    /// This requires the catalog to have had its data derived
    /// beforehand
    #[track_caller]
    pub fn closest_to_ra_de<I>(
        &self,
        subcube_iter: I,
        ra: f64,
        de: f64,
    ) -> Option<(f64, CatalogIndex)>
    where
        I: Iterator<Item = Subcube>,
    {
        let v = Star::vec_of_ra_de(ra, de);
        self.closest_to_dir(subcube_iter, &v)
    }

    //mp find_stars_around
    /// Find stars within a certain angle around a vector
    ///
    /// Needs data to have been derived for the Catalog
    ///
    /// max_angle is in radians
    #[track_caller]
    pub fn find_stars_around(&self, vector: &[f64; 3], max_angle: f64) -> Vec<CatalogIndex> {
        assert!(
            self.has_derived_data(),
            "Attempt to find a star in the Catalog that has not has its data derived"
        );

        // The maximum angle between the vector and stars to return is max_angle
        //
        // The maximum angle between any star in a subcube and the vector of the subcube is half the angle subtended by the subcube, which is asin(sqrt(3)*HALF_SUBCUBE_SIDE / 1)
        //
        // Hence the maximum angle between the vector and the subcube containing stars within max_angle is max_angle PLUS this half-angle-subtended-by-subcube
        let max_subcube_cos = (max_angle + Subcube::half_max_angle_subtended()).cos();
        let mut result = vec![];
        let max_cos = max_angle.cos();
        for sub in Subcube::iter_sphere_within_cos_of_vector(vector, max_subcube_cos) {
            for index in self[sub].iter() {
                let star = &self[*index];
                let c = star.vector.dot(&vector);
                if c < max_cos {
                    continue;
                }
                if !self.filter.call(star, result.len()) {
                    continue;
                }
                result.push(*index);
            }
        }
        result
    }

    /// Find a triangle of stars given the visual angles between them
    ///
    /// This uses the three angles in the [StarTriangleSearch] - the angles between each pair of three triangles
    ///
    /// Needs data to have been derived for the Catalog
    #[track_caller]
    pub fn find_star_triangles<I>(
        &self,
        subcube_iter: I,
        search: &StarTriangleSearch,
        max_candidates: usize,
    ) -> (bool, Vec<StarTriangleMatch>)
    where
        I: Iterator<Item = Subcube> + Clone,
    {
        let mut result = vec![];
        let completed_search = self.map_star_triangles(
            subcube_iter,
            search,
            |abc: StarTriangle| {
                let tm = search.triangle_match(self, abc);
                if tm.angle_sum() < 3.0 * search.max_angle_delta {
                    result.push(tm);
                }
            },
            max_candidates,
        );
        (completed_search, result)
    }

    /// Call a function for every triangle of stars given the visual angles between them
    ///
    /// Needs data to have been derived for the Catalog
    ///
    /// This uses the three angles in the [StarTriangleSearch] - the angles between each pair of three triangles
    ///
    /// For each pair it generates an angle range that it will look for
    /// candidate star pairs; it turns this into a range of cosines for each
    /// pair. Hence each pair has a minimum cosine and a maximum cosine for the
    /// angle between two candidate stars.
    #[track_caller]
    pub fn map_star_triangles<I, F>(
        &self,
        subcube_iter: I,
        search: &StarTriangleSearch,
        mut map: F,
        max_candidates: usize,
    ) -> bool
    where
        I: Iterator<Item = Subcube> + Clone,
        F: FnMut(StarTriangle) -> (),
    {
        assert!(
            self.has_derived_data(),
            "Attempt to find a star in the Catalog that has not has its data derived"
        );

        // Find the range of subcube centre angles that are allowed for each of the triangle angles
        let subcube_max_angle = 2.0 * (Subcube::SUBCUBE_RADIUS).asin();
        let subcube_angle_ranges: Vec<(f64, f64)> = search
            .angles_to_find
            .iter()
            .map(|a| {
                (
                    (*a - search.max_angle_delta - subcube_max_angle).max(0.),
                    (*a + search.max_angle_delta + subcube_max_angle)
                        .min(std::f64::consts::PI / 2.),
                )
            })
            .collect();
        let subcube_cos_angle_ranges: Vec<(f64, f64)> = subcube_angle_ranges
            .iter()
            .map(|(min, max)| (max.cos(), min.cos()))
            .collect();

        // Determine the delta to the subcube for each angle (subcubes
        // outside the delta range for a subcube are guaranteed to
        // have a larger angle between all the stars in them than any
        // of the angle deltas that are being looked for)
        //
        // With max mag 7...
        // For max 33.57 degrees (mag 7.0) needs range = 8
        // For max 25.71 degrees (mag 5.0) needs range = 7
        // let range = Subcube::ELE_PER_SIDE / 2;
        // For max 15.71 degrees (mag 5.0)  needs range = 3
        // let range = Subcube::ELE_PER_SIDE / 2;
        let max_angle = search
            .angles_to_find
            .iter()
            .fold(0.0, |acc: f64, b| acc.max(*b));
        let subcube_range = (max_angle / subcube_max_angle).trunc() as usize + 3;

        // Run through all the supplied subcubes
        let mut number_candidates_tried = 0;
        let mut number_found = 0;
        let mut subcubes_to_search = vec![];

        for sub0 in subcube_iter {
            if self[sub0].is_empty() {
                continue;
            }

            // Before we run through the stars in the subcube, find all
            // the subcubes that are close enough to this one for the
            // neighbors we are going to have to look for; this is for both [1] and [2]
            //
            // For large angle this might be doing 50x the work required
            //
            // However, for small angles the subcubes_to_search will only
            // be about 6 things, all relevant,
            let sub0_center = sub0.center_non_unit().normalize();
            subcubes_to_search.clear();
            let min_cos = subcube_cos_angle_ranges[0]
                .0
                .min(subcube_cos_angle_ranges[1].0);
            let max_cos = subcube_cos_angle_ranges[0]
                .1
                .max(subcube_cos_angle_ranges[1].1);
            for s12 in sub0.iter_range(subcube_range) {
                if self[s12].is_empty() {
                    continue;
                }
                let Some(c) = s12.cos_angle_on_sphere(&sub0_center) else {
                    continue;
                };
                if c < min_cos || c > max_cos {
                    continue;
                }
                subcubes_to_search.push(s12)
            }

            // For all the stars (by CatalogIndex) in the subcube of interest...
            for i0 in self[sub0].iter() {
                let s0 = &self[*i0];
                if !self.filter.call(s0, number_found) {
                    continue;
                }
                // iterate through subcubes_to_search, skipping those that are nowhere near angles_to_find[0] away
                let subcubes_for_s0 = subcubes_to_search
                    .iter()
                    .filter(|s| {
                        let c = s.center_non_unit().normalize().dot(&sub0_center);
                        c > subcube_cos_angle_ranges[0].0 && c < subcube_cos_angle_ranges[0].1
                    })
                    .copied();

                // For all the other stars (by CatalogIndex) in the subcubes around s0
                //
                // Ditch them if they have cos(angle) to the first star outside of the cos range permitted
                for sub1 in subcubes_for_s0 {
                    for i1 in self[sub1].iter() {
                        if *i0 == *i1 {
                            continue;
                        }
                        let s1 = &self[*i1];
                        if !self.filter.call(s1, number_found) {
                            continue;
                        }

                        let c_s01 = s0.cos_angle_between(s1);
                        if c_s01 < search.cos_max_angles_to_find[0]
                            || c_s01 > search.cos_min_angles_to_find[0]
                        {
                            continue;
                        }

                        let sub1_center = sub1.center_non_unit().normalize();
                        let subcubes_for_s1 = subcubes_to_search
                            .iter()
                            .filter(|s| {
                                let c = s.center_non_unit().normalize().dot(&sub1_center);
                                c > subcube_cos_angle_ranges[2].0
                                    && c < subcube_cos_angle_ranges[2].1
                            })
                            .filter(|s| {
                                let c = s.center_non_unit().normalize().dot(&sub0_center);
                                c > subcube_cos_angle_ranges[1].0
                                    && c < subcube_cos_angle_ranges[1].1
                            })
                            .copied();

                        // For all the other stars (by CatalogIndex) in the subcubes around s0
                        //
                        // Ditch them if they have cos(angle) to the first star outside of the cos range permitted, or the angle to the second star is ditto
                        for sub2 in subcubes_for_s1 {
                            for i2 in self[sub2].iter() {
                                if *i0 == *i2 || *i1 == *i2 {
                                    continue;
                                }

                                number_candidates_tried += 1;
                                if number_candidates_tried >= max_candidates {
                                    return false;
                                }

                                let s2 = &self[*i2];
                                if !self.filter.call(s2, number_found) {
                                    continue;
                                }
                                let c_s02 = s0.cos_angle_between(s2);
                                if c_s02 < search.cos_max_angles_to_find[1]
                                    || c_s02 > search.cos_min_angles_to_find[1]
                                {
                                    continue;
                                }
                                let c_s12 = s1.cos_angle_between(s2);
                                if c_s12 < search.cos_max_angles_to_find[2]
                                    || c_s12 > search.cos_min_angles_to_find[2]
                                {
                                    continue;
                                }
                                // angles [0], [1] and [2] are the angles at the points A, B and C
                                // i0<>i1 matches angle [0] at A (star opposite BC)
                                // i0<>i2 matches angle [1] at B (star opposite CA)
                                // i1<>i2 matches angle [2] at C (star opposite AB)
                                //
                                // Hence i0 is C; i1 is B, i2 is A
                                //
                                // For the angles opposite AB/CB/AC we need C,A,B which is i2, i1, i0
                                number_found += 1;
                                map((*i2, *i1, *i0).into());
                            }
                        }
                    }
                }
            }
        }
        true
    }

    /// Finds the best star mappings for
    ///
    /// Needs data to have been derived for the Catalog
    ///
    /// Returns an indicatino that max_candides was sufficient, and an
    /// *unsorted* vec of [StarMatchMappingSet], which indicate which three main
    /// stars were mapped, the actual mappings of the vectors provided, the
    /// derived quaternion, and the quality of the match
    #[track_caller]
    pub fn find_best_star_mappings<I>(
        &self,
        subcube_iter: I,
        img_space_vectors: &[[f64; 3]],
        max_angle_delta: f64,
        max_candidates: usize,
    ) -> (bool, Vec<StarMatchMappingSet>)
    where
        I: Iterator<Item = Subcube> + Clone,
    {
        assert!(
            self.has_derived_data(),
            "Attempt to find a star in the Catalog that has not has its data derived"
        );

        let Some(search) = StarTriangleSearch::of_vectors(img_space_vectors, max_angle_delta)
        else {
            return (true, vec![]);
        };

        let (finished, mut candidates) =
            self.find_star_triangles(subcube_iter.clone(), &search, max_candidates);
        candidates.sort_by(StarTriangleMatch::compare_angle_sum);
        let mut mapped_candidates = vec![];
        for c in candidates {
            let mut okay = true;
            let q = c.quaternion();
            let mut mapping_set = crate::StarMatchMappingSet {
                initial_match: c,
                mappings: vec![],
                q,
                angle_mean: 0.0,
                quality: 0.0,
            };
            for (i, isv) in img_space_vectors.iter().enumerate() {
                let v = q.apply3_arr(isv);
                let Some(cos_star) = self.closest_to_dir(subcube_iter.clone(), &v) else {
                    okay = false;
                    break;
                };
                mapping_set.mappings.push(StarMatchMapping {
                    star: cos_star.1,
                    img_index: i,
                    ordering: 0.0,
                    quality: 0.0,
                    img_vector: (*isv).into(),
                    star_vector: (*self[cos_star.1].vector()).into(),
                });
            }
            if !okay {
                continue;
            }
            mapping_set.order_by_angle();
            mapping_set.generate_q();
            mapping_set.derive_stats();
            mapped_candidates.push(mapping_set);
        }
        (finished, mapped_candidates)
    }
}

//ip Catalog - Iterators
impl Catalog {
    //mp iter_stars
    pub fn iter_stars(&self) -> StarIter<'_> {
        StarIter {
            catalog: self,
            i: 0,
        }
    }

    //mp iter_within_subcubes
    /// Iterate over all the stars in the catalog within a set of
    /// subcubes provide by an iterator
    pub fn iter_within_subcubes<I>(&self, subcube_iter: I) -> StarSubcubeIter<'_, I>
    where
        I: std::iter::Iterator<Item = Subcube>,
    {
        StarSubcubeIter {
            catalog: self,
            subcube_iter,
            subcube: None,
            i: 0,
        }
    }
}

//ip Index<CatalogIndex> for Catalog
impl std::ops::Index<CatalogIndex> for Catalog {
    type Output = Star;
    fn index(&self, s: CatalogIndex) -> &Star {
        &self.stars[s.0]
    }
}

//ip Index<Subcube> for Catalog
impl std::ops::Index<Subcube> for Catalog {
    type Output = Vec<CatalogIndex>;
    fn index(&self, q: Subcube) -> &Vec<CatalogIndex> {
        &self.subcubes[q.as_usize()]
    }
}
// impl std::ops::IndexMut<Subcube> for Catalog {
//     fn index_mut(&mut self, q: Subcube) -> &mut Vec<usize> {
//         &mut self.subcubes[q.as_usize()]
//     }
// }

//a StarIter
//tp StarIter
/// An iterator over *all* the stars in a catalog
pub struct StarIter<'a> {
    catalog: &'a Catalog,
    i: usize,
}

//ip Iterator for StarIter
impl<'a> std::iter::Iterator for StarIter<'a> {
    type Item = &'a Star;
    fn next(&mut self) -> Option<&'a Star> {
        if self.i < self.catalog.len() {
            let i = self.i;
            self.i += 1;
            Some(&self.catalog.stars[i])
        } else {
            None
        }
    }
}

//a StarSubcubeIter
//tp StarSubcubeIter
/// An iterator of stars in a catalog that line within subcubes
/// provided by an iterator of Subcube
pub struct StarSubcubeIter<'a, I>
where
    I: std::iter::Iterator<Item = Subcube>,
{
    catalog: &'a Catalog,
    subcube_iter: I,
    subcube: Option<Subcube>,
    i: usize,
}

//ip Iterator for StarSubcubeIter
impl<'a, I> std::iter::Iterator for StarSubcubeIter<'a, I>
where
    I: std::iter::Iterator<Item = Subcube>,
{
    type Item = &'a Star;
    fn next(&mut self) -> Option<&'a Star> {
        loop {
            if self.subcube.is_none() {
                self.subcube = self.subcube_iter.next();
            }
            let subcube = self.subcube?;
            if self.i < self.catalog[subcube].len() {
                let i = self.i;
                self.i += 1;
                return Some(&self.catalog.stars[self.catalog[subcube][i].0]);
            }
            self.i = 0;
            self.subcube = None;
        }
    }
}
