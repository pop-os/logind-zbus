// signal function name, Stream name, Iter name
macro_rules! receive_signal_name {
    ( $receive_name:ident, $receive_stream:ty, $receive_iter:ty) => {
        #[inline]
        #[cfg(feature = "non_blocking")]
        pub fn $receive_name(&self) -> Result<$receive_stream> {
            self.0.$receive_name()
        }

        #[inline]
        #[cfg(not(feature = "non_blocking"))]
        pub fn $receive_name(&self) -> Result<$receive_iter> {
            self.0.$receive_name()
        }
    };
}
