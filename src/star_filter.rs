use geo_nd::Vector;

use std::rc::Rc;

use crate::{Star, Vec3};

//a StarFilterFn
pub trait StarFilterFn: Fn(&Star, usize) -> bool + 'static {}

//ip StarFilterFn for Fn(&Star, usize) -> bool
impl<F> StarFilterFn for F where F: for<'a> Fn(&'a Star, usize) -> bool + 'static {}

//tp StarFilter
#[derive(Clone)]
pub struct StarFilter(Rc<dyn StarFilterFn>);

//ip Default for StarFilter
impl std::default::Default for StarFilter {
    fn default() -> Self {
        let f = Rc::new(|_s: &Star, _n: usize| true);
        StarFilter(f)
    }
}

//ip From<StarFilterFn> for StarFilter
impl<F: StarFilterFn + 'static> From<F> for StarFilter {
    fn from(f: F) -> Self {
        let f = Rc::new(f);
        Self(f)
    }
}

//ip StarFilter
impl StarFilter {
    //mp call
    /// Invoke the filter
    pub fn call(&self, s: &Star, n: usize) -> bool {
        self.0(s, n)
    }

    //cp then
    /// Create a new filter that calls the current filter, an if
    /// *true* calls a follow-on filter
    pub fn then(mut self, f: StarFilter) -> Self {
        let f_first = self.0.clone();
        // self.0 = Rc::new(move |s, n| f_first(s, n) && f.call(s, n));
        self.0 = Rc::new(move |s, n| if f_first(s, n) { f.call(s, n) } else { false });
        self
    }

    //cp select
    /// Create a new filter that returns true after the first *skip* entries up to *limit*
    ///
    /// This can be used to capture a subset of star results, for example
    pub fn select(skip: usize, limit: usize) -> Self {
        let select = StarFilterSelect::new(skip, limit);
        Self(Rc::new(move |_s_, _n| select.filter()))
    }

    //cp brighter_than
    /// Create a new filter that returns true for stars brighter than a certain magnitude
    pub fn brighter_than(magnitude: f32) -> Self {
        let f = Rc::new(move |s: &Star, _n: usize| s.mag < magnitude);
        Self(f)
    }

    //cp cos_to_gt
    /// Create a new filter that returns true for stars that are
    /// closer in angle to a (unit) vector than a specified angle
    /// (this being given by its cosine)
    pub fn cos_to_gt(v: Vec3, cos: f64) -> Self {
        let f = Rc::new(move |s: &Star, _n: usize| v.dot(&s.vector) > cos);
        Self(f)
    }
}

//tp StarFilterSelect
/// A filter that skips the first *N* by returning false *N* times,
/// then returns true *M* times to accept some results, before
/// returning false thereafter
#[derive(Clone)]
pub struct StarFilterSelect {
    skip: std::cell::RefCell<usize>,
    limit: std::cell::RefCell<usize>,
}

//ip StarFilterSelect
impl StarFilterSelect {
    //cp new
    /// Create a new filter with a given skip and limit
    pub fn new(skip: usize, limit: usize) -> Self {
        let skip = skip.into();
        let limit = limit.into();
        Self { skip, limit }
    }

    //mp filter
    /// The actual function invoked to filter
    pub fn filter(&self) -> bool {
        if *self.skip.borrow() > 0 {
            *self.skip.borrow_mut() -= 1;
            false
        } else if *self.limit.borrow() > 0 {
            *self.limit.borrow_mut() -= 1;
            true
        } else {
            false
        }
    }
}
