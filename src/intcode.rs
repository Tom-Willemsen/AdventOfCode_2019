use ahash::AHashMap;
use std::collections::VecDeque;

const PANIC_ON_HIGH_MEM: bool = true;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct IntCodeState<const LOW_MEM_AMOUNT: usize = 256> {
    instruction_ptr: i64,
    base_ptr: i64,
    low_memory: Vec<i64>,
    high_memory: AHashMap<i64, i64>,
    pub out_buffer: VecDeque<i64>,
}

impl<const LOW_MEM_AMOUNT: usize> From<&[i64]> for IntCodeState<LOW_MEM_AMOUNT> {
    fn from(item: &[i64]) -> Self {
        let mut low_mem = item.to_vec();
        low_mem.extend(vec![0; LOW_MEM_AMOUNT.saturating_sub(low_mem.len())]);
        IntCodeState {
            instruction_ptr: 0,
            base_ptr: 0,
            low_memory: low_mem,
            high_memory: AHashMap::default(),
            out_buffer: VecDeque::new(),
        }
    }
}

impl<const LOW_MEM_AMOUNT: usize> From<Vec<i64>> for IntCodeState<LOW_MEM_AMOUNT> {
    fn from(item: Vec<i64>) -> Self {
        let mut low_mem = item;
        low_mem.extend(vec![0; LOW_MEM_AMOUNT.saturating_sub(low_mem.len())]);
        IntCodeState {
            instruction_ptr: 0,
            base_ptr: 0,
            low_memory: low_mem,
            high_memory: AHashMap::default(),
            out_buffer: VecDeque::new(),
        }
    }
}

pub fn parse_intcode_to_vec(inp: &str) -> Vec<i64> {
    inp.trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i64>>()
}

impl<const LOW_MEM_AMOUNT: usize> From<&str> for IntCodeState<LOW_MEM_AMOUNT> {
    fn from(item: &str) -> Self {
        parse_intcode_to_vec(item).into()
    }
}

#[derive(Debug)]
struct Instruction {
    num: u32,
}

impl From<i64> for Instruction {
    #[inline]
    fn from(item: i64) -> Self {
        Instruction {
            num: item.try_into().expect("invalid instruction"),
        }
    }
}

impl Instruction {
    #[inline]
    fn typ(&self) -> u32 {
        self.num % 100
    }

    #[inline]
    fn mode1(&self) -> u32 {
        (self.num / 100) % 10
    }

    #[inline]
    fn mode2(&self) -> u32 {
        (self.num / 1000) % 10
    }

    #[inline]
    fn mode3(&self) -> u32 {
        (self.num / 10000) % 10
    }
}

impl<const LOW_MEM_AMOUNT: usize> IntCodeState<LOW_MEM_AMOUNT> {
    #[cold]
    fn get_high_mem(&self, address_absolute: i64) -> i64 {
        if PANIC_ON_HIGH_MEM {
            panic!("high mem read address {}", address_absolute);
        }
        *self.high_memory.get(&address_absolute).unwrap_or(&0)
    }

    #[inline]
    pub fn get_mem(&self, address_absolute: i64) -> i64 {
        debug_assert!(address_absolute >= 0, "Address can't be negative");
        if (address_absolute as usize) < self.low_memory.len() {
            self.low_memory[address_absolute as usize]
        } else {
            self.get_high_mem(address_absolute)
        }
    }

    #[cold]
    fn set_high_mem(&mut self, address_absolute: i64, new: i64) {
        if PANIC_ON_HIGH_MEM {
            panic!("high mem write address {}", address_absolute);
        }
        self.high_memory.insert(address_absolute, new);
    }

    #[inline]
    pub fn set_mem(&mut self, address_absolute: i64, new: i64) {
        debug_assert!(address_absolute >= 0, "Address can't be negative");
        if (address_absolute as usize) < self.low_memory.len() {
            self.low_memory[address_absolute as usize] = new;
        } else {
            self.set_high_mem(address_absolute, new);
        }
    }

    #[inline]
    fn get_parameter(&self, mode: u32, offset: i64) -> i64 {
        let pos = self.get_mem(self.instruction_ptr + offset);
        if mode == 0 {
            self.get_mem(pos)
        } else if mode == 1 {
            pos
        } else if mode == 2 {
            self.get_mem(self.base_ptr + pos)
        } else {
            panic!("unexpected mode {:?}", mode)
        }
    }

    #[inline]
    fn set_parameter(&mut self, mode: u32, offset: i64, value: i64) {
        let pos = self.get_mem(self.instruction_ptr + offset);
        if mode == 0 {
            self.set_mem(pos, value)
        } else if mode == 1 {
            panic!("can't set parameter in immediate mode!")
        } else if mode == 2 {
            self.set_mem(self.base_ptr + pos, value)
        } else {
            panic!("unexpected mode {:?}", mode)
        }
    }

    fn handle_add(&mut self, ins: &Instruction) {
        let src1 = self.get_parameter(ins.mode1(), 1);
        let src2 = self.get_parameter(ins.mode2(), 2);

        self.set_parameter(ins.mode3(), 3, src1 + src2);

        self.instruction_ptr += 4;
    }

    fn handle_mul(&mut self, ins: &Instruction) {
        let src1 = self.get_parameter(ins.mode1(), 1);
        let src2 = self.get_parameter(ins.mode2(), 2);

        self.set_parameter(ins.mode3(), 3, src1 * src2);

        self.instruction_ptr += 4;
    }

    fn handle_inp<F>(&mut self, ins: &Instruction, mut input_handler: F)
    where
        F: FnMut(&mut IntCodeState<LOW_MEM_AMOUNT>) -> Option<i64>,
    {
        if let Some(inp) = input_handler(self) {
            self.set_parameter(ins.mode1(), 1, inp);
            self.instruction_ptr += 2;
        }
    }

    fn handle_out(&mut self, ins: &Instruction) {
        let x = self.get_parameter(ins.mode1(), 1);
        self.out_buffer.push_back(x);
        self.instruction_ptr += 2;
    }

    fn handle_jump_if<const COND: bool>(&mut self, ins: &Instruction) {
        if (self.get_parameter(ins.mode1(), 1) != 0) == COND {
            self.instruction_ptr = self.get_parameter(ins.mode2(), 2);
        } else {
            self.instruction_ptr += 3;
        }
    }

    fn handle_cmp_eq(&mut self, ins: &Instruction) {
        let x = self.get_parameter(ins.mode1(), 1);
        let y = self.get_parameter(ins.mode2(), 2);
        self.set_parameter(ins.mode3(), 3, if x == y { 1 } else { 0 });
        self.instruction_ptr += 4;
    }

    // Note: separate cmp_lt and cmp_eq implementations to help branch predictor
    // 20% perf improvement
    fn handle_cmp_lt(&mut self, ins: &Instruction) {
        let x = self.get_parameter(ins.mode1(), 1);
        let y = self.get_parameter(ins.mode2(), 2);
        self.set_parameter(ins.mode3(), 3, if x < y { 1 } else { 0 });
        self.instruction_ptr += 4;
    }

    fn handle_adjust_base_ptr(&mut self, ins: &Instruction) {
        self.base_ptr += self.get_parameter(ins.mode1(), 1);
        self.instruction_ptr += 2;
    }

    pub fn execute_single_step<F>(&mut self, mut input_handler: F) -> bool
    where
        F: FnMut(&mut IntCodeState<LOW_MEM_AMOUNT>) -> Option<i64>,
    {
        let raw_instruction: i64 = self.get_mem(self.instruction_ptr);
        let instruction: Instruction = raw_instruction.into();
        match instruction.typ() {
            1 => self.handle_add(&instruction),
            2 => self.handle_mul(&instruction),
            3 => self.handle_inp(&instruction, &mut input_handler),
            4 => self.handle_out(&instruction),
            5 => self.handle_jump_if::<true>(&instruction),
            6 => self.handle_jump_if::<false>(&instruction),
            7 => self.handle_cmp_lt(&instruction),
            8 => self.handle_cmp_eq(&instruction),
            9 => self.handle_adjust_base_ptr(&instruction),
            99 => return true,
            other => panic!("bad instruction {:?}", other),
        }

        false
    }

    pub fn execute_until_halt<F>(&mut self, mut input_handler: F)
    where
        F: FnMut(&mut IntCodeState<LOW_MEM_AMOUNT>) -> Option<i64>,
    {
        loop {
            let halt = self.execute_single_step(&mut input_handler);
            if halt {
                break;
            }
        }
    }

    pub fn execute_until_halt_no_input(&mut self) {
        self.execute_until_halt(|_| panic!("should not ask for input"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_negative_number() {
        let mut prog: IntCodeState = vec![1101, 100, -1, 4, 0].into();
        prog.execute_until_halt_no_input();
        assert_eq!(prog.get_mem(4), 99);
    }

    #[test]
    fn test_multiply_parameter_modes() {
        let mut prog: IntCodeState = vec![1002, 4, 3, 4, 33].into();
        prog.execute_until_halt_no_input();
        assert_eq!(prog.get_mem(4), 99);
    }

    #[test]
    fn test_basic_io() {
        let mut prog: IntCodeState = vec![3, 0, 4, 0, 99].into();
        prog.execute_until_halt(|_| Some(123456));
        assert_eq!(prog.out_buffer.pop_front(), Some(123456));
    }

    #[test]
    fn test_eq_8_position_mode() {
        for val in [0, 7, 8, 9, 123456] {
            let mut prog: IntCodeState = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].into();
            prog.execute_until_halt(|_| Some(val));
            assert_eq!(
                prog.out_buffer.pop_front(),
                Some(if val == 8 { 1 } else { 0 })
            );
        }
    }

    #[test]
    fn test_lt_8_position_mode() {
        for val in [0, 7, 8, 9, 123456] {
            let mut prog: IntCodeState = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].into();
            prog.execute_until_halt(|_| Some(val));
            assert_eq!(
                prog.out_buffer.pop_front(),
                Some(if val < 8 { 1 } else { 0 })
            );
        }
    }

    #[test]
    fn test_eq_8_immediate_mode() {
        for val in [0, 7, 8, 9, 123456] {
            let mut prog: IntCodeState = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99].into();
            prog.execute_until_halt(|_| Some(val));
            assert_eq!(
                prog.out_buffer.pop_front(),
                Some(if val == 8 { 1 } else { 0 })
            );
        }
    }

    #[test]
    fn test_lt_8_immediate_mode() {
        for val in [0, 7, 8, 9, 123456] {
            let mut prog: IntCodeState = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99].into();
            prog.execute_until_halt(|_| Some(val));
            assert_eq!(
                prog.out_buffer.pop_front(),
                Some(if val < 8 { 1 } else { 0 })
            );
        }
    }

    #[test]
    fn test_jmp_position_mode() {
        for val in [0, 7, 8, 9, 123456] {
            let mut prog: IntCodeState =
                vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9].into();
            prog.execute_until_halt(|_| Some(val));
            assert_eq!(
                prog.out_buffer.pop_front(),
                Some(if val > 0 { 1 } else { 0 })
            );
        }
    }

    #[test]
    fn test_jmp_immediate_mode() {
        for val in [0, 7, 8, 9, 123456] {
            let mut prog: IntCodeState =
                vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1].into();
            prog.execute_until_halt(|_| Some(val));
            assert_eq!(
                prog.out_buffer.pop_front(),
                Some(if val > 0 { 1 } else { 0 })
            );
        }
    }

    #[test]
    fn test_quine() {
        let quine = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut prog: IntCodeState = quine.clone().into();
        prog.execute_until_halt_no_input();
        prog.out_buffer.make_contiguous();
        assert_eq!(prog.out_buffer.as_slices().0, quine);
    }

    #[test]
    fn test_output_16_digit_number() {
        let mut prog: IntCodeState = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0].into();
        prog.execute_until_halt_no_input();
        let n = prog.out_buffer.pop_front().unwrap();
        assert_eq!(n, 1219070632396864);
    }

    #[test]
    fn test_output_large_number() {
        let mut prog: IntCodeState = vec![104, 1125899906842624, 99].into();
        prog.execute_until_halt_no_input();
        assert_eq!(prog.out_buffer.pop_front(), Some(1125899906842624));
    }
}
