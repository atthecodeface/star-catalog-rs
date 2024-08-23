use std::collections::HashMap;

use geo_nd::Vector;
use serde::{Deserialize, Serialize};

use crate::{Error, Star, Subcube};

//tp CatalogIndex
/// An index into the Catalog to identify a particular star
///
/// A [CatalogIndex] becomes invalid if the Catalog is sorted again
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct CatalogIndex(usize);

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
    /// Star indices within each subcube
    #[serde(skip)]
    subcubes: Vec<Vec<CatalogIndex>>,
}

impl Catalog {
    //mp retain
    /// Retain stars that match a certain criterion; the rest are
    /// dropped
    ///
    /// This should be invoked prior to any stars being named; it also
    /// clears the derived data (e.g. geometric searching will not be
    /// allowed until a derive_data() call is invoked)
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Star) -> bool,
    {
        self.sorted = false;
        self.clear_derived_data();
        self.stars.retain(f);
    }

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

    //mp closest_to
    /// Find the closest star in the catalog given an RA and DE in radians
    ///
    /// This requires the catalog to have had its data derived
    /// beforehand
    pub fn closest_to(&self, ra: f64, de: f64) -> Option<(f64, CatalogIndex)> {
        assert!(
            self.has_derived_data(),
            "Attempt to find a star in the Catalog that has not has its data derived"
        );
        let v = Star::vec_of_ra_de(ra, de);
        let s = Subcube::of_vector(&v);
        let mut closest = None;
        for s in s.iter_range(1) {
            for index in self[s].iter() {
                let c = v.dot(&self.stars[index.0].vector);
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

    //mp iter_stars
    pub fn iter_stars(&self) -> StarIter {
        StarIter {
            catalog: self,
            i: 0,
        }
    }

    //mp iter_within_subcubes
    /// Iterate over all the stars in the catalog within a set of
    /// subcubes provide by an iterator
    pub fn iter_within_subcubes<I>(&self, subcube_iter: I) -> StarSubcubeIter<I>
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

    //mp find_star_triangles
    /// Find
    ///
    /// Needs data to have been derived for the Catalog
    pub fn find_star_triangles<I>(
        &self,
        subcube_iter: I,
        angles_to_find: &[f64; 3],
        max_angle_delta: f64,
    ) -> Vec<(CatalogIndex, CatalogIndex, CatalogIndex)>
    where
        I: Iterator<Item = Subcube>,
    {
        // Find the range of cosines for the angles that we will accept
        //
        // Note cos(0) > cos(0.1) so min cos is cos(angle + max)
        let cos_angle_ranges: Vec<(f64, f64)> = angles_to_find
            .iter()
            .map(|a| {
                (
                    (*a + max_angle_delta).cos(),
                    (*a - max_angle_delta).max(0.).cos(),
                )
            })
            .collect();

        // Find the range of subcube centre angles that are allowed for each of the triangle angles
        let subcube_max_angle = 2.0 * (Subcube::SUBCUBE_RADIUS).asin();
        let subcube_angle_ranges: Vec<(f64, f64)> = angles_to_find
            .iter()
            .map(|a| {
                (
                    (*a - max_angle_delta - subcube_max_angle).max(0.),
                    (*a + max_angle_delta + subcube_max_angle).min(std::f64::consts::PI / 2.),
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
        let max_angle = angles_to_find.iter().fold(0.0, |acc: f64, b| acc.max(*b));
        let subcube_range = (max_angle / subcube_max_angle).trunc() as usize + 3;

        // Run through all the supplied subcubes
        let mut result = vec![];
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
            let sub0_center = sub0.center().normalize();
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

            for i0 in self[sub0].iter() {
                let s0 = &self[*i0];
                // iterate through subcubes_to_search, skipping those that are nowhere near angles_to_find[0] away
                let subcubes_for_s0 = subcubes_to_search
                    .iter()
                    .filter(|s| {
                        let c = s.center().normalize().dot(&sub0_center);
                        c > subcube_cos_angle_ranges[0].0 && c < subcube_cos_angle_ranges[0].1
                    })
                    .copied();
                for sub1 in subcubes_for_s0 {
                    for i1 in self[sub1].iter() {
                        if *i0 == *i1 {
                            continue;
                        }
                        let s1 = &self[*i1];

                        let c_s01 = s0.cos_angle_between(s1);
                        if c_s01 < cos_angle_ranges[0].0 || c_s01 > cos_angle_ranges[0].1 {
                            continue;
                        }

                        let sub1_center = sub1.center().normalize();
                        let subcubes_for_s1 = subcubes_to_search
                            .iter()
                            .filter(|s| {
                                let c = s.center().normalize().dot(&sub1_center);
                                c > subcube_cos_angle_ranges[2].0
                                    && c < subcube_cos_angle_ranges[2].1
                            })
                            .filter(|s| {
                                let c = s.center().normalize().dot(&sub0_center);
                                c > subcube_cos_angle_ranges[1].0
                                    && c < subcube_cos_angle_ranges[1].1
                            })
                            .copied();
                        for sub2 in subcubes_for_s1 {
                            for i2 in self[sub2].iter() {
                                if *i0 == *i2 || *i1 == *i2 {
                                    continue;
                                }
                                let s2 = &self[*i2];
                                let c_s02 = s0.cos_angle_between(s2);
                                if c_s02 < cos_angle_ranges[1].0 || c_s02 > cos_angle_ranges[1].1 {
                                    continue;
                                }
                                let c_s12 = s1.cos_angle_between(s2);
                                if c_s12 < cos_angle_ranges[2].0 || c_s12 > cos_angle_ranges[2].1 {
                                    continue;
                                }
                                result.push((*i0, *i1, *i2));
                            }
                        }
                    }
                }
            }
        }
        result
    }
}

impl std::ops::Index<CatalogIndex> for Catalog {
    type Output = Star;
    fn index(&self, s: CatalogIndex) -> &Star {
        &self.stars[s.0]
    }
}

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
pub struct StarIter<'a> {
    catalog: &'a Catalog,
    i: usize,
}
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

pub struct StarSubcubeIter<'a, I>
where
    I: std::iter::Iterator<Item = Subcube>,
{
    catalog: &'a Catalog,
    subcube_iter: I,
    subcube: Option<Subcube>,
    i: usize,
}
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
