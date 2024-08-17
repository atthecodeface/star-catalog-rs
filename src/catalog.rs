use std::collections::HashMap;

use geo_nd::Vector;
use serde::{Deserialize, Serialize};

use crate::{Star, Subcube};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to find Id {0} in the catalog")]
    FailedToFindId(usize),
    #[error("Failed to read CSV file")]
    #[from(csv::Error)]
    CsvError,
}

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
    subcubes: Vec<Vec<usize>>,
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
            self.subcubes[s.subcube.as_usize()].push(i)
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
    pub fn add_names<I: Into<String> + Copy>(
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
            self.named_stars.insert((*name).into(), index);
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
                let c = v.dot(&self.stars[*index].vector);
                if let Some((cc, _)) = closest {
                    if c > cc {
                        closest = Some((c, CatalogIndex(*index)));
                    }
                } else {
                    closest = Some((c, CatalogIndex(*index)));
                }
            }
        }
        closest
    }

    //mp iter_within_subcubes
    /// Iteratre over all the stars in the catalog within a set of
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
}

impl std::ops::Index<CatalogIndex> for Catalog {
    type Output = Star;
    fn index(&self, s: CatalogIndex) -> &Star {
        &self.stars[s.0]
    }
}

impl std::ops::Index<Subcube> for Catalog {
    type Output = Vec<usize>;
    fn index(&self, q: Subcube) -> &Vec<usize> {
        &self.subcubes[q.as_usize()]
    }
}
// impl std::ops::IndexMut<Subcube> for Catalog {
//     fn index_mut(&mut self, q: Subcube) -> &mut Vec<usize> {
//         &mut self.subcubes[q.as_usize()]
//     }
// }
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
            let Some(subcube) = self.subcube else {
                return None;
            };
            while self.i < self.catalog[subcube].len() {
                let i = self.i;
                self.i += 1;
                return Some(&self.catalog.stars[self.catalog[subcube][i]]);
            }
            self.i = 0;
            self.subcube = None;
        }
    }
}
