/*
    CHIP-8 CPU Implementation 
*/

// definintion of the CPU 
struct CPU {
    registers: [u8; 16], 
    pc: usize, // program counter 
    memory: [u8; 4096], 
    stack: [u16; 16], 
    stack_pointer: usize, 
}

impl CPU {
    // reading opcode from memory 
    fn read_opcode(&self) -> u16 {
        let p = self.pc; 
        let op_byte1 = self.memory[p] as u16; 
        let op_byte2 = self.memory[p + 1] as u16; 

        op_byte1 << 8 | op_byte2 
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode(); 
            self.pc += 2; 

            // decoding opcode in form of 0xCXYD 
            let c = ((opcode & 0xF000) >> 12) as u8; // opcode group 
            let x = ((opcode & 0x0F00) >>  8) as u8; // CPU register 
            let y = ((opcode & 0x00F0) >>  4) as u8; // CPU register 
            let d = ((opcode & 0x000F)      ) as u8; // opcode subgroup 

            let nnn = (opcode & 0x0FFF); // memory address 

            // dispatches execution 
            match (c, x, y, d) {
                (0, 0, 0, 0)        => { return; }, // terminate execution 
                (0, 0, 0xE, 0xE)    => self.ret(), 
                (0x2, _, _, _)      => self.call(nnn), 
                (0x8, _, _, 0x4)    => self.add_xy(x, y), 
                _                   => todo! ("opcode {:04x}", opcode), 
            }
        }
    }

    // add two numbers at register x and y   
    fn add_xy(&mut self, x: u8, y:u8) {
        let vx = self.registers[x as usize]; 
        let vy = self.registers[y as usize]; 

        let (val, overflow) = vx.overflowing_add(vy); 
        self.registers[x as usize] = val; 

        // handle overflow 
        if overflow {
            // last register as a carry flag 
            self.registers[0xF] = 1; 
        } else {
            self.registers[0xF] = 0; 
        }
    }

    // calling memory location  at addr 
    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer; 
        let stack = &mut self.stack; 

        if sp > stack.len() {
            panic!("Stack overflow!") 
        }

        stack[sp] = self.pc as u16; // current position 
        self.stack_pointer += 1;    // increments stack pointer 
        self.pc = addr as usize; 
    }

    // retruning from a function 
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow"); 
        }

        self.stack_pointer -= 1; // to earlier position 
        let call_addr = self.stack[self.stack_pointer]; 
        self.pc = call_addr as usize; 
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16], 
        memory: [0; 4096], 
        pc: 0, 
        stack: [0; 16], 
        stack_pointer: 0, 
    }; 

    cpu.registers[0] = 1;   // x
    cpu.registers[1] = 2;   // y 

    let mem = &mut cpu.memory; 
    mem[0x000] = 0x21; mem[0x001] = 0x00; // opcode 0x2100: call 0x100 
    mem[0x002] = 0x21; mem[0x003] = 0x00; // opcode 0x2100
    mem[0x004] = 0x00; mem[0x005] = 0x00; // opcode 0x0000: halt   

    mem[0x100] = 0x80; mem[0x101] = 0x14; // opcode 0x8014: r0+r1  
    mem[0x102] = 0x80; mem[0x103] = 0x14; // opcode 0x8014 
    mem[0x104] = 0x00; mem[0x105] = 0xEE; // opcode 0x00EE: ret  

    cpu.run(); 
}
