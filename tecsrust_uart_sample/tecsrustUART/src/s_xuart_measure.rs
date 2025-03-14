pub trait SXuartMeasure {
	fn open(&'static self);
	fn put_char(&'static self, c: &u8)-> bool;
	fn get_char(&'static self, c: &mut u8)-> bool;
}
