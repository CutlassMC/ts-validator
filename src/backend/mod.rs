#[cfg(feature = "const")]
#[path = "const_backend/mod.rs"]
mod backend_impl;

#[cfg(feature = "ref-impl")]
#[path = "ref_backend.rs"]
mod backend_impl;

#[cfg(not(feature = "backend"))]
compile_error!(
    "No backend is implemented, which is the result of an invalid configuration of the crate"
);
