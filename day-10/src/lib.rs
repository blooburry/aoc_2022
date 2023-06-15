use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct Sprite {
    pub cycle: usize,
    pub pending: Operation,
    pub eta: u32,
    pub position: i32,
    pub future_operations: Vec<Operation>,
}

#[derive(Clone, Debug)]
pub enum Operation {
    Noop,
    Add(i32),
}

impl Operation {
    pub fn build(s: &str) -> Result<Self, Error> {
        if s == "noop" {Ok(Self::Noop)}
        else if s.starts_with("add"){
            let v = s.split(" ").skip(1).next().unwrap()
                .parse::<i32>()
                .or_else(|_| Err(Error::new(ErrorKind::InvalidData, "couldn't parse int")))?;
            Ok(Self::Add(v))
        } else {
            Err(Error::new(ErrorKind::InvalidData, "unrecognised operation"))
        }
    }
}

impl Sprite {
    pub fn tick(&mut self) -> Result<i32, Error> {
        self.cycle += 1;

        let res = self.position;

        if self.eta > 0 {
            self.eta -= 1;

            if self.eta == 0 {
                self.execute_operation()?;
            }
        }

        Ok(res)
    }

    fn execute_operation(&mut self) -> Result<(), Error>{
        match self.pending {
            Operation::Noop => {}
            Operation::Add(v) => {self.position += v;}
        }

        self.pending = self.future_operations.pop()
            .ok_or(Error::new(ErrorKind::InvalidData, "ran out of operations"))?;

        self.eta = match self.pending {
            Operation::Noop => 1,
            Operation::Add(_) => 2,
        };

        Ok(())
    }


    pub fn new(mut future_operations: Vec<Operation>) -> Self {
        future_operations.reverse();
        let pending_operation = future_operations.pop().unwrap();
        let time_left_until_execution = match pending_operation {
            Operation::Noop => 1,
            Operation::Add(_) => 2,
        };
        Self { cycle: 0,
            pending: pending_operation,
            eta: time_left_until_execution + 1,
            position: 1,
            future_operations }
    }
}