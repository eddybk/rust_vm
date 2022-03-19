mod vm;
#[path ="./utils/list.rs"]
mod list;
#[path ="./utils/string.rs"]
mod string;
#[path ="./utils/io.rs"]
mod io;

use crate::{vm::VM, list::{AsList, Tail, Head}, string::StringView, io::cout};

#[allow(unused_must_use)]

fn main() -> std::io::Result<()> {
    let list = vec![1,2,3,4,5,6].as_list();
    let mut vm = VM::init();
    cout![]
    % String::from("hello").as_list()
    | "\n";
    
    /*vm.load_program(vec![
        Instruction::PUSH(1),
        Instruction::DUMP
    ]);

    vm.write_to_file("foo.ekvm")?;*/
    //vm.load_from_file("foo.ekvm")?;
    VM::compile_source("foo.vm", "foo.ekvm")?;
    vm.load_from_file("foo.ekvm")?;
    vm.run_program();
    Ok(())
}