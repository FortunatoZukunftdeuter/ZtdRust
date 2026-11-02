use std::io::{
    self,
    Write
};

pub struct Debugger {
    pub sholl: bool,
}

impl Debugger {
    /* Print Kern Function */
        fn conditional_print(&self, msg: &str, newline: bool, ender: &str) -> &Self {
            if self.sholl {
            
                if newline {
                    println!("{msg}{ender}");
                } else {
                    print!("{msg}{ender}");
                }
            }
        return self;}

    /* Print */
        // Raw
        pub fn print<AsStr: AsRef<str>>(&self, msg: AsStr) -> &Self {
            self.conditional_print(msg.as_ref(), false, "")
        }
        // Dotted
        pub fn printd<AsStr: AsRef<str>>(&self, msg: AsStr) -> &Self {
            self.conditional_print(msg.as_ref(), false, ".")
        }
        // Spaced 
        pub fn prints<AsStr: AsRef<str>>(&self, msg: AsStr) -> &Self {
            self.conditional_print(msg.as_ref(), false, " ")
        }
        // Dotted and Spaced
        pub fn printds<AsStr: AsRef<str>>(&self, msg: AsStr) -> &Self {
            self.conditional_print(msg.as_ref(), false, ". ")
        }

    /* Print New Line */
        // Raw
        pub fn println<AsStr: AsRef<str>>(&self, msg: AsStr) -> &Self {
            self.conditional_print(msg.as_ref(), true, "")
        }
        // Dotted
        pub fn printdln<AsStr: AsRef<str>>(&self, msg: AsStr) -> &Self {
            self.conditional_print(msg.as_ref(), true, ".")
        }
    /* Flush */
        pub fn flush(&self) -> io::Result<&Self> {
            if self.sholl {
                io::stdout().flush()?;
            }

            Ok(self)
        }

    /* Executer */
        pub fn run_and_rtn<Fnc, T>(&self, fnc: Fnc) -> Option<T>
        where Fnc: FnOnce() -> T
        {
            if !self.sholl {
                return None;
            }

            return Some(fnc());
        }
        pub fn run<Fnc>(&self, fnc: Fnc) -> &Self
        where Fnc: FnOnce()
        {
            if self.sholl {
                fnc();
            }

            self
        }
}

impl From<bool> for Debugger {
    fn from(value: bool) -> Self {
        Self {
            sholl: value
        }
    }
}
