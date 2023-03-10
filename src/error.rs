use core::fmt;
#[cfg(nightly)]
use core::{
    any::{Demand, Provider},
    error::Error,
};
#[cfg(not(nightly))]
use std::error::Error;

use crate::Report;

#[repr(transparent)]
pub(crate) struct ReportError(Report);

impl ReportError {
    pub(crate) const fn new(report: Report) -> Self {
        Self(report)
    }

    pub(crate) const fn from_ref(report: &Report) -> &Self {
        // SAFETY: `ReportError` is a `repr(transparent)` wrapper around `Report`.
        unsafe { &*(report as *const Report).cast() }
    }
}

impl fmt::Debug for ReportError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, fmt)
    }
}

impl fmt::Display for ReportError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, fmt)
    }
}

impl Error for ReportError {
    #[cfg(nightly)]
    fn provide<'a>(&'a self, demand: &mut Demand<'a>) {
        self.0.frames().for_each(|frame| frame.provide(demand));
    }
}
