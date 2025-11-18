use crate::tecs_global::*;
pub trait SXUart {
	fn open(&self);
	fn put_char(&self, c: &u8)-> Result<(),()>;
	fn get_char(&self)-> Result<u8,()>;
}
