use crate::DoipCodec;
use pyo3::prelude::*;

#[cfg(feature = "std")]
#[pymodule]
fn doip_codec(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<DoipCodec>()?;
    Ok(())
}
