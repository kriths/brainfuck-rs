pub const MINIMUM_MEMORY_SIZE: usize = 30_000;

pub struct CPU {
    code: Vec<u8>,
    ip: usize,

    memory: Vec<u8>,
    dp: usize,
}

impl CPU {
    fn read_data(&self) -> u8 {
        self.memory[self.dp]
    }

    pub fn run(mut self) {
        loop {
            if self.ip == self.code.len() {
                // Program finished
                return;
            }

            if self.ip > self.code.len() {
                unreachable!("IP should never be able to exceed code bounds");
            }

            let instruction = self.code[self.ip];
            match instruction {
                b'[' => {
                    if self.read_data() == 0 {
                        // Forward IP to closing paren
                        let mut nesting_level: usize = 0;
                        for i in self.ip + 1..self.code.len() {
                            match (self.code[i], nesting_level) {
                                (b'[', _) => { nesting_level += 1 }
                                (b']', 0) => {
                                    self.ip = i + 1;
                                    break;
                                }
                                (b']', _) => { nesting_level -= 1 }
                                _ => {}
                            }
                        }
                    } else {
                        self.ip += 1;
                    }
                }
                b']' => {
                    if self.read_data() == 0 {
                        self.ip += 1;
                    } else {
                        // Walk backwards to matching opening paren
                        let mut nesting_level: usize = 0;
                        let mut i: usize = self.ip - 1;
                        loop {
                            match (self.code[i], nesting_level) {
                                (b'[', 0) => {
                                    self.ip = i + 1;
                                    break;
                                }
                                (b'[', _) => { nesting_level -= 1 }
                                (b']', _) => { nesting_level += 1 }
                                _ => {}
                            }
                            i -= 1;
                        }
                    }
                }
                b'+' => {
                    self.memory[self.dp] = self.read_data().wrapping_add(1);
                    self.ip += 1;
                }
                b'-' => {
                    self.memory[self.dp] = self.read_data().wrapping_sub(1);
                    self.ip += 1;
                }
                b'>' => {
                    self.dp += 1;
                    self.ip += 1;
                    if self.dp >= self.memory.len() {
                        panic!("Data pointer exceeded memory range");
                    }
                }
                b'<' => {
                    if self.dp == 0 {
                        panic!("Tried to access memory at DP < 0");
                    }
                    self.dp -= 1;
                    self.ip += 1;
                }
                b'.' => {
                    print!("{}", self.memory[self.dp] as char);
                    self.ip += 1;
                }
                b',' => { todo!("input not implemented") }
                _ => unreachable!("Invalid BF source character {}", instruction)
            }
        }
    }

    pub fn new(memory_size: usize, code: Vec<u8>) -> Self {
        assert!(memory_size >= MINIMUM_MEMORY_SIZE);
        Self {
            code,
            ip: 0,

            memory: vec![0; memory_size],
            dp: 0,
        }
    }
}

