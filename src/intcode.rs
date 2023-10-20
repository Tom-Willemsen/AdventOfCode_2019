use std::collections::VecDeque;
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct IntCodeState {
    state: Vec<i64>,
    instruction_ptr: usize,
    pub out_buffer: VecDeque<i64>,
}

impl From<&[i64]> for IntCodeState {
    fn from(item: &[i64]) -> Self {
        IntCodeState {
            state: item.to_vec(),
            instruction_ptr: 0,
            out_buffer: VecDeque::new(),
        }
    }
}

impl From<Vec<i64>> for IntCodeState {
    fn from(item: Vec<i64>) -> Self {
        IntCodeState {
            state: item,
            instruction_ptr: 0,  
            out_buffer: VecDeque::new(),

        }
    }
}

#[derive(Debug)]
struct Instruction {
    typ: i64,
    mode1: i64,
    mode2: i64,
}

impl From<i64> for Instruction {
    fn from(item: i64) -> Self {
        assert!(item > 0);
        Instruction {
            typ: item % 100,
            mode1: (item / 100) % 10,
            mode2: (item / 1000) % 10,
        }
    }
}

impl IntCodeState {
    fn offset_to_ref(&self, offset: usize) -> usize {
        let pos = *self
            .state
            .get(self.instruction_ptr + offset)
            .expect("invalid offset");
        usize::try_from(pos).expect("can't convert to usize")
    }

    pub fn get_mem(&self, address_absolute: usize) -> i64 {
        *self.state.get(address_absolute).expect("invalid reference")
    }

    pub fn set_mem(&mut self, address_absolute: usize, new: i64) {
        let r = self
            .state
            .get_mut(address_absolute)
            .expect("invalid reference");
        *r = new;
    }

    fn deref(&self, offset: usize) -> &i64 {
        let pos = self.offset_to_ref(offset);
        self.state.get(pos).expect("invalid reference")
    }

    fn deref_mut(&mut self, offset: usize) -> &mut i64 {
        let pos = self.offset_to_ref(offset);
        self.state.get_mut(pos).expect("invalid reference")
    }

    fn get_parameter(&self, mode: i64, offset: usize) -> i64 {
        if mode == 0 {
            *self.deref(offset)    
        } else if mode == 1 { 
            self.get_mem(self.instruction_ptr + offset) 
        } else { 
            panic!("unexpected mode {:?}", mode)
        }
    }

    fn handle_add(&mut self, mode1: i64, mode2: i64) {
        let src1 = self.get_parameter(mode1, 1);
        let src2 = self.get_parameter(mode2, 2);

        let dest = self.deref_mut(3);
        *dest = src1 + src2;

        self.instruction_ptr += 4;
    }

    fn handle_mul(&mut self, mode1: i64, mode2: i64) {
        let src1 = self.get_parameter(mode1, 1);
        let src2 = self.get_parameter(mode2, 2);

        let dest = self.deref_mut(3);
        *dest = src1 * src2;

        self.instruction_ptr += 4;
    }

    fn handle_inp<F>(&mut self, mut input_handler: F)
        where F: FnMut() -> Option<i64>
    {
        if let Some(inp) = input_handler() {
            let dest = self.deref_mut(1);
            *dest = inp;
            self.instruction_ptr += 2;
        }
    }

    fn handle_out(&mut self, mode1: i64) {
        let x = self.get_parameter(mode1, 1);
        self.out_buffer.push_back(x);
        self.instruction_ptr += 2;
    }

    fn handle_jump_if<const COND: bool>(&mut self, mode1: i64, mode2: i64) {
        if (self.get_parameter(mode1, 1) != 0) == COND {
            self.instruction_ptr = usize::try_from(self.get_parameter(mode2, 2)).expect("can't assign to inst ptr");
        } else {
            self.instruction_ptr += 3;
        }
    }

    fn handle_cmp(&mut self, order: Ordering, mode1: i64, mode2: i64) {
        let x = self.get_parameter(mode1, 1);
        let y = self.get_parameter(mode2, 2);
        let dest = self.deref_mut(3);
        *dest = if x.cmp(&y) == order { 1 } else { 0 };
        self.instruction_ptr += 4;
    }

    pub fn execute_single_step<F>(&mut self, mut input_handler: F) -> bool
        where F: FnMut() -> Option<i64>
    {
        let raw_instruction: i64 = self.get_mem(self.instruction_ptr);
        let instruction: Instruction = raw_instruction.into();
        
        match instruction.typ {
            1 => self.handle_add(instruction.mode1, instruction.mode2),
            2 => self.handle_mul(instruction.mode1, instruction.mode2),
            3 => self.handle_inp(&mut input_handler),
            4 => self.handle_out(instruction.mode1),
            5 => self.handle_jump_if::<true>(instruction.mode1, instruction.mode2),
            6 => self.handle_jump_if::<false>(instruction.mode1, instruction.mode2),
            7 => self.handle_cmp(Ordering::Less, instruction.mode1, instruction.mode2),
            8 => self.handle_cmp(Ordering::Equal, instruction.mode1, instruction.mode2),
            99 => return true,
            other => panic!("bad instruction {:?}", other),
        }

        false
    }

    pub fn execute_until_halt<F>(&mut self, mut input_handler: F) 
        where F: FnMut() -> Option<i64>
    {
        loop {
            let halt = self.execute_single_step(&mut input_handler);
            if halt {
                break;
            }
        }
    }
    
    pub fn execute_until_halt_no_input(&mut self) {
        self.execute_until_halt(|| panic!("should not ask for input"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_negative_number() {
        let mut prog: IntCodeState = vec![1101,100,-1,4,0].into();
        prog.execute_until_halt_no_input();
        assert_eq!(prog.get_mem(4), 99);
    }

    #[test]
    fn test_multiply_parameter_modes() {
        let mut prog: IntCodeState = vec![1002,4,3,4,33].into();
        prog.execute_until_halt_no_input();
        assert_eq!(prog.get_mem(4), 99);
    }

    #[test]
    fn test_basic_io() {
        let mut prog: IntCodeState = vec![3,0,4,0,99].into();
        prog.execute_until_halt(|| Some(123456));
        assert_eq!(prog.out_buffer.pop_front(), Some(123456));
    }
    
    #[test]
    fn test_eq_8_position_mode() {
        for val in [0, 7, 8, 9, 123456] {
            let mut prog: IntCodeState = vec![3,9,8,9,10,9,4,9,99,-1,8].into();
            prog.execute_until_halt(|| Some(val));
            assert_eq!(prog.out_buffer.pop_front(), Some(if val == 8 { 1 } else { 0 }));
        }
    }
    
    #[test]
    fn test_lt_8_position_mode() {
        for val in [0, 7, 8, 9, 123456] {
            let mut prog: IntCodeState = vec![3,9,7,9,10,9,4,9,99,-1,8].into();
            prog.execute_until_halt(|| Some(val));
            assert_eq!(prog.out_buffer.pop_front(), Some(if val < 8 { 1 } else { 0 }));
        }
    }
    
    #[test]
    fn test_eq_8_immediate_mode() {
        for val in [0, 7, 8, 9, 123456] {
            let mut prog: IntCodeState = vec![3,3,1108,-1,8,3,4,3,99].into();
            prog.execute_until_halt(|| Some(val));
            assert_eq!(prog.out_buffer.pop_front(), Some(if val == 8 { 1 } else { 0 }));
        }
    }
    
    #[test]
    fn test_lt_8_immediate_mode() {
        for val in [0, 7, 8, 9, 123456] {
            let mut prog: IntCodeState = vec![3,3,1107,-1,8,3,4,3,99].into();
            prog.execute_until_halt(|| Some(val));
            assert_eq!(prog.out_buffer.pop_front(), Some(if val < 8 { 1 } else { 0 }));
        }
    }
    
    #[test]
    fn test_jmp_position_mode() {
        for val in [0, 7, 8, 9, 123456] {
            let mut prog: IntCodeState = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9].into();
            prog.execute_until_halt(|| Some(val));
            assert_eq!(prog.out_buffer.pop_front(), Some(if val > 0 { 1 } else { 0 }));
        }
    }
    
    #[test]
    fn test_jmp_immediate_mode() {
        for val in [0, 7, 8, 9, 123456] {
            let mut prog: IntCodeState = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1].into();
            prog.execute_until_halt(|| Some(val));
            assert_eq!(prog.out_buffer.pop_front(), Some(if val > 0 { 1 } else { 0 }));
        }
    }

}
