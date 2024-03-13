use super::arithmetictarget::{ArithmeticTarget, WideArithmeticTarget};
use super::instruction::{Instruction, JumpTest, JumpType};
use super::memory::SharedMemory;
use super::registers::Registers;
use std::ops::Not;

type Delay = u32;
type ProgramCounter = u16;

/// Custom type to split the ProgramCounter and the time offset / number of cycle of the CPU
type CpuEffect = (ProgramCounter, Delay);
const NO_CPU_EFFECT: CpuEffect = (0, 0);

pub struct Cpu {
    registers: Registers,
    pc: ProgramCounter,
    sp: u16,
    is_halted: bool,
    memory: SharedMemory,
}

impl Cpu {
    pub fn new(memory: SharedMemory) -> Self {
        Self {
            registers: Registers::new(),
            pc: 0 as ProgramCounter,
            sp: 0u16,
            is_halted: false,
            memory,
        }
    }

    pub fn step(&mut self) -> Delay {
        // Check if prefixed instruction
        let instruction_byte = self.memory.read_byte(self.pc);
        let instruction = match instruction_byte {
            // prefetched
            0xCB => {
                let instruction_byte = self.memory.read_byte(self.pc + 1);
                Instruction::from_prefixed_byte(instruction_byte)
            }
            _ => Instruction::from_byte(instruction_byte),
        }
        .unwrap_or_else(|| {
            log::warn!("Unknown instruction : 0x{:x}", instruction_byte);
            Instruction::Nop
        });

        log::trace!(
            "|0x{:2x}|{:24}|Pc:0x{:04x}|HL:0x{:04x}|\r",
            instruction_byte,
            instruction.to_string(),
            self.pc,
            self.registers.hl(),
        );

        let (new_pc, delay) = self.execute(instruction);
        self.pc = new_pc;
        delay
    }

    fn execute(&mut self, instruction: Instruction) -> CpuEffect {
        match instruction {
            Instruction::Adc(target) => self.adc(&target),
            Instruction::Add(target) => self.add(&target),
            // CHECK special treamtment here
            Instruction::AddHL(_target) => todo!(),
            Instruction::AddSp => self.add_sp(),
            Instruction::And(target) => self.and(&target),
            Instruction::Bit(target, byte) => self.bit(&target, byte),
            Instruction::Ccf => self.ccf(),
            Instruction::Cp(target) => self.cp(&target),
            Instruction::Cpl => self.cpl(),
            Instruction::Dec(target) => self.dec(&target),
            Instruction::Dec16(target) => self.dec_16(&target),
            Instruction::Inc(target) => self.inc(&target),
            Instruction::Inc16(target) => self.inc_16(&target),
            Instruction::Or(target) => self.or(&target),
            Instruction::Res(target, byte) => self.reset(&target, byte),
            Instruction::Rla => self.rla(),
            Instruction::Rlc(target) => self.rlc(&target),
            Instruction::Rr(target) => self.rr(&target),
            Instruction::Rl(target) => self.rl(&target),
            Instruction::Rra => self.rra(),
            Instruction::Rrc(target) => self.rrc(&target),
            Instruction::Rrca => self.rrca(),
            Instruction::Rlca => self.rlca(),
            Instruction::Sbc(target) => self.sbc(&target),
            Instruction::Set(target, byte) => self.set(&target, byte),
            Instruction::Sla(target) => self.sla(&target),
            Instruction::Sra(target) => self.sra(&target),
            Instruction::Srl(target) => self.srl(&target),
            Instruction::Sub(target) => self.sub(&target),
            Instruction::Swap(target) => self.swap(&target),
            Instruction::Xor(target) => self.xor(&target),
            // JUMP
            Instruction::Jump(test, nature) => self.jump(&test, &nature),
            // TODO check me
            Instruction::Load {
                to: target,
                from: source,
            } => self.load(&target, &source),
            Instruction::Load16 { to, from } => self.load_16(&to, &from),
            // Stack and Call
            Instruction::Push(source) => self.push(&source),
            Instruction::Pop(target) => self.pop(&target),
            Instruction::Call(test) => self.call(&test),
            Instruction::Ret(test) => self.ret(&test),
            // Interrupt
            Instruction::DisableInterrupt => self.disable_interrupt(),
            Instruction::EnableInterrupt => self.enable_interrupt(),
            Instruction::Nop => self.nop(),
            Instruction::Halt => self.halt(),
            Instruction::Stop => self.stop(),
            Instruction::Rst(address) => self.rst(address),
            Instruction::Daa => self.daa(),
            Instruction::Scf => self.scf(),
        }
    }

    fn read_value(&mut self, target: &ArithmeticTarget) -> (u8, ProgramCounter, Delay) {
        match target {
            ArithmeticTarget::A => (self.registers.a(), 0, 0),
            ArithmeticTarget::B => (self.registers.b(), 0, 0),
            ArithmeticTarget::C => (self.registers.c(), 0, 0),
            ArithmeticTarget::D => (self.registers.d(), 0, 0),
            ArithmeticTarget::E => (self.registers.e(), 0, 0),
            ArithmeticTarget::H => (self.registers.h(), 0, 0),
            ArithmeticTarget::L => (self.registers.l(), 0, 0),
            // Memory
            ArithmeticTarget::BCTarget => (self.memory.read_byte(self.registers.bc()), 0, 4),
            ArithmeticTarget::DETarget => (self.memory.read_byte(self.registers.de()), 0, 4),
            ArithmeticTarget::HLTarget => (self.memory.read_byte(self.registers.hl()), 0, 4),
            // PC
            ArithmeticTarget::FFC => (
                self.memory.read_byte(0xFF00 + self.registers.c() as u16),
                0,
                4,
            ),
            ArithmeticTarget::FFRead => (
                self.memory
                    .read_byte(0xFF00 + self.memory.read_byte(self.pc + 1) as u16),
                1,
                4,
            ),
            ArithmeticTarget::ReadByte => (self.memory.read_byte(self.pc + 1), 1, 4),
            // CHECKME
            ArithmeticTarget::Pointer => {
                let address = self.memory.read_word(self.pc + 1);
                (self.memory.read_byte(address), 2, 12)
            }
            // Read value pointer by HL then increment HL
            ArithmeticTarget::HLInc => {
                let address = self.registers.hl();
                let value = self.memory.read_byte(address);
                self.registers.set_hl(address + 1);
                (value, 1, 8)
            }
            // Read value pointer by HL then decrement HL
            ArithmeticTarget::HLDec => {
                let address = self.registers.hl();
                let value = self.memory.read_byte(address);
                self.registers.set_hl(address - 1);
                (value, 1, 8)
            }
        }
    }

    fn read_value_16(&self, target: &WideArithmeticTarget) -> (u16, ProgramCounter, Delay) {
        match target {
            WideArithmeticTarget::HL => (self.registers.hl(), 0, 0),
            WideArithmeticTarget::BC => (self.registers.bc(), 0, 0),
            WideArithmeticTarget::DE => (self.registers.de(), 0, 0),
            WideArithmeticTarget::AF => (self.registers.af(), 0, 0),
            WideArithmeticTarget::SP => (self.memory.read_word(self.sp), 0, 0),
            WideArithmeticTarget::ReadWord => (self.memory.read_word(self.pc + 1), 2, 4),
            WideArithmeticTarget::ReadAddress => panic!("Reading an address has no value here"),
        }
    }

    /// The the provided value in the right arithmetic target.
    /// The offset is either 4 or 8
    fn write_value(&mut self, target: &ArithmeticTarget, value: u8) -> CpuEffect {
        match target {
            ArithmeticTarget::A => {
                self.registers.set_a(value);
                NO_CPU_EFFECT
            }
            ArithmeticTarget::B => {
                self.registers.set_b(value);
                NO_CPU_EFFECT
            }
            ArithmeticTarget::C => {
                self.registers.set_c(value);
                NO_CPU_EFFECT
            }
            ArithmeticTarget::D => {
                self.registers.set_d(value);
                NO_CPU_EFFECT
            }
            ArithmeticTarget::E => {
                self.registers.set_e(value);
                NO_CPU_EFFECT
            }
            ArithmeticTarget::H => {
                self.registers.set_h(value);
                NO_CPU_EFFECT
            }
            ArithmeticTarget::L => {
                self.registers.set_l(value);
                NO_CPU_EFFECT
            }
            // target type
            ArithmeticTarget::BCTarget => {
                let address = self.registers.bc();
                self.memory.write_byte(address, value);
                (0, 4)
            }
            ArithmeticTarget::DETarget => {
                let address = self.registers.de();
                self.memory.write_byte(address, value);
                (0, 4)
            }
            ArithmeticTarget::HLTarget => {
                let address = self.registers.bc();
                self.memory.write_byte(address, value);
                (0, 4)
            }
            ArithmeticTarget::FFRead => {
                let offset = self.memory.read_byte(self.pc + 1);
                let address = 0xFF00 + (offset as u16);
                self.memory.write_byte(address, value);
                (1, 4)
            }
            ArithmeticTarget::FFC => {
                let address = 0xFF00 + (self.registers.c() as u16);
                self.memory.write_byte(address, value);
                (0, 4)
            }
            ArithmeticTarget::ReadByte => unreachable!("Can't right directly to next byte."),
            ArithmeticTarget::Pointer => {
                let address = self.memory.read_word(self.pc + 1);
                self.memory.write_byte(address, value);
                (2, 12)
            }
            // SPECIAL
            ArithmeticTarget::HLDec => {
                let address = self.registers.hl();
                self.memory.write_byte(address, value);
                self.registers.set_hl(address - 1);
                (0, 4)
            }
            ArithmeticTarget::HLInc => {
                let address = self.registers.hl();
                self.memory.write_byte(address, value);
                self.registers.set_hl(address + 1);
                (0, 4)
            }
        }
    }

    fn write_value_16(&mut self, target: &WideArithmeticTarget, value: u16) -> Delay {
        match target {
            WideArithmeticTarget::HL => {
                self.registers.set_hl(value);
                0
            }
            WideArithmeticTarget::BC => {
                self.registers.set_bc(value);
                0
            }
            WideArithmeticTarget::DE => {
                self.registers.set_de(value);
                0
            }
            WideArithmeticTarget::AF => {
                self.registers.set_af(value);
                0
            }
            // CHECKME  0 right offset ?
            WideArithmeticTarget::SP => {
                self.sp = value;
                0
            }
            WideArithmeticTarget::ReadWord => panic!("Can't right directly to the next bytes"),
            WideArithmeticTarget::ReadAddress => {
                let address = self.memory.read_word(self.pc + 1);
                self.memory.write_word(address, value);
                2
            }
        }
    }

    fn jump(&self, test: &JumpTest, nature: &JumpType) -> CpuEffect {
        if test.evaluate(self.registers.f()) {
            // should jump
            match nature {
                JumpType::Relative8 => {
                    let offset = self.memory.read_byte(self.pc + 1) as i8;
                    // This comes from the size of the instruction↘️
                    let address = (self.pc as i32 + offset as i32 + 2) as u16;
                    (address, 12)
                }
                JumpType::Pointer16 => (self.memory.read_word(self.pc + 1), 16),
                JumpType::HL => (self.registers.hl(), 4),
                _ => unimplemented!("Jump type missing!"),
            }
        } else {
            // just continue and skip the trailing data
            match nature {
                JumpType::Relative8 => (self.pc + 2, 8),
                JumpType::Pointer16 => (self.pc + 3, 12),
                JumpType::HL => unreachable!("HL jump as the JumpTest::Always"),
                _ => unimplemented!("Jump type missing!"),
            }
        }
    }

    fn halt(&mut self) -> CpuEffect {
        self.is_halted = true;
        (self.pc + 1, 4)
    }

    fn load(&mut self, target: &ArithmeticTarget, source: &ArithmeticTarget) -> CpuEffect {
        let (value, pc_offset, source_offset) = self.read_value(source);

        let (write_pc_offset, write_offset) = self.write_value(target, value);
        (
            self.pc + 1 + pc_offset + write_pc_offset,
            source_offset + write_offset,
        )
    }

    /// Load next 2 bytes in memory in the provided registers
    fn load_16(
        &mut self,
        target: &WideArithmeticTarget,
        source: &WideArithmeticTarget,
    ) -> CpuEffect {
        let (value, pc_offset, read_offset) = self.read_value_16(source);

        // CHECKME : can we write a u16 in other thing than a register ?
        let write_offset = self.write_value_16(target, value);
        (self.pc + 1 + pc_offset, 4 + read_offset + write_offset)
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
    fn add(&mut self, target: &ArithmeticTarget) -> CpuEffect {
        let (value, pc_offset, offset) = self.read_value(target);
        let (new_value, did_overflow) = self.registers.a().overflowing_add(value);

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_carry(did_overflow);

        let register_a = self.registers.a();
        self.registers.set_a(new_value);
        self.registers
            .f_as_mut()
            .set_half_carry((register_a & 0xF) + (value & 0xF) > 0xF);

        (self.pc + 1 + pc_offset, offset)
    }
    /// Read the next value as i8 then add it to the SP
    fn add_sp(&mut self) -> CpuEffect {
        let (value, _pc_offset, _offset) = self.read_value(&ArithmeticTarget::ReadByte);
        let sp_value = self.pop_word();
        self.push_word((sp_value as i16 + value as i16) as u16);
        (self.pc + 2, 16)
    }

    /// Add with carry
    fn adc(&mut self, target: &ArithmeticTarget) -> CpuEffect {
        let (value, pc_offset, offset) = self.read_value(target);
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

        (self.pc + 1 + pc_offset, 4 + offset)
    }
    /// Subscrate the target value to the A register.
    fn sub(&mut self, target: &ArithmeticTarget) -> CpuEffect {
        let (value, pc_offset, offset) = self.read_value(target);
        let (new_value, did_overflow) = self.registers.a().overflowing_sub(value);

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(true);
        self.registers.f_as_mut().set_carry(did_overflow);

        let register_a = self.registers.a();
        self.registers
            .f_as_mut()
            .set_half_carry((register_a & 0xF) < (value & 0xF));

        self.registers.set_a(new_value);

        (self.pc + 1 + pc_offset, 4 + offset)
    }

    /// Like sub but the carry value is also substracted
    fn sbc(&mut self, target: &ArithmeticTarget) -> CpuEffect {
        let (value, pc_offset, offset) = self.read_value(target);

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

        (self.pc + 1 + pc_offset, 4 + offset)
    }

    fn and(&mut self, target: &ArithmeticTarget) -> CpuEffect {
        let (value, pc_offset, offset) = self.read_value(target);
        let new_value = self.registers.a() & value;

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_half_carry(false);
        self.registers.f_as_mut().set_carry(false);

        self.registers.set_a(new_value);

        (self.pc + 1 + pc_offset, 4 + offset)
    }

    fn xor(&mut self, target: &ArithmeticTarget) -> CpuEffect {
        let (value, pc_offset, offset) = self.read_value(target);
        let new_value = self.registers.a() ^ value;

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_half_carry(false);
        self.registers.f_as_mut().set_carry(false);

        self.registers.set_a(new_value);

        (self.pc + 1 + pc_offset, 4 + offset)
    }

    fn or(&mut self, target: &ArithmeticTarget) -> CpuEffect {
        let (value, pc_offset, offset) = self.read_value(target);
        let new_value = self.registers.a() | value;

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_half_carry(false);
        self.registers.f_as_mut().set_carry(false);

        self.registers.set_a(new_value);
        (self.pc + 1 + pc_offset, 4 + offset)
    }

    fn cp(&mut self, target: &ArithmeticTarget) -> CpuEffect {
        let (value, pc_offset, offset) = self.read_value(target);
        let (new_value, did_overflow) = self.registers.a().overflowing_sub(value);

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(true);
        self.registers.f_as_mut().set_carry(did_overflow);

        let register_a = self.registers.a();
        self.registers
            .f_as_mut()
            .set_half_carry((register_a & 0xF) + (value & 0xF) > 0xF);

        (self.pc + 1 + pc_offset, 4 + offset)
    }

    /// Shift left arithmetic. Multiplies by 2
    fn sla(&mut self, target: &ArithmeticTarget) -> CpuEffect {
        let (value, pc_offset, source_offset) = self.read_value(target);
        let (new_value, did_overflow) = value.overflowing_mul(2);

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_half_carry(false);
        self.registers.f_as_mut().set_carry(did_overflow);

        let (write_pc_offset, write_delay_offset) = self.write_value(target, new_value);

        (
            self.pc + 1 + pc_offset + write_pc_offset,
            8 + write_delay_offset + source_offset,
        )
    }

    /// Shift right arithmetic. Divides by 2
    fn sra(&mut self, target: &ArithmeticTarget) -> CpuEffect {
        let (value, pc_offset, read_offset) = self.read_value(target);
        // check first bit
        let carry = (value & 0x01) == 0x01;
        let new_value = (value >> 1) | (value & 0x80);

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_half_carry(false);
        self.registers.f_as_mut().set_carry(carry);

        let (write_pc_offset, write_delay_offset) = self.write_value(target, new_value);

        (
            self.pc + 1 + pc_offset + write_pc_offset,
            4 + read_offset + write_delay_offset,
        )
    }

    /// Bit shift right
    fn srl(&mut self, target: &ArithmeticTarget) -> CpuEffect {
        let (value, pc_offset, read_offset) = self.read_value(target);
        // check first bit
        let carry = (value & 0x01) == 0x01;

        let new_value = value >> 1;

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_half_carry(false);
        self.registers.f_as_mut().set_carry(carry);

        let (write_pc_offset, write_delay_offset) = self.write_value(target, new_value);

        (
            self.pc + 1 + pc_offset + write_pc_offset,
            4 + read_offset + write_delay_offset,
        )
    }

    /// Rotate right for register A
    fn rra(&mut self) -> CpuEffect {
        let value = self.registers.a();

        // check last bit
        let carry = (value & 0x01) == 0x01;

        let new_value = (value >> 1) | (if self.registers.f().carry() { 0x8 } else { 0 });

        self.registers.f_as_mut().set_carry(carry);
        self.registers.f_as_mut().set_zero(new_value == 0);

        self.registers.set_a(new_value);
        (self.pc + 1, 4)
    }

    // Rotate left for register A
    fn rla(&mut self) -> CpuEffect {
        let value = self.registers.a();

        // check first bit
        let carry = (value & 0x80) == 0x80;

        let new_value = (value << 1) | (if self.registers.f().carry() { 1 } else { 0 });

        self.registers.f_as_mut().set_carry(carry);
        self.registers.f_as_mut().set_zero(new_value == 0);

        self.registers.set_a(new_value);
        (self.pc + 1, 4)
    }

    // Rotate right without carry the register A
    fn rrca(&mut self) -> CpuEffect {
        let value = self.registers.a();

        // check first bit
        let carry = (value & 0x01) == 0x01;

        let new_value = (value >> 1) | (if carry { 0x80 } else { 0 });

        self.registers.f_as_mut().set_carry(carry);
        self.registers.f_as_mut().set_zero(new_value == 0);

        self.registers.set_a(new_value);
        (self.pc + 1, 4)
    }
    // Rotate left without carry the register A
    fn rlca(&mut self) -> CpuEffect {
        let value = self.registers.a();

        // check first bit
        let carry = (value & 0x80) == 0x80;

        let new_value = (value << 1) | (if self.registers.f().carry() { 1 } else { 0 });

        self.registers.f_as_mut().set_carry(carry);

        self.registers.set_a(new_value);
        (self.pc + 1, 4)
    }

    // rotate left
    fn rl(&mut self, target: &ArithmeticTarget) -> CpuEffect {
        let (value, _pc_offset, read_offset) = self.read_value(target);

        // check first bit
        let carry = (value & 0x80) == 0x80;

        let new_value = (value << 1) | (if self.registers.f().carry() { 1 } else { 0 });

        self.registers.f_as_mut().set_carry(carry);
        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_half_carry(false);
        self.registers.f_as_mut().set_subtract(false);

        let (_write_pc_offset, write_delay_offset) = self.write_value(target, new_value);

        (self.pc + 2, 8 + read_offset + write_delay_offset)
    }

    fn rlc(&mut self, target: &ArithmeticTarget) -> CpuEffect {
        let (value, _pc_offset, read_offset) = self.read_value(target);

        // check first bit
        let carry = (value & 0x80) == 0x80;

        let new_value = (value << 1) | (if carry { 1 } else { 0 });

        self.registers.f_as_mut().set_carry(carry);
        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_half_carry(false);
        self.registers.f_as_mut().set_subtract(false);

        let (_write_pc_offset, write_delay_offset) = self.write_value(target, new_value);

        (self.pc + 2, 8 + read_offset + write_delay_offset)
    }

    /// Rotate right - rotate via the carry flag by one bit
    fn rr(&mut self, target: &ArithmeticTarget) -> CpuEffect {
        let (value, _pc_offset, read_offset) = self.read_value(target);

        // check first bit
        let carry = (value & 0x01) == 0x01;
        let new_value = (value >> 1) | (if self.registers.f().carry() { 0x80 } else { 0 });

        self.registers.f_as_mut().set_carry(carry);
        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_half_carry(false);
        self.registers.f_as_mut().set_subtract(false);

        let (_write_pc_offset, write_delay_offset) = self.write_value(target, new_value);

        (self.pc + 2, 8 + read_offset + write_delay_offset)
    }

    /// Rotate right - rotate NOT via the carry flag by one bit
    fn rrc(&mut self, target: &ArithmeticTarget) -> CpuEffect {
        let (value, _pc_offset, read_offset) = self.read_value(target);

        // check first bit
        let carry = (value & 0x01) == 0x01;
        let new_value = (value >> 1) | (if carry { 0x80 } else { 0 });

        self.registers.f_as_mut().set_carry(carry);
        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_half_carry(false);
        self.registers.f_as_mut().set_subtract(false);

        let (_write_pc_offset, write_delay_offset) = self.write_value(target, new_value);

        (self.pc + 2, 8 + read_offset + write_delay_offset)
    }

    /// Increment te value of the specified register by one
    fn inc(&mut self, target: &ArithmeticTarget) -> CpuEffect {
        let (value, pc_offset, read_offset) = self.read_value(target);
        let (new_value, _did_overflow) = value.overflowing_add(1);

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        // CHECKME
        self.registers
            .f_as_mut()
            .set_half_carry((new_value & 0xF) + (value & 0xF) > 0xF);

        let (write_pc_offset, write_delay_offset) = self.write_value(target, new_value);

        (
            self.pc + 1 + pc_offset + write_pc_offset,
            4 + read_offset + write_delay_offset,
        )
    }

    /// Increment te value of the specified registers by one
    fn inc_16(&mut self, target: &WideArithmeticTarget) -> CpuEffect {
        let (value, pc_offset, read_offset) = self.read_value_16(target);
        let (new_value, _did_overflow) = value.overflowing_add(1);

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        // CHECKME
        self.registers
            .f_as_mut()
            .set_half_carry((new_value & 0xF) + (value & 0xF) > 0xF);

        let write_delay_offset = self.write_value_16(target, new_value);

        (
            self.pc + 1 + pc_offset,
            4 + read_offset + write_delay_offset,
        )
    }

    fn dec(&mut self, target: &ArithmeticTarget) -> CpuEffect {
        let (value, pc_offset, read_offset) = self.read_value(target);
        let new_value = value.wrapping_sub(1);

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(true);
        self.registers.f_as_mut().set_half_carry((value & 0xF) == 0);

        let (write_pc_offset, write_delay_offset) = self.write_value(target, new_value);

        (
            self.pc + 1 + pc_offset + write_pc_offset,
            4 + read_offset + write_delay_offset,
        )
    }

    fn dec_16(&mut self, target: &WideArithmeticTarget) -> CpuEffect {
        let (value, pc_offset, read_offset) = self.read_value_16(target);
        let new_value = value.wrapping_sub(1);

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(true);

        let write_delay_offset = self.write_value_16(target, new_value);

        (
            self.pc + 1 + pc_offset,
            4 + read_offset + write_delay_offset,
        )
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
    fn set(&mut self, target: &ArithmeticTarget, bit_pos: u8) -> CpuEffect {
        let (value, _pc_offset, read_offset) = self.read_value(target);
        let new_value = value | (1u8 << bit_pos);

        let (_write_pc_offset, write_delay_offset) = self.write_value(target, new_value);

        (self.pc + 2, 8 + read_offset + write_delay_offset)
    }

    /// reset register bit at bit position to 0
    fn reset(&mut self, target: &ArithmeticTarget, bit_pos: u8) -> CpuEffect {
        let (value, _pc_offset, read_offset) = self.read_value(target);
        let new_value = value & (!(1 << bit_pos));

        let (_write_pc_offset, write_delay_offset) = self.write_value(target, new_value);

        (self.pc + 2, 8 + write_delay_offset + read_offset)
    }

    /// bit value
    fn bit(&mut self, target: &ArithmeticTarget, bit_pos: u8) -> CpuEffect {
        let (value, _pc_offset, read_offset) = self.read_value(target);

        self.registers
            .f_as_mut()
            .set_zero((value & (1 << bit_pos)) == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_half_carry(true);

        (self.pc + 2, 8 + read_offset)
    }

    /// swap
    /// CHECKME, swap lower and higher part or swapping all bits?
    fn swap(&mut self, target: &ArithmeticTarget) -> CpuEffect {
        let (value, pc_offset, read_offset) = self.read_value(target);
        let new_value = (value << 4) | (value >> 4);
        let (write_pc_offset, write_delay_offset) = self.write_value(target, new_value);

        self.registers.f_as_mut().set_zero(new_value == 0);
        self.registers.f_as_mut().set_subtract(false);
        self.registers.f_as_mut().set_half_carry(false);
        self.registers.f_as_mut().set_carry(false);

        (
            self.pc + 1 + pc_offset + write_pc_offset,
            8 + write_delay_offset + read_offset,
        )
    }

    /// Push 2 bytes to stack
    fn push(&mut self, source: &WideArithmeticTarget) -> CpuEffect {
        // read value from register
        let (value, _pc_offset, _read_offset) = self.read_value_16(source);

        self.push_word(value);

        (self.pc + 1, 16)
    }

    /// Write a word to the stack
    fn push_word(&mut self, value: u16) {
        // decrese stack
        self.sp = self.sp.wrapping_sub(1);

        // write most significant part first
        self.memory.write_byte(self.sp, (value >> 8) as u8);

        // decrese stack
        self.sp = self.sp.wrapping_sub(1);

        // write least significant part then
        self.memory.write_byte(self.sp, (value & 0xFF) as u8)
    }

    /// Pop 2 bytes from the stack
    fn pop(&mut self, target: &WideArithmeticTarget) -> CpuEffect {
        let value = self.pop_word();

        self.write_value_16(target, value);

        (self.pc + 1, 12)
    }

    /// Pop word from the stack and return its value as u16
    fn pop_word(&mut self) -> u16 {
        let lower_part = self.memory.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(1);

        let higher_part = self.memory.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(1);

        ((higher_part as u16) << 8) | (lower_part as u16)
    }

    /// Call function
    fn call(&mut self, test: &JumpTest) -> CpuEffect {
        let next_pc = self.pc + 3;

        if test.evaluate(self.registers.f()) {
            self.push_word(next_pc);
            // new pc is in the following bytes
            (self.memory.read_word(self.pc + 1), 12)
        } else {
            (next_pc, 12)
        }
    }

    /// Call provided address to reset the process
    fn rst(&mut self, address: u16) -> CpuEffect {
        self.push_word(self.pc + 1);
        (address, 4)
    }

    /// Return from function
    fn ret(&mut self, test: &JumpTest) -> CpuEffect {
        if test.evaluate(self.registers.f()) {
            let address = self.pop_word();

            // CHEKME
            return (address, 16);
        }
        // else juste skip?

        //TODO
        (self.pc + 1, 20)
    }

    /// Disable the interrupt flag
    fn disable_interrupt(&mut self) -> CpuEffect {
        log::info!("Disable interrupt");
        self.registers.f_as_mut().set_emi(false);
        (self.pc + 1, 4)
    }

    fn enable_interrupt(&mut self) -> CpuEffect {
        log::info!("Enable interrupt");
        // This flag should be set only *after* the next instruction
        self.registers.f_as_mut().set_emi(true);
        (self.pc + 1, 4)
    }

    /// Enter CPU very low power mode. Also used to switch between double and normal speed CPU
    /// modes in GBC.
    fn stop(&mut self) -> CpuEffect {
        log::info!("Stop");
        (self.pc + 1, 4)
    }

    /// Decimal Adjust Accumulator, of the A register
    fn daa(&mut self) -> CpuEffect {
        let mut value = self.registers.a();
        let mut set_carry = false;
        // Create adjust with carries
        let adjust = match (
            self.registers.f().carry() || (value > 0x99 && self.registers.f().subtract().not()),
            self.registers.f().half_carry()
                || (value & 0x0f > 0x09 && self.registers.f().subtract().not()),
        ) {
            (false, false) => 0x00,
            (false, true) => 0x06,
            (true, false) => {
                set_carry = true;
                0x60
            }
            (true, true) => {
                set_carry = true;
                0x66
            }
        };

        // is a substraction
        if self.registers.f().subtract() {
            value = value.wrapping_sub(adjust);
        // is an addition
        } else {
            value = value.wrapping_add(adjust);
        }

        self.registers.f_as_mut().set_zero(value == 0);
        self.registers.f_as_mut().set_half_carry(false);
        self.registers.f_as_mut().set_carry(set_carry);

        self.registers.set_a(value);

        (self.pc + 1, 4)
    }

    /// Set Carry Flag
    fn scf(&mut self) -> CpuEffect {
        self.registers.f_as_mut().set_carry(true);
        self.registers.f_as_mut().set_half_carry(false);
        self.registers.f_as_mut().set_subtract(false);
        (self.pc + 1, 4)
    }
}

#[cfg(test)]
#[path = "cpu_tests.rs"]
mod tests;
