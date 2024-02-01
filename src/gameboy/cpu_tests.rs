use crate::gameboy::{
    arithmetictarget::ArithmeticTarget, instruction::Instruction, memory::MemoryBus,
};
use std::ops::Not;

use super::Cpu;
mod instructions {
    use super::*;
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
        let mut cpu = Cpu::new();
        let mut memory_bus = MemoryBus::default();
        cpu.registers.set_a(1);
        cpu.registers.set_b(2);
        let instruction = Instruction::Adc(ArithmeticTarget::B);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.a(), 3);
        assert!(cpu.registers.f().carry().not());
        // testing overflow
        cpu.registers.set_a(250);
        cpu.registers.set_b(7);
        let instruction = Instruction::Adc(ArithmeticTarget::B);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.a(), 1);
        assert!(cpu.registers.f().carry());
    }

    #[test]
    fn sub() {
        let mut cpu = Cpu::new();
        let mut memory_bus = MemoryBus::default();
        cpu.registers.set_a(0);
        cpu.registers.set_b(1);
        let instruction = Instruction::Sub(ArithmeticTarget::B);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.a(), 255);
        assert!(cpu.registers.f().carry());
        // testing overflow
        cpu.registers.set_a(4);
        cpu.registers.set_b(1);
        let instruction = Instruction::Sub(ArithmeticTarget::B);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.a(), 3);
    }

    #[test]
    fn sbc() {
        let mut cpu = Cpu::new();
        let mut memory_bus = MemoryBus::default();
        cpu.registers.set_a(0);
        cpu.registers.set_b(1);
        let instruction = Instruction::Sub(ArithmeticTarget::B);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.a(), 255);
        assert!(cpu.registers.f().carry());
        let instruction = Instruction::Sub(ArithmeticTarget::B);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.a(), 254);
        assert!(cpu.registers.f().carry().not());
    }

    #[test]
    fn and() {
        let mut cpu = Cpu::new();
        let mut memory_bus = MemoryBus::default();
        cpu.registers.set_a(0xf0);
        cpu.registers.set_b(0x0f);
        let instruction = Instruction::And(ArithmeticTarget::B);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.a(), 0);
        assert!(cpu.registers.f().carry().not());
    }

    #[test]
    fn xor() {
        let mut cpu = Cpu::new();
        let mut memory_bus = MemoryBus::default();
        cpu.registers.set_a(0xf0);
        cpu.registers.set_b(0x0f);
        let instruction = Instruction::Xor(ArithmeticTarget::B);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.a(), 0xff);
        assert!(cpu.registers.f().carry().not());
        cpu.registers.set_a(0x0f);
        let instruction = Instruction::Xor(ArithmeticTarget::B);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.a(), 0x00);
        assert!(cpu.registers.f().carry().not());
    }

    #[test]
    fn or() {
        let mut cpu = Cpu::new();
        let mut memory_bus = MemoryBus::default();
        cpu.registers.set_a(0xf0);
        cpu.registers.set_b(0x0f);
        let instruction = Instruction::Or(ArithmeticTarget::B);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.a(), 0xff);
        assert!(cpu.registers.f().carry().not());
        cpu.registers.set_a(0x0f);
        let instruction = Instruction::Or(ArithmeticTarget::B);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.a(), 0x0f);
        assert!(cpu.registers.f().carry().not());
    }

    #[test]
    fn cp() {
        let mut cpu = Cpu::new();
        let mut memory_bus = MemoryBus::default();
        // A > B
        cpu.registers.set_a(0xf0);
        cpu.registers.set_b(0x0f);
        let instruction = Instruction::Cp(ArithmeticTarget::B);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.a(), 0xf0);
        assert!(cpu.registers.f().carry().not());
        assert!(cpu.registers.f().zero().not());
        // Eq
        cpu.registers.set_a(0x0f);
        let instruction = Instruction::Cp(ArithmeticTarget::B);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.a(), 0x0f);
        assert!(cpu.registers.f().carry().not());
        assert!(cpu.registers.f().zero());
        // B > A
        cpu.registers.set_a(0x0e);
        let instruction = Instruction::Cp(ArithmeticTarget::B);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.a(), 0x0e);
        assert!(cpu.registers.f().carry());
        assert!(cpu.registers.f().zero().not());
    }

    #[test]
    fn inc() {
        let mut cpu = Cpu::new();
        let mut memory_bus = MemoryBus::default();
        cpu.registers.set_b(0x0f);
        let instruction = Instruction::Inc(ArithmeticTarget::B);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.b(), 0x10);
        assert!(cpu.registers.f().carry().not());
        assert!(cpu.registers.f().zero().not());

        cpu.registers.set_b(0xff);
        let instruction = Instruction::Inc(ArithmeticTarget::B);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.b(), 0x0);
        assert!(cpu.registers.f().carry().not());
        assert!(cpu.registers.f().zero());
    }

    #[test]
    fn dec() {
        let mut cpu = Cpu::new();
        let mut memory_bus = MemoryBus::default();
        cpu.registers.set_b(0x01);
        let instruction = Instruction::Dec(ArithmeticTarget::B);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.b(), 0x0);
        assert!(cpu.registers.f().carry().not());
        assert!(cpu.registers.f().zero());

        cpu.registers.set_b(0xff);
        let instruction = Instruction::Dec(ArithmeticTarget::B);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.b(), 0xfe);
        assert!(cpu.registers.f().carry().not());
        assert!(cpu.registers.f().zero().not());
    }

    #[test]
    fn daa() {
        let mut cpu = Cpu::new();
        let mut memory_bus = MemoryBus::default();
        cpu.registers.set_b(0x01);
        let instruction = Instruction::Dec(ArithmeticTarget::B);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.b(), 0x0);
        assert!(cpu.registers.f().carry().not());
        assert!(cpu.registers.f().zero());
    }

    #[test]
    fn cpl() {
        let mut cpu = Cpu::new();
        let mut memory_bus = MemoryBus::default();
        cpu.registers.set_a(0x01);
        let instruction = Instruction::Cpl;
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.a(), 0xfe);
        assert!(cpu.registers.f().half_carry());
        assert!(cpu.registers.f().subtract());
    }
}

#[cfg(NON)]
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

#[cfg(NON)]
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

mod test_1bit {
    use super::*;
    #[test]
    fn bit() {
        let mut cpu = Cpu::new();
        let mut memory_bus = MemoryBus::default();
        cpu.registers.set_b(0b0100_0000);
        let instruction = Instruction::Bit(ArithmeticTarget::B, 6);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.b(), 0b0100_0000);
        assert!(cpu.registers.f().half_carry());
        assert!(cpu.registers.f().zero().not());
        assert!(cpu.registers.f().subtract().not());
        // not carry; zero
        let instruction = Instruction::Bit(ArithmeticTarget::B, 7);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.b(), 0b0100_0000);
        assert!(cpu.registers.f().half_carry());
        assert!(cpu.registers.f().zero());
        assert!(cpu.registers.f().subtract().not());
    }

    #[test]
    fn set() {
        let mut cpu = Cpu::new();
        let mut memory_bus = MemoryBus::default();
        cpu.registers.set_b(0b0100_0000);
        let instruction = Instruction::Set(ArithmeticTarget::B, 6);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.b(), 0b0100_0000);

        let instruction = Instruction::Set(ArithmeticTarget::B, 0);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.b(), 0b0100_0001);
    }

    #[test]
    fn res() {
        let mut cpu = Cpu::new();
        let mut memory_bus = MemoryBus::default();
        cpu.registers.set_b(0b0100_0001);
        let instruction = Instruction::Res(ArithmeticTarget::B, 6);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.b(), 0b0000_0001);

        let instruction = Instruction::Res(ArithmeticTarget::B, 2);
        cpu.execute(instruction, &mut memory_bus);
        assert_eq!(cpu.registers.b(), 0b0000_0001);
    }
}

#[cfg(NON)]
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

#[cfg(NON)]
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
