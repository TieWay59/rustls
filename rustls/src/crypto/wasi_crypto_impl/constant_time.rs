use crate::Error;

pub(crate) fn verify_slices_are_equal(a: &[u8], b: &[u8]) -> Result<(), Error> {
    if b == a {
        Ok(())
    } else {
        Err(Error::General("Unspecified".into()))
    }
}
