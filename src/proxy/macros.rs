// signal function name, Stream name, Iter name
macro_rules! receive_signal_name {
    ( $receive_name:ident, $receive_stream:ty, $receive_iter:ty) => {
        #[inline]
        pub fn $receive_name(&self) -> Result<$receive_iter> {
            self.0.$receive_name()
        }
    };
}