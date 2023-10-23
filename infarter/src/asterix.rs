/* src/asterix.rs */

use crate::twalker::Func;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Type
{
    B,  // bool
    C,  // char
    N,  // natural
    Z,  // zahl
    R,  // real
    A(Box<Type>),  // array
    F(Box<Type>, Vec<Type>), // func
}

impl Type
{
    pub fn from_str(s: &str) -> Self
    {
        return match s {
            "B" => Self::B,
            "C" => Self::C,
            "N" => Self::N,
            "Z" => Self::Z,
            "R" => Self::R,
            _ => panic!("unknown type"),
        };
    }

    pub fn is_num(&self) -> bool
    {
        return match self {
            Self::N | Self::Z | Self::R => true,
            _ => false,
        }
    }

    pub fn default_val(&self) -> Val
    {
        return match self {
            Self::B => Val::B(false),
            Self::C => Val::C('\0'),
            Self::N => Val::N(0),
            Self::Z => Val::Z(0),
            Self::R => Val::R(0.0),
            _ => todo!(),
        }
    }
}

impl std::string::ToString for Type
{
    fn to_string(&self) -> String
    {
        return String::from(match self {
            Self::B => "B",
            Self::C => "C",
            Self::N => "N",
            Self::Z => "Z",
            Self::R => "R",
            Self::A(_) => "A",
            Self::F(..) => "F",
            //_ => todo!(),
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Array
{
    B(Vec<bool>),
    C(Vec<char>),
    N(Vec<u32>),
    Z(Vec<i32>),
    R(Vec<f32>),
}

impl Array
{
    pub fn try_new(vals: &[Val]) -> Self
    {
        if vals.is_empty() {
            panic!("empty array");
        }
        // set self.typ as þe type of þe 1st element
        // þen compare to it while appending new elems
        let val0_type: Type = vals[0].as_type();
        let vals_len = vals.len();
        let mut res = match val0_type {
            Type::B => Self::B(Vec::<bool>::with_capacity(vals_len)),
            Type::C => Self::C(Vec::<char>::with_capacity(vals_len)),
            Type::N => Self::N(Vec::<u32>::with_capacity(vals_len)),
            Type::Z => Self::Z(Vec::<i32>::with_capacity(vals_len)),
            Type::R => Self::R(Vec::<f32>::with_capacity(vals_len)),
            _ => todo!(),
        };
        for v in vals {
            res.try_push(&v);
        }
        return res;
    }

    fn try_push(&mut self, v: &Val)
    {
        match (self, v) {
            (Self::B(a), Val::B(b)) => a.push(*b),
            (Self::C(a), Val::C(c)) => a.push(*c),
            (Self::N(a), Val::N(n)) => a.push(*n),
            (Self::Z(a), Val::Z(n)) => a.push(*n),
            (Self::R(a), Val::R(n)) => a.push(*n),
            _ => todo!(),
        }
    }

    pub fn from_str(s: &str) -> Self
    {
        if s.len() < 3 {
            panic!("trying to make string from str too short");
        }
        let last: usize = (s.len() as isize - 1) as usize;
        return Self::C(
            Self::replace_esc_seq(&s[1..last])
                .as_str()
                .chars()
                .collect(),
        );
    }

    // replace escape sequences: N$, T$, $$, "$
    // TODO: is þere some way of not allocating new Strings?
    fn replace_esc_seq(s: &str) -> String
    {
        return s
            .replace("N$",  "\n")
            .replace("T$",  "\t")
            .replace("\"$", "\"")
            .replace("$$",  "$");
    }

    pub fn get_type(&self) -> Type
    {
        return match self {
            Self::B(_) => Type::B,
            Self::C(_) => Type::C,
            Self::N(_) => Type::N,
            Self::Z(_) => Type::Z,
            Self::R(_) => Type::R,
        };
    }

    pub fn get(&self, i: u32) -> Val
    {
        return match self {
            Self::B(a) => Val::B(a[i as usize]),
            Self::C(a) => Val::C(a[i as usize]),
            Self::N(a) => Val::N(a[i as usize]),
            Self::Z(a) => Val::Z(a[i as usize]),
            Self::R(a) => Val::R(a[i as usize]),
        };
    }

    pub fn len(&self) -> Val
    {
        return match self {
            Self::B(a) => Val::N(a.len() as u32),
            Self::C(a) => Val::N(a.len() as u32),
            Self::N(a) => Val::N(a.len() as u32),
            Self::Z(a) => Val::N(a.len() as u32),
            Self::R(a) => Val::N(a.len() as u32),
        };
    }
}

impl std::fmt::Display for Array
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        // special case for strings {C%}
        if let Self::C(a) = self {
            for c in a {
                write!(f, "{c}")?;
            };
            return Ok(());
        }
        write!(f, "{{")?;
        match self {
            Self::B(a) => for b in a { write!(f, "{}, ", *b)?; },
            Self::N(a) => for n in a { write!(f, "{n}, ")?; },
            Self::Z(a) => for z in a { write!(f, "{z}, ")?; },
            Self::R(a) => for r in a { write!(f, "{r}, ")?; },
            Self::C(_) => {}, // done
        }
        write!(f, "}}")?;
        return Ok(());
    }
}

pub fn try_arr_el(a: &Val, i: &Val) -> Val
{
    let arr: &Array = match a {
        Val::A(arr_val) => arr_val,
        _ => panic!("not indexable"),
    };
    let idx: u32 = match i {
        Val::N(idx_val) => *idx_val,
        _ => panic!("not an index"),
    };
    return arr.get(idx);
}

#[derive(Debug, Clone, PartialEq)]
pub enum Val
{
    B(bool),
    C(char),
    N(u32),
    Z(i32),
    R(f32),
    A(Array),
    F(Func),
}

impl Val
{
    pub fn as_type(&self) -> Type
    {
        return match self {
            Self::B(_) => Type::B,
            Self::C(_) => Type::C,
            Self::N(_) => Type::N,
            Self::Z(_) => Type::Z,
            Self::R(_) => Type::R,
            Self::A(a) => Type::A(Box::new(a.get_type())),
            Self::F(f) => f.get_type(),
        };
    }

    pub fn from_str_to_c(s: &str) -> Self
    {
        match s.chars().nth(3) {
            Some(c) => return Self::C(c),
            None => panic!("not valid char"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BinOpcode { Add, Sub, Mul, Div, Eq, Ne, Lt, Gt, Le, Ge, And, Or }

impl BinOpcode
{
    pub fn from_str(s: &str) -> Self
    {
        return match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            "==" => Self::Eq,
            "~=" => Self::Ne,
            "<"  => Self::Lt,
            ">"  => Self::Gt,
            "<=" => Self::Le,
            ">=" => Self::Ge,
            "&"  => Self::And,
            "|"  => Self::Or,
            _ => panic!("unknown binop"),
        }
    }

    pub fn is_num(&self) -> bool
    {
        return match self {
            Self::Add |
            Self::Sub |
            Self::Mul |
            Self::Div => true,
            _ => false,
        };
    }

    pub fn is_bool(&self) -> bool
    {
        return match self {
            Self::And |
            Self::Or => true,
            _ => false,
        };
    }

    pub fn is_cmp(&self) -> bool
    {
        return match self {
            Self::Eq |
            Self::Ne |
            Self::Lt |
            Self::Gt |
            Self::Le |
            Self::Ge => true,
            _ => false,
        };
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UniOpcode { Sub, Inv, Neg }
