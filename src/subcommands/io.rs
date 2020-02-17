use std::collections::HashMap;
use std::fmt;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Arg {
    pub exchange: String,
    pub name: String,
    pub secret: Option<String>,
}

type Exchange = String;
type Name = String;
type Secret = String;
pub type Data = HashMap<Exchange, HashMap<Name, Secret>>;

#[derive(Debug)]
pub enum Rtn {
    Empty,
    Code { code: String },
    Secret { secret: String },
    Single { exchange: String, name: String },
    Multiple { data: Box<Vec<Rtn>> },
}

impl fmt::Display for Rtn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Rtn::Empty => write!(f, ""),
            Rtn::Code { code } => write!(f, "{}", code),
            Rtn::Secret { .. } => write!(f, "$$$"),
            Rtn::Single { exchange, name } => write!(f, "{} -> {}", exchange, name),
            Rtn::Multiple { data } => {
                for rtn in (*data).iter() {
                    rtn.fmt(f).unwrap();
                    write!(f, "\n").unwrap();
                }
                Ok(())
            }
        }
    }
}
