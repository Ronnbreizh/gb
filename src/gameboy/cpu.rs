use super::instruction::{Instruction, JumpTest, JumpType};
use super::arithmetictarget::ArithmeticTarget;
use super::registers::Registers;
use super::memory::MemoryBus;

type Delay = u32;
type ProgramCounter = u16;

type CpuEffect = (ProgramCounter, Delay);

pub struct Cpu {
    registers: Registers,
    pc: ProgramCounter,
    sp: u16,
    is_halted: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Self{
            registers: Registers::new(),
            pc: 0 as ProgramCounter,
            sp: 0 as u16,
            is_halted: false,
        }
    }

    pub fn step(&mut self, bus: &mut MemoryBus) {
        // Check if prefixed instruction
        let instruction_byte = bus.read_byte(self.pc);
        let instruction = match instruction_byte {
            // prefetched
            0xCB => {
                let instruction_byte = bus.read_byte(self.pc+1);
                println!("\\\\=> Prefix Ins : {:X}\t", instruction_byte);
                Instruction::from_prefixed_byte(instruction_byte)
            },
            _ => Instruction::from_byte(instruction_byte),
        }.expect(&format!("Unknown instruction : 0x{:x}", instruction_byte));

        println!("Instruction : {:4x}\t{}\t Pc : {:x}", instruction_byte, instruction.to_string(), self.pc);
        
        let (new_pc, _delay) = self.execute(instruction, bus);
        self.pc = new_pc;
    }
    
    fn execute(&mut self, instruction: Instruction, bus: &mut MemoryBus) -> CpuEffect {
        match instruction {
            Instruction::Adc(target) => self.adc(&target, bus),
            Instruction::Add(target) => self.add(&target, bus),
            Instruction::AddHL(target) => self.sub(&target, bus),
            Instruction::And(target) => self.and(&target, bus),
            Instruction::Bit(target, byte) => self.bit(&target, byte, bus),
            Instruction::Ccf(target) => unimplemented!(),
            Instruction::Cp(target) => unimplemented!(),
            Instruction::Cpl(target) => unimplemented!(),
            Instruction::Dec(target) => self.dec(&target, bus),
            Instruction::Inc(target) => self.inc(&target, bus),
            Instruction::Or(target) => self.or(&target, bus),
            Instruction::Res(target, byte) => self.reset(&target, byte, bus),
            Instruction::Rla(target) => unimplemented!(),
            Instruction::Rlc(target) => unimplemented!(),
            Instruction::Rr(target) => unimplemented!(),
            Instruction::Rl(target) => self.rl(&target, bus),
            Instruction::Rra(target) => unimplemented!(),
            Instruction::Rrc(target) => unimplemented!(),
            Instruction::Rrca(target) => unimplemented!(),
            Instruction::Rrla(target) => unimplemented!(),
            Instruction::Sbc(target) => self.sbc(&target, bus),
            Instruction::Scf(target) => unimplemented!(),
            Instruction::Set(target, byte) => self.set(&target, byte, bus),
            Instruction::Sla(target) => unimplemented!(),
            Instruction::Sra(target) => unimplemented!(),
            Instruction::Srl(target) => unimplemented!(),
            Instruction::Sub(target) => self.sub(&target, bus),
            Instruction::Swap(target) => unimplemented!(),
            Instruction::Xor(target) => self.xor(&target, bus),
            Instruction::Jump(test, nature) => self.jump(&test, &nature, bus),
            // TODO check me
            Instruction::Load(target, source) => self.load(&target, &source, bus),
            Instruction::Load8(target) => self.load8(&target, bus),
            Instruction::Load16(target) => self.load16(&target, bus),
            Instruction::LoadH(target, source) => self.loadh(&target, &source, bus),
            Instruction::LoadH8(source) => self.loadh8(&source, bus),
            // Stack and Call
            Instruction::Push(source) => self.push(&source, bus),
            Instruction::Pop(target) => self.pop(&target, bus),
            Instruction::Call(test) => self.call(&test, bus),
            Instruction::Ret(test) => self.ret(&test, bus),

            Instruction::Nop => self.nop(),
            Instruction::Halt => self.halt(),
            _ => unimplemented!("Unknown instruction"),
        }
    }

    fn read_value(&self, target: &ArithmeticTarget, bus: &MemoryBus) -> u8 {
        match target {
            ArithmeticTarget::A => self.registers.a(),
            ArithmeticTarget::B => self.registers.b(),
            ArithmeticTarget::C => self.registers.c(),
            ArithmeticTarget::D => self.registers.d(),
            ArithmeticTarget::E => self.registers.e(),
            ArithmeticTarget::H => self.registers.h(),
            ArithmeticTarget::L => self.registers.l(),
            // Heap
            ArithmeticTarget::DEH => bus.read_byte(self.registers.de()),
            ArithmeticTarget::HLH => bus.read_byte(self.registers.hl()),
            _ => panic!("Want to read 16 bytes register"),
        }
    }

    fn read_value_16(&self, target: &ArithmeticTarget) -> u16 {
        match target {
            ArithmeticTarget::HL => self.registers.hl(),
            ArithmeticTarget::BC => self.registers.bc(),
            ArithmeticTarget::DE => self.registers.de(),
            ArithmeticTarget::AF => self.registers.af(),
            _ => panic!("Want to read 8 bytes register"),
        }
    }

    fn set_value(&mut self, target: &ArithmeticTarget, value: u8, bus: &mut MemoryBus) {
        match target {
            ArithmeticTarget::A => self.registers.set_a(value),
            ArithmeticTarget::B => self.registers.set_b(value),
            ArithmeticTarget::C => self.registers.set_c(value),
            ArithmeticTarget::D => self.registers.set_d(value),
            ArithmeticTarget::E => self.registers.set_e(value),
            ArithmeticTarget::H => self.registers.set_h(value),
            ArithmeticTarget::L => self.registers.set_l(value),
            ArithmeticTarget::HLDec => {
                // write value to area pointed
                let address = self.read_value_16(&ArithmeticTarget::HL);
                bus.write_byte(address, value);
                // Decrement HL register
                let value = self.read_value_16(&ArithmeticTarget::HL) - 1;
                self.set_value_16(&ArithmeticTarget::HL, value);
            },
            _ => panic!("Try to set 16 bytes"),
        }
    }

    fn set_value_16(&mut self, target: &ArithmeticTarget, value: u16) {
        match target {
            ArithmeticTarget::HL => self.registers.set_hl(value),
            ArithmeticTarget::BC => self.registers.set_bc(value),
            ArithmeticTarget::DE => self.registers.set_de(value),
            ArithmeticTarget::AF => self.registers.set_af(value),
            ArithmeticTarget::SP => self.sp = value,
            _ => panic!("Try to set 16 bytes"),
        }
    }

    fn jump(&self, test: &JumpTest, nature: &JumpType, bus: &MemoryBus) -> CpuEffect {
        if test.evaluate(self.registers.f()) {
            // should jump
            match nature {
                JumpType::Relative8 => ((self.pc as u32 as i32 + (bus.read_byte(self.pc+1) as i8 as i32)) as u16 + 2,8),
                JumpType::Relative16 => ((self.pc as u32 as i32 + bus.read_word(self.pc+1) as u32 as i32) as u16 + 3,12),
                _ => unimplemented!("Jump type missing!"),
            }
        } else {
            // just continue
            match nature {
                JumpType::Relative8 => (self.pc + 2,8), 
                JumpType::Relative16 => (self.pc + 3,12),
                _ => unimplemented!("Jump type missing!"),
            }
        }
    }

    fn halt(&mut self) -> CpuEffect {
        self.is_halted = true;
        (self.pc + 1, 4)
    }

    fn load(&mut self,target: &ArithmeticTarget, source: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let (offset, delay) = match target {
            ArithmeticTarget::A => {
                let value = match source {
                    ArithmeticTarget::A| ArithmeticTarget::C => self.read_value(source, bus),
                    ArithmeticTarget::DEH => self.read_value(&ArithmeticTarget::DEH, bus),
                    _=> unimplemented!("Load"),
                };
                self.set_value(&ArithmeticTarget::A, value, bus);
                (1,4)
            },
            ArithmeticTarget::B => {
                let value = match source {
                    ArithmeticTarget::A => self.read_value(&ArithmeticTarget::A, bus),
                    _=> unimplemented!(),
                };
                self.set_value(&ArithmeticTarget::B, value, bus);
                (1,4)
            },
            ArithmeticTarget::C => {
                let value = match source {
                    ArithmeticTarget::A => self.read_value(&ArithmeticTarget::A, bus),
                    _=> unimplemented!(),
                };
                self.set_value(&ArithmeticTarget::C, value, bus);
                (1,4)
            },
            ArithmeticTarget::H => {
                let value = match source {
                    ArithmeticTarget::A => self.read_value(&ArithmeticTarget::A, bus),
                    _=> unimplemented!(),
                };
                self.set_value(&ArithmeticTarget::H, value, bus);
                (1,4)
            },
            ArithmeticTarget::HLDec => {
                // retrieve value
                let value = match source {
                    ArithmeticTarget::A => self.read_value(&ArithmeticTarget::A, bus),
                    _ => unimplemented!(),
                };
                let address = self.read_value_16(&ArithmeticTarget::HL);
                // write to memory bus
                bus.write_byte(address, value);

                let new_address = address.wrapping_sub(1);
                // decrement HL
                self.set_value_16(
                    &ArithmeticTarget::HL,
                    new_address,
                );
                (1,4)
            },
            ArithmeticTarget::HLH => {
                // retrieve value
                let value = match source {
                    ArithmeticTarget::A => self.read_value(&ArithmeticTarget::A, bus),
                    _ => unimplemented!(),
                };
                let address = self.read_value_16(&ArithmeticTarget::HL);
                // write to memory bus
                bus.write_byte(address, value);

                (1,4)
            }
            _ => unimplemented!("[Load] Uninmplemented target register"),
        };

        (self.pc + offset, delay)
    }
    
    /// load8
    fn load8(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let value = bus.read_byte(self.pc+1);

        self.set_value(target, value, bus);

        (self.pc + 2, 8)
    }

    /// load16
    fn load16(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let value = bus.read_word(self.pc+1);

        self.set_value_16(target, value);

        (self.pc + 3, 12)
    }

    /// loadh
    /// write to xFF00 + target value
    fn loadh(&mut self, target: &ArithmeticTarget, source: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        // offset from register target
        let offset = self.read_value(target, bus);

        let address = 0xFF00 | (offset as u16);

        // value from source
        let value = self.read_value(source, bus);

        bus.write_byte(address, value);

        (self.pc + 1, 12)
    }

    /// loadh8
    /// write to xFF00 + next byte
    fn loadh8(&mut self, source: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        // offset from register target
        let offset = bus.read_byte(self.pc + 1);

        let address = 0xFF00 | (offset as u16);

        // value from source
        let value = self.read_value(source, bus);

        bus.write_byte(address, value);

        (self.pc + 2, 12)
    }

    /// no operation
    fn nop(&mut self) -> CpuEffect {
        (self.pc + 1, 1)
    }

    fn add(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let value = self.read_value_16(target);
        let (new_value, did_overflow) = self.registers.hl().overflowing_add(value);

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_carry(did_overflow);

        let register_hl = self.registers.hl();

        self.registers.f_as_mut().set_half_carry((register_hl & 0x07FF) + (value & 0x07FF) > 0x07FF);

        self.registers.set_hl(new_value);

        (self.pc + 1,8)
    }

    /// Add for u16 i.e. larger registers
    fn add_16(&mut self, target:&ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let value = self.read_value_16(target);

        let (new_value, did_overflow) = self.registers.hl().overflowing_add(value);

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_carry(did_overflow);
        self.registers.f_as_mut().set_zero(new_value == 0);

        self.registers.set_hl(new_value);

        (self.pc + 1, 4)
    }

    fn adc(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let value = self.read_value(target, bus);
        // if no overflow, value can overflow
        let (mut new_value, mut did_overflow) = self.registers.a().overflowing_add(value);

        if self.registers.f().carry(){
            let (new_value_carry, did_overflow_carry) = new_value.overflowing_add(1);
            new_value = new_value_carry;
            did_overflow |= did_overflow_carry;
        }

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_carry(did_overflow);
        self.registers.f_as_mut().set_zero(new_value == 0);
        let register_a = self.registers.a();
        self.registers.f_as_mut().set_half_carry((register_a & 0xF) + (value & 0xF) > 0xF);

        self.registers.set_a(new_value);

        (self.pc + 1,4)
    }

    fn sub(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let value = self.read_value(target, bus);
        let (mut new_value, did_overflow) = self.registers.a()
            .overflowing_sub(value);
        if self.registers.f().carry() {
            new_value = new_value.wrapping_sub(1);
        }

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(true);
        self.registers.f_as_mut().set_carry(did_overflow);
        self.registers.f_as_mut().set_zero(new_value == 0);

        let register_a = self.registers.a();
        self.registers.f_as_mut().set_half_carry((register_a & 0xF) < (value & 0xF));

        self.registers.set_a(new_value);

        (self.pc + 1, 4)
    }

    fn sbc(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let value = self.read_value(target, bus);

        let (mut new_value, mut did_overflow) = self.registers.a()
            .overflowing_sub(value);

        if self.registers.f().carry(){
            new_value = new_value.wrapping_sub(1);  
        }

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(true);
        self.registers.f_as_mut().set_carry(did_overflow);

        let register_a = self.registers.a();
        self.registers.f_as_mut().set_half_carry((register_a & 0xF) < (value & 0xF));

        self.registers.set_a(new_value);

        (self.pc + 1, 4)
    }

    fn and(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let value = self.read_value(target, bus);
        let new_value = self.registers.a() & value;

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_half_carry(false);
        self.registers.f_as_mut().set_carry(false);
        (self.pc + 1, 4)
    }

    fn xor(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let value = self.read_value(target, bus);
        let new_value = self.registers.a() ^ value;

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_half_carry(false);
        self.registers.f_as_mut().set_carry(false);

        self.set_value(target, new_value, bus);

        (self.pc + 1, 4)
    }

    fn or(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let value = self.read_value(target, bus);
        let new_value = self.registers.a() | value;

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_half_carry(false);
        self.registers.f_as_mut().set_carry(false);
        (self.pc + 1, 4)
    }

    fn cp(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let value = self.read_value(target, bus);
        let (new_value, did_overflow) = self.registers.a().overflowing_sub(value);

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(true);
        self.registers.f_as_mut().set_carry(did_overflow);
        self.registers.f_as_mut().set_zero(new_value == 0);
        let register_a = self.registers.a();
        self.registers.f_as_mut().set_half_carry((register_a & 0xF) + (value & 0xF) > 0xF);

        (self.pc + 1, 4)
    }

    fn sla(&mut self, target: &ArithmeticTarget) -> CpuEffect {
        unimplemented!();
        (self.pc + 1, 4)
    }

    fn sra(&mut self, target: &ArithmeticTarget) -> CpuEffect {
        unimplemented!();
        (self.pc + 1, 4)
    }
    
    // rotate left
    fn rl(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let value = self.read_value(target, bus);
    
        // check first bit
        let carry = (value & 0x80) == 0x80;

        let new_value = value << 1 | (if self.registers.f().carry() {1} else {0});

        self.registers.f_as_mut().set_carry(carry);

        self.set_value(target, new_value, bus);

        (self.pc + 1, 8)
    }

    fn inc(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let value = self.read_value(&target, bus);
        let (new_value, did_overflow) = value.overflowing_add(1);

        let register_a = self.registers.a();
        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_half_carry((register_a & 0xF) + (value & 0xF) > 0xF);
        self.registers.f_as_mut().set_carry(did_overflow);

        self.set_value(&target, new_value, bus);

        //TODO : case 16 registers
        (self.pc + 1, 4)
    }

    fn dec(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let value = self.read_value(&target, bus);
        let new_value = value.wrapping_sub(1);

        let register_a = self.registers.a();
        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(true);
        self.registers.f_as_mut().set_half_carry((register_a & 0xF) == 0);

        self.set_value(&target, new_value, bus);

        // TODO
        (self.pc + 1, 4)
    }

    /// set register bit at bit position to 1
    fn set(&mut self, target: &ArithmeticTarget, bit_pos: u8, bus: &mut MemoryBus) -> CpuEffect {
        let value = self.read_value(target, bus);
        let new_value = value | 1u8<<bit_pos;

        self.set_value(target, new_value, bus);

        (self.pc + 1, 8)
    }

    /// reset register bit at bit position to 0
    fn reset(&mut self, target: &ArithmeticTarget, bit_pos: u8, bus: &mut MemoryBus) -> CpuEffect {
        let value = self.read_value(target, bus);
        let new_value = value & (0xFF ^ (1<<bit_pos));

        self.set_value(target, new_value, bus);

        (self.pc + 1, 8)
    }

    /// bit value
    fn bit(&mut self, target: &ArithmeticTarget, bit_pos: u8, bus: &mut MemoryBus) -> CpuEffect {
        let value = self.read_value(target, bus);

        self.registers.f_as_mut().set_zero(value & (1<<bit_pos) == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_half_carry(true);

        //TODO 12 sometimes
        (self.pc + 2, 8)
    }

    /// swap
    fn swap(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let value = self.read_value(target, bus);
        let new_value = (value << 4) | (value >> 4);
        self.set_value(target, new_value, bus);

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_half_carry(false);
        self.registers.f_as_mut().set_carry(false);


        (self.pc + 1, 8)
    }

    /// Push 2 bytes to stack
    fn push(&mut self, source: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        // read value from register
        let value = self.read_value_16(source);

        self.push_word(value, bus);

        (self.pc + 1, 16)
    }

    fn push_word(&mut self, value: u16, bus: &mut MemoryBus) {
        // decrese stack
        self.sp = self.sp.wrapping_sub(1);
        
        // write most significant part first
        bus.write_byte(self.sp, (value>>8) as u8);

        // decrese stack
        self.sp = self.sp.wrapping_sub(1);

        // write least significant part then 
        bus.write_byte(self.sp, (value & 0xFF) as u8)
    }

    /// Pop 2 bytes from the stack
    fn pop(&mut self, target: &ArithmeticTarget, bus:&MemoryBus) -> CpuEffect {

        let value = self.pop_word(bus); 

        self.set_value_16(target, value);

        (self.pc + 1, 12)
    }

    /// Pop word from the stack and return its value as u16
    fn pop_word(&mut self, bus: &MemoryBus) -> u16 {
        let lower_part = bus.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(1);

        let higher_part = bus.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(1);

        ((higher_part as u16) << 8) | (lower_part as u16)
    }

    /// Call function
    fn call(&mut self, test : &JumpTest, bus: &mut MemoryBus) -> CpuEffect {
        let next_pc = self.pc + 3;

        if test.evaluate(self.registers.f()) {
            self.push_word(next_pc, bus);
            // new pc is in the following bytes
            (bus.read_word(self.pc + 1), 12)
        } else {
            (next_pc, 12)
        }
    }

    /// Return from function
    fn ret(&mut self, test: &JumpTest, bus: &MemoryBus) -> CpuEffect {
        
        if test.evaluate(self.registers.f()) {
            let address = self.pop_word(bus);

            // CHEKME
            return (address,16)
        }
        // else juste skip?

        //TODO
        (self.pc + 1, 20)

    }
}

#[cfg(test)]
mod test_8bit {
    use crate::gameboy::{arithmetictarget::ArithmeticTarget, instruction::Instruction, memory::MemoryBus};

    use super::Cpu;
    #[test]
    fn add() {
        let mut cpu = Cpu::new();
        let mut memory_bus = MemoryBus::default();
        cpu.registers.set_a(1);
        cpu.registers.set_b(2);
        let instruction = Instruction::Add(ArithmeticTarget::B);
        cpu.execute(instruction, &mut memory_bus);
    
        assert_eq!(cpu.registers.a(), 3)
    }    

    #[test]
    fn adc() {
        todo!()
    }

    #[test]
    fn sub() {
        todo!()
    }

    #[test]
    fn sbc() {
        todo!()
    }

    #[test]
    fn and() {
        todo!()
    }

    #[test]
    fn xor() {
        todo!()
    }

    #[test]
    fn or() {
        todo!()
    }

    #[test]
    fn cp() {
        todo!()
    }

    #[test]
    fn inc() {
        todo!()
    }

    #[test]
    fn dec() {
        todo!()
    }

    #[test]
    fn daa() {
        todo!()
    }

    #[test]
    fn cpl() {
        todo!()
    }
}

#[cfg(test)]
mod test_16bits {
    #[test]
    fn add_hl() {
        todo!()
    }

    #[test]
    fn inc() {
        todo!()
    }

    #[test]
    fn dec() {
        todo!()
    }

    #[test]
    fn add_sp() {
        todo!()
    }

    #[test]
    fn ld() {
        todo!()
    }
}

#[cfg(test)]
mod test_rotate_shift {
    #[test]
    fn rlca() {
        todo!()
    }

    #[test]
    fn rla() {
        todo!()
    }

    #[test]
    fn rrca() {
        todo!()
    }

    #[test]
    fn rra() {
        todo!()
    }

    #[test]
    fn rlc() {
        todo!()
    }

    #[test]
    fn rl() {
        todo!()
    }

    #[test]
    fn rrc() {
        todo!()
    }

    #[test]
    fn rr() {
        todo!()
    }

    #[test]
    fn sla() {
        todo!()
    }

    #[test]
    fn swap() {
        todo!()
    }

    #[test]
    fn sra() {
        todo!()
    }

    #[test]
    fn srl() {
        todo!()
    }
}

#[cfg(test)]
mod test_1bit {
    #[test]
    fn bit() {
        todo!()
    }

    #[test]
    fn set() {
        todo!()
    }

    #[test]
    fn rst() {
        todo!()
    }

}

#[cfg(test)]
mod test_cpu_control {
    #[test]
    fn ccf() {
        todo!()
    }

    #[test]
    fn scf() {
        todo!()
    }

    #[test]
    fn nop() {
        todo!()
    }

    #[test]
    fn halt() {
        todo!()
    }

    #[test]
    fn stop() {
        todo!()
    }

    #[test]
    fn di() {
        todo!()
    }

    #[test]
    fn ei() {
        todo!()
    }
}

#[cfg(test)]
mod test_jump {
    #[test]
    fn jp_nn() {
        todo!()
    }

    #[test]
    fn jp_hl() {
        todo!()
    }

    #[test]
    fn jp_conditionnal() {
        todo!()
    }

    #[test]
    fn jp_relative() {
        todo!()
    }

    #[test]
    fn jp_relative_conditionnal() {
        todo!()
    }

    #[test]
    fn call_nn() {
        todo!()
    }

    #[test]
    fn call_conditionnal() {
        todo!()
    }

    #[test]
    fn ret() {
        todo!()
    }

    #[test]
    fn ret_conditionnal() {
        todo!()
    }

    #[test]
    fn ret_interrupt() {
        todo!()
    }

    #[test]
    fn rst() {
        todo!()
    }
}