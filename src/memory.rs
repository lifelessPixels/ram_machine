
pub struct Memory {
    storage: Vec<i64>
}

impl Memory {
    pub fn new(size: usize) -> Self {
        let mut new_memory = Memory {
            storage: Vec::<i64>::new()
        };
        new_memory.storage.resize(size, 0);
        new_memory
    }

    pub fn set(&mut self, value: i64, index: usize) {
        if index >= self.storage.len() { return }
        self.storage[index] = value;
    }

    pub fn get(&self, index: usize) -> i64 {
        if index >= self.storage.len() { return 0; }
        self.storage[index].clone()
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        print!("dumping memory: ");
        for (index, value) in self.storage.iter().enumerate() {
            if index % 8 == 0 {
                println!();
                print!("{:016x}: ", index);
            }
            print!("{:016x} ", *value);
        }
    }

}

