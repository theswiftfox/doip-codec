pub mod header;
pub mod payload;
pub mod version;

#[cfg(test)]
mod size_tests {
    use std::mem;

    use crate::doip::header::{header::DoipHeader, version::DoipVersion};

    #[test]
    fn test_struct_sizes() {
        dbg!(mem::size_of::<DoipHeader>());
        dbg!(mem::size_of::<DoipVersion>());
    }
}
