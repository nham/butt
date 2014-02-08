#[crate_id = "radicle"];

//! A lisp interpreter.

pub use std::hashmap::HashMap;
pub use std::vec::MoveItems;
use std::str;
use std::io::File;
use std::os;

use tree::Tree;
pub use tree::Nil;
pub use Atom = tree::Leaf;
pub use List = tree::Branch;

use repl::do_repl;
use eval::eval;
use read::read;

pub mod tree;
pub mod repl;
pub mod eval;
pub mod read;
mod test;

fn main() {
    let globenv = Environment::new();

    let args = os::args();
    if args.len() == 1 {
        do_repl();
        return;
    } else if args.len() > 2 {
        println!("radicle: Only one argument allowed.");
        return;
    }

    if !"--test".equiv(&args[1]) {
        let fname = args[1].clone();
        let path = Path::new(args[1]);
        if path.is_file() {
            let mut hw_file = File::open(&path);
            let contents = hw_file.read_to_end();
            if contents.is_err() {
                println!("{}", contents.unwrap_err());
            } else {
                let data = str::from_utf8_owned(contents.unwrap());
                read_eval(data.unwrap(), &globenv);
            }
            return;
        } else {
            println!("radicle: can't open file {}", fname);
            return;
        }
    }

    /*
    read_eval("((lambda (x) (cons x (quote (ab cd)))) (quote CONSME))", &globenv);
    read_eval("((lambda (x y z) (cons y (cons z (cons x (quote (batman)))))) (quote CONSME) (quote santa) (car (quote (10 20 30))))", &globenv);
    read_eval("((lambduh (x) (cons x (quote ()))) (quote CONSME))", &globenv);
    read_eval("(((lambda (x) 
           (lambda (y) (cons x 
                             (cons y 
                                   (quote ()))))) 
   (quote 5)) (quote 6))", &globenv);
    read_eval(
"((label ZABBA (lambda (x) (cons x (quote (ab cd)))))
  (quote CONSME))", &globenv);

    read_eval(
"((label ZABBA (lambda (x y z) (cons y (cons z (cons x (quote (batman)))))))
  (quote CONSME) (quote santa) (car (quote (10 20 30))))", &globenv);


 */
    read_eval("(quote x) (quote y) (quote z)", &globenv);
}


/// A convenience function that calls read & eval and displays their results
pub fn read_eval(s: &str, env: &Environment) {
    let parsed = read(s);
    if parsed.is_ok() {
        println!("Parsed: {}", parsed);

        for expr in parsed.unwrap().move_iter() {
            match eval(expr, env) {
                Ok(x) => { println!("\nEvaled: {}", x); },
                Err(x) => { println!("\nEval error: {}", x); }
            }
        }
    } else {
        println!("\nParse error: {}", parsed.unwrap_err());
    }
}


/// The representation of Lisp expressions
pub type Expr = Tree<~str>;
pub type Exprs = ~[Expr];


pub struct Environment<'a> {
    parent: Option<&'a Environment<'a>>,
    bindings: HashMap<~str, Expr>,
}

impl<'a> Environment<'a> {
    fn new() -> Environment<'a> {
        Environment { parent: None, bindings: HashMap::new() }
    }

    fn find(&'a self, key: &~str) -> Option<&'a Expr> {
        if self.bindings.contains_key(key) {
            self.bindings.find(key)
        } else {
            if self.parent.is_some() {
                self.parent.unwrap().find(key)
            } else {
                None
            }
        }
    }

    fn find_copy(&self, key: &~str) -> Option<Expr> {
        if self.bindings.contains_key(key) {
            self.bindings.find_copy(key)
        } else {
            if self.parent.is_some() {
                self.parent.unwrap().find_copy(key)
            } else {
                None
            }
        }
    }
}

/// Wrapping the standard Tree methods for aesthetic reasons, I guess
impl ::tree::Tree<~str> {
    fn is_atom(&self) -> bool {
        self.is_leaf()
    }

    fn is_list(&self) -> bool {
        self.is_branch()
    }

    fn is_empty_list(&self) -> bool {
        self.eq(&List(~[]))
    }
}
