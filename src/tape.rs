pub trait Tape {
    fn read(&mut self) -> Option<i64>;
    fn write(&mut self, value: i64);
} 