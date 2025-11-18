pub trait SXuartMeasure {
	fn open(&'static self);
	fn put_char(&'static self, c: &u8)-> bool;
}
