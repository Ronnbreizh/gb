use super::arithmetictarget::{ArithmeticTarget, WideArithmeticTarget};
use super::instruction::{Instruction, JumpTest, JumpType};
use super::memory::MemoryBus;
use super::registers::Registers;
use std::ops::Not;

type Delay = u32;
type ProgramCounter = u16;

/// Custom type to split the ProgramCounter and the time offset / number of cycle of the CPU
type CpuEffect = (ProgramCounter, Delay);

pub struct Cpu {
    registers: Registers,
    pc: ProgramCounter,
    sp: u16,
    is_halted: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            pc: 0 as ProgramCounter,
            sp: 0u16,
            is_halted: false,
        }
    }

    pub fn step(&mut self, bus: &mut MemoryBus) {
        // Check if prefixed instruction
        let instruction_byte = bus.read_byte(self.pc);
        let instruction = match instruction_byte {
            // prefetched
            0xCB => {
                self.pc += 1;
                let instruction_byte = bus.read_byte(self.pc);
                Instruction::from_prefixed_byte(instruction_byte)
            }
            _ => Instruction::from_byte(instruction_byte),
        }
        .unwrap_or_else(|| panic!("Unknown instruction : 0x{:x}", instruction_byte));

        println!(
            "|0x{:2x}|{:24}|Pc:0x{:04x}|",
            instruction_byte,
            instruction.to_string(),
            self.pc
        );

        let (new_pc, _delay) = self.execute(instruction, bus);
        self.pc = new_pc;
    }

    fn execute(&mut self, instruction: Instruction, bus: &mut MemoryBus) -> CpuEffect {
        match instruction {
            Instruction::Adc(target) => self.adc(&target, bus),
            Instruction::Add(target) => self.add(&target, bus),
            // CHECK special treamtment here
            Instruction::AddHL(_target) => todo!(),
            Instruction::And(target) => self.and(&target, bus),
            Instruction::Bit(target, byte) => self.bit(&target, byte, bus),
            Instruction::Ccf => self.ccf(),
            Instruction::Cp(target) => self.cp(&target, bus),
            Instruction::Cpl => self.cpl(),
            Instruction::Dec(target) => self.dec(&target, bus),
            Instruction::Inc(target) => self.inc(&target, bus),
            Instruction::Or(target) => self.or(&target, bus),
            Instruction::Res(target, byte) => self.reset(&target, byte, bus),
            Instruction::Rla => self.rla(),
            Instruction::Rlc(target) => self.rlc(&target, bus),
            Instruction::Rr(target) => self.rr(&target, bus),
            Instruction::Rl(target) => self.rl(&target, bus),
            Instruction::Rra => unimplemented!(),
            Instruction::Rrc(target) => self.rrc(&target, bus),
            Instruction::Rrca => self.rrca(),
            Instruction::Rlca => self.rlca(),
            Instruction::Sbc(target) => self.sbc(&target, bus),
            Instruction::Scf => unimplemented!(),
            Instruction::Set(target, byte) => self.set(&target, byte, bus),
            Instruction::Sla(target) => self.sla(&target, bus),
            Instruction::Sra(target) => self.sra(&target, bus),
            Instruction::Srl(target) => self.srl(&target, bus),
            Instruction::Sub(target) => self.sub(&target, bus),
            Instruction::Swap(target) => self.swap(&target, bus),
            Instruction::Xor(target) => self.xor(&target, bus),
            // JUMP
            Instruction::Jump(test, nature) => self.jump(&test, &nature, bus),
            // TODO check me
            Instruction::Load {
                to: target,
                from: source,
            } => self.load(&target, &source, bus),
            Instruction::Load16{to, from} => self.load16(&to, &from, bus),
            // Stack and Call
            Instruction::Push(source) => self.push(&source, bus),
            Instruction::Pop(target) => self.pop(&target, bus),
            Instruction::Call(test) => self.call(&test, bus),
            Instruction::Ret(test) => self.ret(&test, bus),

            Instruction::Nop => self.nop(),
            Instruction::Halt => self.halt(),
        }
    }

    fn read_value(&self, target: &ArithmeticTarget, bus: &MemoryBus) -> (u8, ProgramCounter,Delay) {
        match target {
            ArithmeticTarget::A => (self.registers.a(), 0, 0),
            ArithmeticTarget::B => (self.registers.b(), 0, 0),
            ArithmeticTarget::C => (self.registers.c(), 0, 0),
            ArithmeticTarget::D => (self.registers.d(), 0, 0),
            ArithmeticTarget::E => (self.registers.e(), 0, 0),
            ArithmeticTarget::H => (self.registers.h(), 0, 0),
            ArithmeticTarget::L => (self.registers.l(), 0, 0),
            // Memory
            ArithmeticTarget::BCTarget => (bus.read_byte(self.registers.bc()), 0, 4),
            ArithmeticTarget::DETarget => (bus.read_byte(self.registers.de()), 0, 4),
            ArithmeticTarget::HLTarget => (bus.read_byte(self.registers.hl()), 0, 4),
            // PC
            ArithmeticTarget::FFC => (bus.read_byte(0xFF00 + self.registers.c() as u16), 0, 4),
            ArithmeticTarget::FFRead => (bus.read_byte(0xFF00 + bus.read_byte(self.pc+1)as u16), 1, 4),
            ArithmeticTarget::ReadByte => (bus.read_byte(self.pc+1), 1, 4),
            // OTHER
            ArithmeticTarget::HLInc => todo!(),
            ArithmeticTarget::HLDec => todo!(),
        }
    }

    fn read_value_16(&self, target: &WideArithmeticTarget, bus: &MemoryBus) -> (u16, ProgramCounter, Delay) {
        match target {
            WideArithmeticTarget::HL => (self.registers.hl(),0, 0),
            WideArithmeticTarget::BC => (self.registers.bc(), 0, 0),
            WideArithmeticTarget::DE => (self.registers.de(), 0, 0),
            WideArithmeticTarget::AF => (self.registers.af(), 0, 0),
            WideArithmeticTarget::SP => todo!("Read Stack"),
            WideArithmeticTarget::ReadWord => (bus.read_word(self.pc+1), 2, 4),
        }
    }

    /// The the provided value in the right arithmetic target.
    /// The offset is either 4 or 8
    fn write_value(&mut self, target: &ArithmeticTarget, value: u8, bus: &mut MemoryBus) -> Delay {
        match target {
            ArithmeticTarget::A => {self.registers.set_a(value); 0},
            ArithmeticTarget::B => {self.registers.set_b(value); 0},
            ArithmeticTarget::C => {self.registers.set_c(value); 0},
            ArithmeticTarget::D => {self.registers.set_d(value); 0},
            ArithmeticTarget::E => {self.registers.set_e(value); 0},
            ArithmeticTarget::H => {self.registers.set_h(value); 0},
            ArithmeticTarget::L => {self.registers.set_l(value); 0},
            // target type
            ArithmeticTarget::BCTarget => {
                let address = self.registers.bc();
                bus.write_byte(address, value);
                4
            },
            ArithmeticTarget::DETarget => {
                let address = self.registers.de();
                bus.write_byte(address, value);
                4
            },
            ArithmeticTarget::HLTarget => {
                let address = self.registers.bc();
                bus.write_byte(address, value);
                4
            },
            ArithmeticTarget::FFRead => {
                let offset = bus.read_byte(self.pc + 1);
                let address = 0xFF00 + (offset as u16);
                bus.write_byte(address, value);
                4
            },
            ArithmeticTarget::FFC => {
                let address = 0xFF00 + (self.registers.c() as u16);
                bus.write_byte(address, value);
                4
            },
            ArithmeticTarget::ReadByte => unreachable!("Can't right directly to next byte."),
            // SPECIAL
            ArithmeticTarget::HLDec => {
                let address = self.registers.hl();
                bus.write_byte(address, value);
                self.registers.set_hl(address - 1);
                4
            },
            ArithmeticTarget::HLInc => {
                todo!()
            },
        }
    }

    fn write_value_16(&mut self, target: &WideArithmeticTarget, value: u16)  -> Delay{
        match target {
            WideArithmeticTarget::HL => {self.registers.set_hl(value); 0},
            WideArithmeticTarget::BC => {self.registers.set_bc(value); 0},
            WideArithmeticTarget::DE => {self.registers.set_de(value); 0},
            WideArithmeticTarget::AF => {self.registers.set_af(value); 0},
            // CHECKME  0 right offset ?
            WideArithmeticTarget::SP => {self.sp = value; 0}
            WideArithmeticTarget::ReadWord => panic!("Can't right directly to the next bytes"),
        }
    }

    fn jump(&self, test: &JumpTest, nature: &JumpType, bus: &MemoryBus) -> CpuEffect {
        if test.evaluate(self.registers.f()) {
            // should jump
            match nature {
                JumpType::Relative8 => (
                    (self.pc as u32 as i32 + (bus.read_byte(self.pc + 1) as i8 as i32)) as u16 + 2,
                    8,
                ),
                JumpType::Pointer16 => (bus.read_word(self.pc + 1), 12),
                _ => unimplemented!("Jump type missing!"),
            }
        } else {
            // just continue and skip the trailing data
            match nature {
                JumpType::Relative8 => (self.pc + 2, 8),
                JumpType::Pointer16 => (self.pc + 3, 12),
                _ => unimplemented!("Jump type missing!"),
            }
        }
    }

    fn halt(&mut self) -> CpuEffect {
        self.is_halted = true;
        (self.pc + 1, 4)
    }

    fn load(
        &mut self,
        target: &ArithmeticTarget,
        source: &ArithmeticTarget,
        bus: &mut MemoryBus,
    ) -> CpuEffect {
        let (value, pc_offset, source_offset) = self.read_value(source, bus);

        let target_offset = self.write_value(target, value, bus);
        (self.pc+1+pc_offset, source_offset+target_offset)
    }

    /// Load next 2 bytes in memory in the provided registers
    /// REMOVE ME
    fn load16(&mut self, target: &WideArithmeticTarget, source: &WideArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let (value, pc_offset, read_offset) = self.read_value_16(source, bus);

        // CHECKME : can we write a u16 in other thing than a register ?
        let write_offset = self.write_value_16(target, value);
        (self.pc+1+pc_offset, 4 + read_offset + write_offset)
    }

    /// no operation
    fn nop(&mut self) -> CpuEffect {
        (self.pc + 1, 1)
    }

    /// Complement carry flag
    fn ccf(&mut self) -> CpuEffect {
        let carry = self.registers.f().carry();
        self.registers.f_as_mut().set_carry(carry.not());
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_half_carry(false);
        (self.pc + 1, 4)
    }

    /// Add the content of the targeted register to the A register.
    fn add(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let (value, pc_offset, offset) = self.read_value(target, bus);
        let (new_value, did_overflow) = self.registers.a().overflowing_add(value);

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_carry(did_overflow);

        let register_a = self.registers.a();
        self.registers.set_a(new_value);
        self.registers
            .f_as_mut()
            .set_half_carry((register_a & 0xF) + (value & 0xF) > 0xF);

        (self.pc + 1+pc_offset, offset)
    }

    /// Add with carry
    fn adc(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let (value, pc_offset, offset) = self.read_value(target, bus);
        // if no overflow, value can overflow
        let (mut new_value, mut did_overflow) = self.registers.a().overflowing_add(value);

        if self.registers.f().carry() {
            let (new_value_carry, did_overflow_carry) = new_value.overflowing_add(1);
            new_value = new_value_carry;
            did_overflow |= did_overflow_carry;
        }

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_carry(did_overflow);

        let register_a = self.registers.a();
        self.registers
            .f_as_mut()
            .set_half_carry((register_a & 0xF) + (value & 0xF) > 0xF);

        self.registers.set_a(new_value);

        (self.pc + 1 + pc_offset, 4+offset)
    }
    /// Subscrate the target value to the A register.
    fn sub(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let (value, pc_offset, offset) = self.read_value(target, bus);
        let (new_value, did_overflow) = self.registers.a().overflowing_sub(value);

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(true);
        self.registers.f_as_mut().set_carry(did_overflow);

        let register_a = self.registers.a();
        self.registers
            .f_as_mut()
            .set_half_carry((register_a & 0xF) < (value & 0xF));

        self.registers.set_a(new_value);

        (self.pc + 1 + pc_offset, 4+offset)
    }

    /// Like sub but the carry value is also substracted
    fn sbc(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let (value, pc_offset, offset) = self.read_value(target, bus);

        let (mut new_value, did_overflow) = self.registers.a().overflowing_sub(value);

        if self.registers.f().carry() {
            new_value = new_value.wrapping_sub(1);
        }

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(true);
        self.registers.f_as_mut().set_carry(did_overflow);

        let register_a = self.registers.a();
        self.registers
            .f_as_mut()
            .set_half_carry((register_a & 0xF) < (value & 0xF));

        self.registers.set_a(new_value);

        (self.pc + 1 + pc_offset, 4+offset)
    }

    fn and(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let (value, pc_offset, offset) = self.read_value(target, bus);
        let new_value = self.registers.a() & value;

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_half_carry(false);
        self.registers.f_as_mut().set_carry(false);

        self.registers.set_a(new_value);

        (self.pc + 1 + pc_offset, 4+offset)
    }

    fn xor(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let (value, pc_offset, offset) = self.read_value(target, bus);
        let new_value = self.registers.a() ^ value;

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_half_carry(false);
        self.registers.f_as_mut().set_carry(false);

        self.registers.set_a(new_value);

        (self.pc + 1 + pc_offset, 4+offset)
    }

    fn or(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let (value, pc_offset, offset) = self.read_value(target, bus);
        let new_value = self.registers.a() | value;

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_half_carry(false);
        self.registers.f_as_mut().set_carry(false);

        self.registers.set_a(new_value);
        (self.pc + 1 + pc_offset, 4+offset)
    }

    fn cp(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let (value, pc_offset, offset) = self.read_value(target, bus);
        let (new_value, did_overflow) = self.registers.a().overflowing_sub(value);

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(true);
        self.registers.f_as_mut().set_carry(did_overflow);

        let register_a = self.registers.a();
        self.registers
            .f_as_mut()
            .set_half_carry((register_a & 0xF) + (value & 0xF) > 0xF);

        (self.pc + 1 + pc_offset, 4+offset)
    }

    /// Shift left arithmetic. Multiplies by 2
    fn sla(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let (value, pc_offset, source_offset) = self.read_value(target, bus);
        let (new_value, did_overflow) = value.overflowing_mul(2);

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_half_carry(false);
        self.registers.f_as_mut().set_carry(did_overflow);

        let target_offset = self.write_value(target, new_value, bus);

        (self.pc + 1 + pc_offset, 8+ target_offset + source_offset)
    }

    /// Shift right arithmetic. Divides by 2
    fn sra(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let (value, pc_offset, read_offset) = self.read_value(target, bus);
        let (new_value, did_overflow) = value.overflowing_div(2);

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_half_carry(false);
        self.registers.f_as_mut().set_carry(did_overflow);

        let write_offset = self.write_value(target, new_value, bus);

        (self.pc + 1 + pc_offset, 4 + read_offset + write_offset)
    }

    /// Bit shift right
    fn srl(&mut self, _target: &ArithmeticTarget, _bus: &mut MemoryBus) -> CpuEffect {
        todo!()
    }

    // rotate left for register A
    fn rla(&mut self) -> CpuEffect {
        let value = self.registers.a();

        // check first bit
        let carry = (value & 0x80) == 0x80;

        let new_value = value << 1 | (if self.registers.f().carry() { 1 } else { 0 });

        self.registers.f_as_mut().set_carry(carry);

        self.registers.set_a(new_value);
        (self.pc + 1 , 4)
    }

    // rotate left
    fn rl(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let (value, pc_offset, read_offset) = self.read_value(target, bus);

        // check first bit
        let carry = (value & 0x80) == 0x80;

        let new_value = value << 1 | (if self.registers.f().carry() { 1 } else { 0 });

        self.registers.f_as_mut().set_carry(carry);

        let write_offset = self.write_value(target, new_value, bus);

        (self.pc + 1 + pc_offset , 8+read_offset+write_offset)
    }

    fn rlc(&mut self, _target: &ArithmeticTarget, _bus: &mut MemoryBus) -> CpuEffect {
        todo!()
    }

    /// Rotate right - rotate via the carry flag by one bit
    fn rr(&mut self, _target: &ArithmeticTarget, _bus: &mut MemoryBus) -> CpuEffect {
        todo!()
    }
    ///
    /// Rotate right - rotate NOT via the carry flag by one bit
    fn rrc(&mut self, _target: &ArithmeticTarget, _bus: &mut MemoryBus) -> CpuEffect {
        todo!()
    }

    // Rotate right without carry the register A
    fn rrca(&mut self) -> CpuEffect {
        todo!()
    }
    // Rotate left without carry the register A
    fn rlca(&mut self) -> CpuEffect {
        todo!()
    }

    /// Increment te value of the specified register by one
    fn inc(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let (value, pc_offset, read_offset) = self.read_value(target, bus);
        let (new_value, _did_overflow) = value.overflowing_add(1);

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        // CHECKME
        self.registers
            .f_as_mut()
            .set_half_carry((new_value & 0xF) + (value & 0xF) > 0xF);

        let write_offset = self.write_value(target, new_value, bus);

        (self.pc + 1 + pc_offset, 4 + read_offset + write_offset)
    }

    fn dec(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let (value, pc_offset, read_offset) = self.read_value(target, bus);
        let new_value = value.wrapping_sub(1);

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(true);
        self.registers
            .f_as_mut()
            .set_half_carry((value & 0xF) == 0);

        let write_offset = self.write_value(target, new_value, bus);

        (self.pc + 1+ pc_offset, 4+ read_offset + write_offset)
    }
    /// Set the complement to register A
    fn cpl(&mut self) -> CpuEffect {
        let value = self.registers.a();
        let new_value = value ^ 0xff;

        self.registers.f_as_mut().set_subtract(true);
        self.registers.f_as_mut().set_half_carry(true);

        self.registers.set_a(new_value);

        (self.pc + 1, 4)
    }

    /// set register bit at bit position to 1
    fn set(&mut self, target: &ArithmeticTarget, bit_pos: u8, bus: &mut MemoryBus) -> CpuEffect {
        let (value, pc_offset, read_offset) = self.read_value(target, bus);
        let new_value = value | (1u8 << bit_pos);

        let write_offset = self.write_value(target, new_value, bus);

        (self.pc + 1 + pc_offset, 8+ read_offset + write_offset)
    }

    /// reset register bit at bit position to 0
    fn reset(&mut self, target: &ArithmeticTarget, bit_pos: u8, bus: &mut MemoryBus) -> CpuEffect {
        let (value, pc_offset, read_offset) = self.read_value(target, bus);
        let new_value = value & (!(1 << bit_pos));

        let write_offset = self.write_value(target, new_value, bus);

        (self.pc + 1+ pc_offset, 8+ write_offset + read_offset)
    }

    /// bit value
    fn bit(&mut self, target: &ArithmeticTarget, bit_pos: u8, bus: &mut MemoryBus) -> CpuEffect {
        let (value, pc_offset, read_offset) = self.read_value(target, bus);

        self.registers
            .f_as_mut()
            .set_zero((value & (1 << bit_pos)) == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_half_carry(true);

        (self.pc + 1 + pc_offset, 8+read_offset)
    }

    /// swap
    fn swap(&mut self, target: &ArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        let (value, pc_offset, read_offset) = self.read_value(target, bus);
        let new_value = (value << 4) | (value >> 4);
        let write_offset = self.write_value(target, new_value, bus);

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_half_carry(false);
        self.registers.f_as_mut().set_carry(false);

        (self.pc + 1+ pc_offset, 8+write_offset + read_offset)
    }

    /// Push 2 bytes to stack
    fn push(&mut self, source: &WideArithmeticTarget, bus: &mut MemoryBus) -> CpuEffect {
        // read value from register
        let (value, _pc_offset, _read_offset) = self.read_value_16(source, bus);

        self.push_word(value, bus);

        // CHECKME
        (self.pc + 1, 16)
    }

    fn push_word(&mut self, value: u16, bus: &mut MemoryBus) {
        // decrese stack
        self.sp = self.sp.wrapping_sub(1);

        // write most significant part first
        bus.write_byte(self.sp, (value >> 8) as u8);

        // decrese stack
        self.sp = self.sp.wrapping_sub(1);

        // write least significant part then
        bus.write_byte(self.sp, (value & 0xFF) as u8)
    }

    /// Pop 2 bytes from the stack
    fn pop(&mut self, target: &WideArithmeticTarget, bus: &MemoryBus) -> CpuEffect {
        let value = self.pop_word(bus);

        self.write_value_16(target, value);

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
    fn call(&mut self, test: &JumpTest, bus: &mut MemoryBus) -> CpuEffect {
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
            return (address, 16);
        }
        // else juste skip?

        //TODO
        (self.pc + 1, 20)
    }
}

#[cfg(test)]
#[path = "cpu_tests.rs"]
mod tests;
