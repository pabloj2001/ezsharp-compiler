use crate::lexical_analysis::Token;
use crate::logger::Loggable;

#[derive(Debug, Clone)]
pub enum TacCommand {
    BeginFunc,
    EndFunc,
    PushParam,
    PopParams,
    LCall,
    IfZ,
    Goto,
    Return,
}

impl Loggable for TacCommand {
    fn to_log_message(&self) -> String {
        match self {
            TacCommand::BeginFunc => {
                String::from("BeginFunc")
            },
            TacCommand::EndFunc => {
                String::from("EndFunc")
            },
            TacCommand::PushParam => {
                String::from("PushParam")
            },
            TacCommand::PopParams => {
                String::from("PopParams")
            },
            TacCommand::LCall => {
                String::from("LCall")
            },
            TacCommand::IfZ => {
                String::from("IfZ")
            },
            TacCommand::Goto => {
                String::from("Goto")
            },
            TacCommand::Return => {
                String::from("Return")
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum TacValue {
    Label(String),
    Var(String),
    Int(u32),
    Double(f64),
    PointerAccess(String, Box<TacValue>),
    GetParams(u32),
    LCallArgs(String),
    IfArgs(String, String),
}

impl Loggable for TacValue {
    fn to_log_message(&self) -> String {
        match self {
            TacValue::Label(val) | TacValue::Var(val) => {
                val.clone()
            },
            TacValue::Int(int) => {
                int.to_string()
            },
            TacValue::Double(double) => {
                double.to_string()
            },
            TacValue::PointerAccess(arr, index) => {
                format!("*({} + {})", arr, index.to_log_message())
            },
            TacValue::GetParams(size) => {
                format!("GetParams {}", size)
            },
            TacValue::LCallArgs(func) => {
                format!("LCall {}", func)
            },
            TacValue::IfArgs(cond, label) => {
                format!("{} Goto {}", cond, label)
            },
        }
    }

}

#[derive(Debug, Clone)]
pub struct TacOperation {
    pub op: Option<Token>,
    pub val1: TacValue,
    pub val2: Option<TacValue>,
}

impl Loggable for TacOperation {
    fn to_log_message(&self) -> String {
        let mut log_message = self.val1.to_log_message();
        if let Some(token) = &self.op {
            log_message.push_str(&format!(" {} {}", token.to_string(), self.val2.as_ref().unwrap().to_log_message()));
        }
        log_message
    }
}

#[derive(Debug, Clone)]
pub enum TacStatement {
    Label(String),
    Assignment(String, TacOperation),
    PointerAssignment(String, TacValue, TacOperation),
    Command(TacCommand, Option<TacValue>),
}

impl Loggable for TacStatement {
    fn to_log_message(&self) -> String {
        match self {
            TacStatement::Label(label) => {
                format!("{}:\n", label)
            },
            TacStatement::Assignment(var, op) => {
                format!("\t{} = {};\n", var, op.to_log_message())
            },
            TacStatement::PointerAssignment(arr, index, op) => {
                format!("\t*({} + {}) = {};\n", arr, index.to_log_message(), op.to_log_message())
            },
            TacStatement::Command(command, val) => {
                match val {
                    Some(val) => {
                        format!("\t{} {};\n", command.to_log_message(), val.to_log_message())
                    },
                    None => {
                        format!("\t{};\n", command.to_log_message())
                    },
                }
            },
        }
    }
}

pub type TacProgram = Vec<TacStatement>;

impl Loggable for TacProgram {
    fn to_log_message(&self) -> String {
        let mut log_message = String::new();
        for statement in self.iter() {
            log_message.push_str(statement.to_log_message().as_str());
        }
        log_message
    }
}