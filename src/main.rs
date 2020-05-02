struct VM{
    pub stack: Vec<i32>
}

impl VM{
    pub fn pop(&mut self) -> i32{
        self.stack.pop().unwrap()
    }

    pub fn run(&mut self, program: Vec<Opcode>){
        for code in program {
            match code{
                Opcode::Push(x) => self.stack.push(x),
                Opcode::Add => {
                    let result = self.pop() + self.pop();
                    self.stack.push(result);
                }
                Opcode::Sub => {
                    let result = self.pop() - self.pop();
                    self.stack.push(result);
                }
                Opcode::Print => println!("{}", self.stack.last().unwrap())
            }
        }
    }
}

enum Opcode {
      Push(i32),
      Add,
      Sub,
      Print
}

fn main() {
    let program = vec![
        Opcode::Push(30),
        Opcode::Push(2),
        Opcode::Add,
        Opcode::Print,
        Opcode::Push(50),
        Opcode::Sub,
        Opcode::Print,
        Opcode::Push(50),
        Opcode::Push(50),
        Opcode::Push(50),
        Opcode::Add,
        Opcode::Add,
        Opcode::Sub,
        Opcode::Print,
        ];
        
        /* PRINTS:
        32,
        18,
        132
        */

    let mut vm: VM = VM{stack: Vec::new()};
    vm.run(program);
}
