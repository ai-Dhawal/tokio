use super::typeid;
#[derive(Debug)]
pub(crate) enum OwnedBuf {
    Vec(Vec<u8>),
    #[cfg(feature = "io-util")]
    Bytes(bytes::Bytes),
}
impl AsRef<[u8]> for OwnedBuf {
    fn as_ref(&self) -> &[u8] {
        panic!("STUB: not implemented");
    }
}
pub(crate) fn upgrade<B: AsRef<[u8]>>(buf: B) -> OwnedBuf {
    panic!("STUB: not implemented");
}
