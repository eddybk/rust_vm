use std::fs::File;
use std::io::prelude::*;
use std::process::exit;
use crate::vm::InstSuccess::OK;
use crate::vm::ExitCode::{FEXT, MEXT};
use lazy_static::lazy_static;
use regex::Regex;


// TODO:
// ENCODE BYTES AS CHARACTERS AND READ THEM AS SUCH WITHING THE BINARY FILES. MASSIVELY SAVES SPACE, N * 8 EFFICIENCY (WHERE N IS #BYTES)

pub type Word = u64;
const STACK_CAP: usize = 2048;
enum ExitCode {
    FEXT = 101,
    MEXT = 202
}


#[derive(Debug)]
pub struct VM {
    stack: [Word; STACK_CAP],
    stack_size: usize,
    program: Vec<Instruction>,
    ptr: usize
}

fn bin_formatter(s: String) -> String {
    let mut b = String::from("");
    let mut l = s.len();
    while l % 8 != 0 {
        b += "0";
        l += 1;
    }
    format!("{}{}", b, s)
}

fn bin_formatter_o(s: String) -> String {
    let mut b = String::from("");
    let mut l = s.len();
    while l % 64 != 0 {
        b += "0";
        l += 1;
    }
    format!("{}{}", b, s)
}

fn inst_to_string(inst: Instruction) -> String {
    let mut ret = String::from("");
    match inst {
        Instruction::PUSH(operand) => {
            ret += bin_formatter(format!("{:b}", 0)).as_str();
            ret += bin_formatter_o(format!("{:b}", operand)).as_str();
        }
        Instruction::ADD => {
            ret += bin_formatter(format!("{:b}", 1)).as_str();
            ret += bin_formatter_o(format!("{:b}", 0)).as_str();
        }
        Instruction::SUB => {
            ret += bin_formatter(format!("{:b}", 2)).as_str();
            ret += bin_formatter_o(format!("{:b}", 0)).as_str();
        }
        Instruction::MUL => {
            ret += bin_formatter(format!("{:b}", 3)).as_str();
            ret += bin_formatter_o(format!("{:b}", 0)).as_str();
        }
        Instruction::DIV => {
            ret += bin_formatter(format!("{:b}", 4)).as_str();
            ret += bin_formatter_o(format!("{:b}", 0)).as_str();
        }
        Instruction::DUP(operand) => {
            ret += bin_formatter(format!("{:b}", 5)).as_str();
            ret += bin_formatter_o(format!("{:b}", operand)).as_str();
        }
        Instruction::DUMP => {
            ret += bin_formatter(format!("{:b}", 6)).as_str();
            ret += bin_formatter_o(format!("{:b}", 0)).as_str();
        }
        Instruction::PRINT => {
            ret += bin_formatter(format!("{:b}", 7)).as_str();
            ret += bin_formatter_o(format!("{:b}", 0)).as_str();
        }
        Instruction::JMP(operand) => {
            ret += bin_formatter(format!("{:b}", 8)).as_str();
            ret += bin_formatter_o(format!("{:b}", operand)).as_str();
        }
        Instruction::EQ => {
            ret += bin_formatter(format!("{:b}", 9)).as_str();
            ret += bin_formatter_o(format!("{:b}", 0)).as_str();
        }
        Instruction::JNZ(operand) => {
            ret += bin_formatter(format!("{:b}", 10)).as_str();
            ret += bin_formatter_o(format!("{:b}", operand)).as_str();
        }
        Instruction::HALT => {
            ret += bin_formatter(format!("{:b}", 11)).as_str();
            ret += bin_formatter_o(format!("{:b}", 0)).as_str();
        }
        Instruction::BLIND => {
            ret += bin_formatter(format!("{:b}", 12)).as_str();
            ret += bin_formatter_o(format!("{:b}", 0)).as_str();
        }
        Instruction::NEQ => {
            ret += bin_formatter(format!("{:b}", 13)).as_str();
            ret += bin_formatter_o(format!("{:b}", 0)).as_str();
        }
    }
    ret
}

fn make_inst(op: isize, operand: u64) -> Instruction {
    match op {
        0 => Instruction::PUSH(operand),
        1 => Instruction::ADD,
        2 => Instruction::SUB,
        3 => Instruction::MUL,
        4 => Instruction::DIV,
        5 => Instruction::DUP(operand), 
        6 => Instruction::DUMP,
        7 => Instruction::PRINT,
        8 => Instruction::JMP(operand),
        9 => Instruction::EQ,
        10 => Instruction::JNZ(operand),
        11 => Instruction::HALT,
        12 => Instruction::BLIND,
        13 => Instruction::NEQ,
        _ => {
            eprintln!("Could not make instruction with op code {} and operand {}", op, operand);
            exit(FEXT as i32);
        }
    }
}

impl VM {
    pub fn init() -> VM {
        VM {
            stack: [0; STACK_CAP],
            stack_size: 0,
            program: vec![],
            ptr: 0
        }
    }
    pub fn get_byte_code(&self) -> String {
        let mut ret = String::from("");
        for i in 0..self.program.len() {
            ret += inst_to_string(self.program[i]).as_str();
        }
        ret
    }
    pub fn write_to_file(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(self.get_byte_code().as_bytes())?;
        Ok(())
    }
    pub fn compile_source(path: &str, output: &str) -> std::io::Result<()> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        VM::unwrap(file.read_to_string(&mut buf), format!("Could not read file at path {}", path).as_str());
        let mut counter = 0;
        let mut opid: isize = 0;
        let mut oprn = 0u64;
        let mut ret = Vec::<Instruction>::new();
        for line in buf.lines() {
            let toks: Vec<&str> = line.split(' ').into_iter().collect();
            if toks.len() == 0 {
                continue;
            }
            match toks[0] {
                "dump"
                | "print"
                | "add"
                | "sub"
                | "mul"
                | "div"
                | "eq"
                | "neq"
                | "halt"
                | "blind" => { // no operands
                    match toks[0] { // add 1, sub 2, mul 3, div 4, dump 6, print 7, eq 9, halt 11
                        "dump" => {opid = 6}
                        "print" => {opid = 7}
                        "add" => {opid = 1}
                        "sub" => {opid = 2}
                        "mul" => {opid = 3}
                        "div" => {opid = 4}
                        "eq" => {opid = 9}
                        "neq" => {opid = 13}
                        "halt" => {opid = 11}
                        "blind" => {opid = 12}
                        _ => {}
                    }
                }
                "push"
                | "dup"
                | "jmp"
                | "jnz" => {// 0, dup 5, jmp 8, jnz 10
                    if toks.len() == 1 {
                        eprintln!("Invalid syntax at line {}.", counter);
                        exit(FEXT as i32);
                    }
                    let operand = match toks[1].parse::<u64>() {
                        Ok(i) => i,
                        Err(_e) => {
                            eprintln!("Invalid syntax at line {}.", counter);
                            exit(FEXT as i32);
                        }
                    };
                    match toks[0] {
                        "push" => {
                            opid = 0 as isize;
                            oprn = operand;
                        } 
                        "dup" => {
                            opid = 5 as isize;
                            oprn = operand;
                        } 
                        "jmp" => {
                            opid = 8 as isize;
                            oprn = operand;
                        } 
                        "jnz" => {
                            opid = 10 as isize;
                            oprn = operand;
                        }
                        _ => {}
                    }
                }
                _ => {
                    eprintln!("Invalid syntax at line {}.", counter);
                    exit(FEXT as i32);
                }
            }
            ret.push(make_inst(opid, oprn));
            counter += 1;
        }
        let mut retvm = VM::init(); 
        retvm.program = ret;
        VM::unwrap(retvm.write_to_file(output), "Could not write to output file specified.");
        Ok(())
    }
    pub fn load_from_file(&mut self, path: &str) -> std::io::Result<()> {
        lazy_static! {
            static ref RE: Regex = 
            Regex::new(
                r"(\d\d\d\d\d\d\d\d)(\d\d\d\d\d\d\d\d)(\d\d\d\d\d\d\d\d)(\d\d\d\d\d\d\d\d)(\d\d\d\d\d\d\d\d)(\d\d\d\d\d\d\d\d)(\d\d\d\d\d\d\d\d)(\d\d\d\d\d\d\d\d)(\d\d\d\d\d\d\d\d)")
                .unwrap();
        }
        let mut file = File::open(path)?;
        let mut buf = String::new();
        VM::unwrap(file.read_to_string(&mut buf), format!("Could not read file at path {}", path).as_str());
        let mut comp = Vec::<String>::new();
        for cap in RE.captures_iter(&buf) {
            comp.push(String::from(&cap[0]));
        }
        lazy_static! {
            static ref BYTE: Regex = Regex::new(r"(\d\d\d\d\d\d\d\d)").unwrap();
        }
        //println!("comp: {:#?}", comp);
        let mut inst_id = 0;
        let mut operand = 0u64;
        let mut ret = Vec::<Instruction>::new();
        for binst in comp {
            let mut o_comp = String::from("");
            let mut counter = 0;
            for byte in BYTE.captures_iter(&binst) {
                let byte_i = isize::from_str_radix(&byte[0], 2).unwrap();
                if counter == 0 {
                    inst_id = byte_i;
                } else if counter != 8 && counter != 0{
                    o_comp += format!("{}", &byte[0]).as_str();
                } else if counter == 8 {
                    o_comp += format!("{}", &byte[0]).as_str();
                    operand = isize::from_str_radix(&o_comp, 2).unwrap() as u64;
                }
                counter += 1;
            }
            ret.push(make_inst(inst_id, operand));     
        }
        self.load_program(ret);
        Ok(())
    }
    fn push(&mut self, o: Word) -> Result<InstSuccess, InstError> {
        let mut ret = Result::Ok(OK);
        self.stack_size += 1;
        if self.stack_size == STACK_CAP {
            ret = Err(InstError::StackOverflow(String::from("Stack overflow occurred.\n")));
            return ret;
        }
        self.stack[self.stack_size - 1] = o;
        ret
    }
    fn pop(&mut self) -> Result<Word, InstError> {
        if self.stack_size == 0 {
            Err(InstError::StackUnderflow("Can not pop from stack with 0 elements.\n".to_string()))
        } else {
            self.stack_size -= 1;
            Ok(self.stack[self.stack_size])
        }
    }
    fn last(&self) -> Result<Word, InstError> {
        if self.stack_size == 0 {
            Err(InstError::StackUnderflow("Can not get last from stack with 0 elements.\n".to_string()))
        } else {
            Ok(self.stack[self.stack_size - 1])
        }
    }
    fn unwrap<T, U> (r: std::result::Result<T, U>, on_err: &str) -> T 
        where U: core::fmt::Debug 
    {
        match r {
            Ok(t) => t,
            Err(e) => {
                println!("Error: {:#?} | {}", e, on_err); 
                exit(FEXT as i32);
            },
        }
    }

    pub fn load_program(&mut self, program: Vec<Instruction>) {
        for i in program {
            self.program.push(i);
        }
    }
    pub fn execute_instruction(&mut self,i: Instruction) -> Result<InstSuccess, InstError> {
        let mut ret = Result::Ok(OK);
        match i {
            Instruction::PUSH(operand) => {
                ret = self.push(operand);
                self.ptr += 1; 
            },

            Instruction::ADD => 
                if self.stack_size < 2 { return Err(InstError::StackUnderflow("Too little elements on the stack for ADD.\n".to_string())); } 
                else { 
                    self.stack[self.stack_size - 1] += VM::unwrap(self.pop(), "Could not pop from stack."); 
                    self.ptr += 1;  
                }
            
            Instruction::SUB => 
                if self.stack_size < 2 { return Err(InstError::StackUnderflow("Too little elements on the stack for ADD.".to_string())); } 
                else { self.stack[self.stack_size - 1] -= VM::unwrap(self.pop(), "Could not pop from stack."); self.ptr += 1; }
            
            Instruction::MUL => 
                if self.stack_size < 2 { return Err(InstError::StackUnderflow("Too little elements on the stack for ADD.".to_string())); } 
                else { self.stack[self.stack_size - 1] *= VM::unwrap(self.pop(), "Could not pop from stack."); self.ptr += 1; }
            
            Instruction::DIV => 
                if self.stack_size < 2 { return Err(InstError::StackUnderflow("Too little elements on the stack for ADD.".to_string())); } 
                else { 
                    let y = VM::unwrap(self.pop(), "Could not pop from stack");
                    if self.stack[self.stack_size - 1] == 0 || y == 0 {
                        return Err(InstError::DivByZero("Can not divide by zero.".to_string()));
                    }
                    self.stack[self.stack_size - 1] /= y;
                    self.ptr += 1; 
                }

            Instruction::DUMP => {
                println!("> STACK: ");
                for n in 0..self.stack_size {
                    println!("\t| {} : {}", n, self.stack[n]);
                }
                if self.stack_size == 0 {
                    println!("\t | EMPTY |");
                }
                self.ptr += 1;  
            }

            Instruction::JMP(operand) => {
                if operand as usize >= self.program.len() {
                    return Err(InstError::IllegalMemAccess(format!("Can not jump to line {}; can not access memory there.", operand)))
                }
                self.ptr = operand as usize;
            }
            Instruction::JNZ(operand) => {
                if operand as usize >= self.program.len() {
                    return Err(InstError::IllegalMemAccess(format!("Can not jump to line {}; can not access memory there.", operand)))
                } else if self.stack_size == 0 {
                    return Err(InstError::StackUnderflow("Can not JNZ with zero elements on stack.".to_string()));
                }
                if VM::unwrap(self.last(), "Could not get last member of stack in inst JNZ.") == 1 {
                    self.stack_size -= 1;
                    self.ptr = operand as usize;
                } else {
                    self.ptr += 1;
                }
            }
            Instruction::PRINT => {
                println!("{}", VM::unwrap(self.last(), "Unknwon error eccurred, could not print."));
                self.ptr += 1;
            }
            Instruction::HALT => {
                let ecode = VM::unwrap(self.pop(), "Could not pop from stack.") as i32;
                println!("\n\t> Program exited with code {}.", ecode);
                exit(ecode);
            }
            Instruction::EQ => {
                self.stack[self.stack_size - 2] =
                    (self.stack[self.stack_size - 2] == self.stack[self.stack_size - 1]) as Word;
                
                self.stack_size -= 1;
                self.ptr += 1;
            }
            Instruction::NEQ => {
                self.stack[self.stack_size - 2] =
                    (self.stack[self.stack_size - 2] != self.stack[self.stack_size - 1]) as Word;
                
                self.stack_size -= 1;
                self.ptr += 1;
            }
            Instruction::DUP(operand) => {
                let operand = operand as usize;
                if self.stack_size - operand <= 0 {
                    return Err(InstError::StackUnderflow(format!("Could not fetch value at {} from stack.", self.stack_size - operand)))
                }
                VM::unwrap(self.push(self.stack[self.stack_size - 1 - operand]), "Could not push duped value to stack.");
                self.ptr += 1;
            }
            Instruction::BLIND => {
                self.ptr += 1;
            }
        };
        ret
    }
    pub fn run_program(&mut self) {
        let mut i = 0;
        while i < self.program.len() {
            match self.execute_instruction(self.program[i]) {
                Ok(_) => {}
                Err(e) => {
                    println!("{:?}", e);
                    exit(MEXT as i32);
                }
            };
            i = self.ptr;
        }
    }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum Instruction {
    
    PUSH(Word),
    ADD,
    SUB,
    MUL,
    DIV,
    DUP(Word),
    DUMP,
    PRINT,
    JMP(Word),
    EQ,
    NEQ,
    JNZ(Word),
    HALT,
    BLIND
}


    

#[derive(Debug)]
pub enum InstSuccess {
    OK
}
#[derive(Debug)]
pub enum InstError {
    StackOverflow(String),
    StackUnderflow(String),
    DivByZero(String),
    IllegalMemAccess(String)
}
