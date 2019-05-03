use crate::{ContextHandle, Geometry, GResult, AsRaw, ContextHandling, ContextInteractions};
use error::PredicateType;
use context_handle::PtrWrap;
use geos_sys::*;
use functions::*;
use std::sync::Arc;
use error::Error;

/// `PreparedGeometry` is an interface which prepares [`Geometry`] for greater performance
/// on repeated calls.
///
/// # Example
///
/// ```
/// use geos::Geometry;
///
/// let geom1 = Geometry::new_from_wkt("POLYGON((0 0, 10 0, 10 6, 0 6, 0 0))").expect("Invalid geometry");
/// let mut prepared_geom = geom1.to_prepared_geom()
///                              .expect("failed to create prepared geom");
/// let geom2 = Geometry::new_from_wkt("POINT (2.5 2.5)").expect("Invalid geometry");
///
/// assert_eq!(prepared_geom.contains(&geom2), Ok(true));
/// ```
pub struct PreparedGeometry<'a> {
    ptr: PtrWrap<*mut GEOSPreparedGeometry>,
    context: Arc<ContextHandle<'a>>,
}

impl<'a> PreparedGeometry<'a> {
    /// Creates a new `PreparedGeometry` from a [`Geometry`].
    ///
    /// # Example
    ///
    /// ```
    /// use geos::{Geometry, PreparedGeometry};
    ///
    /// let geom1 = Geometry::new_from_wkt("POLYGON((0 0, 10 0, 10 6, 0 6, 0 0))").expect("Invalid geometry");
    /// let prepared_geom = PreparedGeometry::new(&geom1);
    /// ```
    pub fn new(g: &Geometry<'a>) -> GResult<PreparedGeometry<'a>> {
        unsafe {
            let ptr = GEOSPrepare_r(g.get_raw_context(), g.as_raw());
            PreparedGeometry::new_from_raw(ptr, g.clone_context(), "new")
        }
    }

    pub(crate) unsafe fn new_from_raw(
        ptr: *mut GEOSPreparedGeometry,
        context: Arc<ContextHandle<'a>>,
        caller: &str,
    ) -> GResult<PreparedGeometry<'a>> {
        if ptr.is_null() {
            return Err(Error::NoConstructionFromNullPtr(format!("PreparedGeometry::{}", caller)));
        }
        Ok(PreparedGeometry { ptr: PtrWrap(ptr), context })
    }

    /// Returns `true` if no points of the other geometry is outside the exterior of `self`.
    ///
    /// ```
    /// use geos::Geometry;
    ///
    /// let geom1 = Geometry::new_from_wkt("POLYGON((0 0, 10 0, 10 6, 0 6, 0 0))").expect("Invalid geometry");
    /// let mut prepared_geom = geom1.to_prepared_geom()
    ///                              .expect("failed to create prepared geom");
    /// let geom2 = Geometry::new_from_wkt("POINT (2.5 2.5)").expect("Invalid geometry");
    ///
    /// assert_eq!(prepared_geom.contains(&geom2), Ok(true));
    /// ```
    pub fn contains<'b>(&self, g2: &Geometry<'b>) -> GResult<bool> {
        let ret_val = unsafe {
            GEOSPreparedContains_r(self.get_raw_context(), self.as_raw(), g2.as_raw())
        };
        check_geos_predicate(ret_val, PredicateType::PreparedContains)
    }

    pub fn contains_properly<'b>(&self, g2: &Geometry<'b>) -> GResult<bool> {
        let ret_val = unsafe {
            GEOSPreparedContainsProperly_r(self.get_raw_context(), self.as_raw(), g2.as_raw())
        };
        check_geos_predicate(ret_val, PredicateType::PreparedContainsProperly)
    }

    pub fn covered_by<'b>(&self, g2: &Geometry<'b>) -> GResult<bool> {
        let ret_val = unsafe {
            GEOSPreparedCoveredBy_r(self.get_raw_context(), self.as_raw(), g2.as_raw())
        };
        check_geos_predicate(ret_val, PredicateType::PreparedCoveredBy)
    }

    pub fn covers<'b>(&self, g2: &Geometry<'b>) -> GResult<bool> {
        let ret_val = unsafe {
            GEOSPreparedCovers_r(self.get_raw_context(), self.as_raw(), g2.as_raw())
        };
        check_geos_predicate(ret_val, PredicateType::PreparedCovers)
    }

    pub fn crosses<'b>(&self, g2: &Geometry<'b>) -> GResult<bool> {
        let ret_val = unsafe {
            GEOSPreparedCrosses_r(self.get_raw_context(), self.as_raw(), g2.as_raw())
        };
        check_geos_predicate(ret_val, PredicateType::PreparedCrosses)
    }

    pub fn disjoint<'b>(&self, g2: &Geometry<'b>) -> GResult<bool> {
        let ret_val = unsafe {
            GEOSPreparedDisjoint_r(self.get_raw_context(), self.as_raw(), g2.as_raw())
        };
        check_geos_predicate(ret_val, PredicateType::PreparedDisjoint)
    }

    pub fn intersects<'b>(&self, g2: &Geometry<'b>) -> GResult<bool> {
        let ret_val = unsafe {
            GEOSPreparedIntersects_r(self.get_raw_context(), self.as_raw(), g2.as_raw())
        };
        check_geos_predicate(ret_val, PredicateType::PreparedIntersects)
    }

    pub fn overlaps<'b>(&self, g2: &Geometry<'b>) -> GResult<bool> {
        let ret_val = unsafe {
            GEOSPreparedOverlaps_r(self.get_raw_context(), self.as_raw(), g2.as_raw())
        };
        check_geos_predicate(ret_val, PredicateType::PreparedOverlaps)
    }

    pub fn touches<'b>(&self, g2: &Geometry<'b>) -> GResult<bool> {
        let ret_val = unsafe {
            GEOSPreparedTouches_r(self.get_raw_context(), self.as_raw(), g2.as_raw())
        };
        check_geos_predicate(ret_val, PredicateType::PreparedTouches)
    }

    pub fn within<'b>(&self, g2: &Geometry<'b>) -> GResult<bool> {
        let ret_val = unsafe {
            GEOSPreparedWithin_r(self.get_raw_context(), self.as_raw(), g2.as_raw())
        };
        check_geos_predicate(ret_val, PredicateType::PreparedWithin)
    }
}

unsafe impl<'a> Send for PreparedGeometry<'a> {}
unsafe impl<'a> Sync for PreparedGeometry<'a> {}

impl<'a> Drop for PreparedGeometry<'a> {
    fn drop(&mut self) {
        unsafe { GEOSPreparedGeom_destroy_r(self.get_raw_context(), self.as_raw()) };
    }
}

impl<'a> ContextInteractions<'a> for PreparedGeometry<'a> {
    /// Set the context handle to the `PreparedGeometry`.
    ///
    /// ```
    /// use geos::{ContextInteractions, ContextHandle, Geometry, PreparedGeometry};
    ///
    /// let point_geom = Geometry::new_from_wkt("POINT (2.5 2.5)").expect("Invalid geometry");
    /// let context_handle = ContextHandle::init().expect("invalid init");
    /// let mut prepared_geom = point_geom.to_prepared_geom()
    ///                                   .expect("failed to create prepared geom");
    /// context_handle.set_notice_message_handler(Some(Box::new(|s| println!("new message: {}", s))));
    /// prepared_geom.set_context_handle(context_handle);
    /// ```
    fn set_context_handle(&mut self, context: ContextHandle<'a>) {
        self.context = Arc::new(context);
    }

    /// Get the context handle of the `PreparedGeometry`.
    ///
    /// ```
    /// use geos::{ContextInteractions, CoordDimensions, Geometry, PreparedGeometry};
    ///
    /// let point_geom = Geometry::new_from_wkt("POINT (2.5 2.5)").expect("Invalid geometry");
    /// let prepared_geom = point_geom.to_prepared_geom()
    ///                               .expect("failed to create prepared geom");
    /// let context = prepared_geom.get_context_handle();
    /// context.set_notice_message_handler(Some(Box::new(|s| println!("new message: {}", s))));
    /// ```
    fn get_context_handle(&self) -> &ContextHandle<'a> {
        &self.context
    }
}

impl<'a> AsRaw for PreparedGeometry<'a> {
    type RawType = *mut GEOSPreparedGeometry;

    fn as_raw(&self) -> Self::RawType {
        *self.ptr
    }
}

impl<'a> ContextHandling for PreparedGeometry<'a> {
    type Context = Arc<ContextHandle<'a>>;

    fn get_raw_context(&self) -> GEOSContextHandle_t {
        self.context.as_raw()
    }

    fn clone_context(&self) -> Arc<ContextHandle<'a>> {
        Arc::clone(&self.context)
    }
}
