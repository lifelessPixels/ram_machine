#[derive(Clone)]
pub enum MemoryValue {
    Empty,
    Integral(i64)
}

impl Default for MemoryValue {
    fn default() -> Self {
        MemoryValue::Empty
    }
}

pub struct Memory {
    storage: Vec<MemoryValue>
}

impl Memory {
    pub fn new(size: usize) -> Self {
        let mut new_memory = Memory {
            storage: Vec::<MemoryValue>::new()
        };
        new_memory.storage.resize_with(size, || -> MemoryValue {MemoryValue::default() });
        new_memory
    }

    pub fn set(&mut self, value: MemoryValue, index: usize) -> Result<(), ()> {
        if index >= self.storage.len() { return Err(()); }
        self.storage[index] = value;
        Ok(())
    }

    pub fn get(&self, index: usize) -> Result<MemoryValue, ()> {
        if index >= self.storage.len() { return Err(()); }
        Ok(self.storage[index].clone())
    }

    pub fn dump(&self) {
        print!("dumping memory: ");
        for (index, value) in self.storage.iter().enumerate() {
            if index % 8 == 0 {
                println!();
                print!("{:016x}: ", index);
            }
            match value {
                MemoryValue::Empty => { print!("{:016} ", "Empty"); },
                MemoryValue::Integral(ref value) => { print!("{:016x} ", value) }
            }
        }
    }

}

