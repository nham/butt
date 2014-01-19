use std::fmt::{Default, Formatter};
use std::from_str::from_str;
use std::str::{eq_slice};
use std::option::{Option};

fn main() {
    let x = eval( Leaf(~"-3.14159") );
    println!("{}", x);

    let y = eval( Leaf(~"'z'") );
    println!("{}", y);

    println!("uhhhhhh {}", BVector(~[BPair( ~Cons(BNumber(3.14159), ~Nil) ), 
                                     BChar('z'), BBoolean(true)]));

}

enum Expression {
    Leaf(~str),
    Node(~[~Expression])
}

// inspired by 3.4, disjointness of types
// see also 6.3 for discussion of the empty list, called BNil here
enum BValue {
    BBoolean(bool),
    BSymbol(~str),
    BChar(char),
    BNumber(f64),
    BString(~str),
    BVector(~[BValue]),
    //BProcedure
    BPair(~List<BValue>),
}

enum List<T> {
    Cons(T, ~List<T>),
    Nil
}

impl<T: Default> Default for List<T> {
    fn fmt(v: &List<T>, f: &mut Formatter) {
        match *v {
            Cons(ref v, ref l) => write!(f.buf, "({} : {})", *v, **l),
            Nil => write!(f.buf, "()")
        }
    }
}

impl Default for BValue {
    fn fmt(v: &BValue, f: &mut Formatter) {
        match *v {
            BBoolean(b)    => write!(f.buf, "BBoolean({})", b),
            BSymbol(ref s) => write!(f.buf, "BSymbol({})", *s),
            BChar(c)       => write!(f.buf, "BChar({})", c),
            BNumber(n)     => write!(f.buf, "BNumber({})", n),
            BString(ref s) => write!(f.buf, "BNumber({})", *s),
            BVector(ref v) => write!(f.buf, "BVector({})", *v),
            BPair(ref p) => write!(f.buf, "BPair({})", **p)
        }
    }
}

impl Default for ~[BValue] {
    fn fmt(v: &~[BValue], f: &mut Formatter) {
        write!(f.buf, "[");

        for x in v.iter() {
            write!(f.buf, " {}", *x);

        }

        write!(f.buf, " ]");
    }
}

fn eval(expr: Expression) -> Result<BValue, ~str> {
    match expr {
        Leaf(x) => {
            match parse_bool(x) {
                Some(b) => Ok( BBoolean(b) ),
                None    => 
                    match parse_char(x) {
                        Some(c) => Ok( BChar(c) ),
                        None    =>
                            match parse_num(x) {
                               Some(n) => Ok( BNumber(n) ),
                               None    => Ok( BSymbol(x) )
                            }
                    }
            }
        },
        _       => Err(~"not implemented")
    }
}


fn parse_bool(s: &str) -> Option<bool> {
    if eq_slice(s, "#t") {
        Some(true)
    } else if eq_slice(s, "#f") {
        Some(false)
    } else {
        None
    }
}

fn parse_num(s: &str) -> Option<f64> {
    from_str::<f64>(s)
}

fn parse_char(s: &str) -> Option<char> {
    let x: ~[char] = s.chars().collect();
    if x.len() == 3 && x[0] == '\'' && x[2] == '\'' {
        Some(x[1])
    } else {
        None
    }

}

#[test]
fn test_is_num() {
    assert!(is_num("9"));
    assert!(is_num("0"));
    assert!(is_num("458915"));
    assert!(is_num("0000009999"));
}
