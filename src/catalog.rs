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

///
/// There are roughly 4,000 (3769 in fact) subcubes used out of 32^3 (i.e. 32,000)
/// by a star catalog with 32 subcube size
///
/// Purely using a vec of 32^3 vecs is 1MB of storage, plus subcube contents
///
/// A hash map from subcube to vec with 4k entries used is probably
/// 64kB plus 128kB for the vecs themselves; access is slower though
///
/// Current choice is to just use vec of vecs
#[derive(Default, Serialize, Deserialize)]
pub struct Catalog {
    /// Stars in the catalog
    ///
    /// When searching this must be sorted by id; if not sorted by id
    /// then searching will return errors
    stars: Vec<Star>,
    sorted: bool,
    /// Stars dictionary to map name to an index in stars
    named_stars: HashMap<String, usize>,
    /// Star indices within each subcube
    #[serde(skip)]
    subcubes: Vec<Vec<usize>>,
}

impl Catalog {
    pub fn star(&self, index: usize) -> &Star {
        &self.stars[index]
    }
    pub fn len(&self) -> usize {
        self.stars.len()
    }
    pub fn is_empty(&self) -> bool {
        self.stars.is_empty()
    }
    pub fn is_sorted(&self) -> bool {
        self.sorted
    }
    fn has_derived_data(&self) -> bool {
        !self.subcubes.is_empty()
    }
    fn clear_derived_data(&mut self) {
        if self.has_derived_data() {
            self.subcubes.clear();
        }
    }
    pub fn derive_data(&mut self) {
        if self.has_derived_data() {
            return;
        }
        self.allocate_subcubes();
    }
    pub fn add_star(&mut self, star: Star) {
        self.clear_derived_data();
        self.sorted = false;
        self.stars.push(star);
    }
    pub fn allocate_subcubes(&mut self) {
        if self.has_derived_data() {
            return;
        }
        self.subcubes.clear();
        for _ in 0..Subcube::NUM_SUBCUBES {
            self.subcubes.push(vec![]);
        }
        for (i, s) in self.stars.iter().enumerate() {
            self.subcubes[s.subcube().as_usize()].push(i)
        }
    }
    // Must remap name identifiers too
    pub fn sort(&mut self) {
        self.stars.sort_by_key(|a| a.id());
        self.clear_derived_data();
        self.sorted = true;
    }
    pub fn add_name<I: Into<String>>(&mut self, index: usize, name: I) {
        self.named_stars.insert(name.into(), index);
    }
    pub fn add_names<I: Into<String> + Copy>(
        &mut self,
        id_names: &[(usize, I)],
    ) -> Result<(), Error> {
        for (id, name) in id_names {
            let Some(index) = self.find_sorted(*id) else {
                return Err(Error::FailedToFindId(*id));
            };
            self.named_stars.insert((*name).into(), index);
        }
        Ok(())
    }
    pub fn find_sorted(&mut self, id: usize) -> Option<usize> {
        assert!(
            self.is_sorted(),
            "Attempt to find_sorted when Catalog was not sorted"
        );
        match self.stars.binary_search_by(|a| a.id().cmp(&id)) {
            Ok(x) => Some(x),
            Err(_) => None,
        }
    }
    pub fn find_name(&self, name: &str) -> Option<usize> {
        self.named_stars.get(name).copied()
    }

    /// Find the closest star in the catalog given an RA and DE in radians
    pub fn closest_to(&self, ra: f64, de: f64) -> Option<(f64, usize)> {
        assert!(
            self.has_derived_data(),
            "Attempt to find a star in the Catalog that has not has its data derived"
        );
        let v = Star::vec_of_ra_de(ra, de);
        let s = Subcube::of_vector(&v);
        let mut closest = None;
        for s in s.iter_neighbors() {
            for index in self[s].iter() {
                let c = v.dot(self.stars[*index].vector());
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

impl std::ops::Index<Subcube> for Catalog {
    type Output = Vec<usize>;
    fn index(&self, q: Subcube) -> &Vec<usize> {
        &self.subcubes[q.as_usize()]
    }
}
impl std::ops::IndexMut<Subcube> for Catalog {
    fn index_mut(&mut self, q: Subcube) -> &mut Vec<usize> {
        &mut self.subcubes[q.as_usize()]
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
