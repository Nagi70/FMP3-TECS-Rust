use crate::tecs_global::*;
pub trait SXUartMeasure {
	fn open(&self);
	fn put_char(&self, c: &u8)-> bool;
	fn get_char(&self, c: &mut u8)-> bool;
}
