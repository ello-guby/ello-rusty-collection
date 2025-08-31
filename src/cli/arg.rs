use std::{
    collections::HashMap,
    fmt,
};

pub struct ArgProcessor {
    opts: HashMap<String, ArgOpt>,
}

impl ArgProcessor {
    /**
     * Instantiate a new ArgProcessor
     */
    pub fn new() -> ArgProcessor {
        return ArgProcessor { opts: HashMap::new() }
    }
    /**
     * Instantiate a new ArgProcessor with multiple ArgOpt
     */
    pub fn new_with_opts(opts: Vec<ArgOpt>) -> ArgProcessor {
        let mut a = ArgProcessor::new();
        for opt in opts {
            a.add_opt(opt);
        }
        a
    }
    /**
     * Instantiate a new ArgProcessor with multiple ArgOpt opt string.
     */
    pub fn new_with_opts_str(opts_str: Vec<&str>) -> ArgProcessor {
        let mut opts = vec![];
        for opt in opts_str {
            opts.push(ArgOpt::new(opt));
        }
        ArgProcessor::new_with_opts(opts)
    }

    /**
     * Add an opt into as string.
     */
    pub fn add_opt_str(&mut self, opt_str: &str) {
        self.add_opt(ArgOpt::new(opt_str));
    }
    /**
     * Add an opt into.
     */
    pub fn add_opt(&mut self, opt: ArgOpt) {
        match self.opts.get(&opt.long) {
            Some(v) => {
                panic!("'{v:?}' and '${opt:?}' have the same long.");
            }
            None => {
                self.opts.insert(opt.long.to_string(), opt);
            }
        }
    }
    
    /**
     * Return copies of opts in this.
     */
    pub fn opts(&self) -> Vec<ArgOpt> {
        let mut opts = vec![];
        for opt in self.opts.values() {
            opts.push(ArgOpt::new_processed(opt.to_string().as_str(), opt.process));
        }
        opts
    }
    /**
     * Get a copy of opt.
     * Panic if the long didn't exist.
     */
    pub fn opt(&self, long: &str) -> ArgOpt {
        let opt = match self.opts.get(long) {
            Some(v) => v,
            None => panic!("there are no opt with the long of '${long}' in '${self}'"),
        };
        ArgOpt::new_processed(opt.to_string().as_str(), opt.process)
    }

    /**
     * Process input_args.
     * Return non processed arguments.
     */
    pub fn process(&mut self, input_args: Vec<String>) -> Vec<String> {
        let mut output_args = vec![];
        'arg: for arg in input_args {
            let mut proc_arg = arg.clone();
            let long = if arg.starts_with("--") {
                proc_arg = proc_arg.split_off(2);
                true
            } else if arg.starts_with("-") {
                proc_arg = proc_arg.split_off(1);
                false
            } else {
                output_args.push(arg);
                continue 'arg;
            };
            
            'opt: for (key, opt) in &mut self.opts {
                if long {
                    if key.to_string() == proc_arg {
                        opt.process = Some(true);
                        continue 'arg;
                    }
                } else {
                    let short = match opt.short {
                        Some(s) => s,
                        None => { break 'opt; },
                    };
                    if short.to_string() == proc_arg {
                        opt.process = Some(true);
                        continue 'arg;
                    }
                }
            }
        }
        for opt in self.opts.values_mut() {
            if opt.process.is_none() {
                opt.process = Some(false);
            }
        }
        output_args
    }
}

impl fmt::Display for ArgProcessor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut opts_vec = vec![];
        for (_k, opt) in self.opts.iter() {
            opts_vec.push(opt.to_string());
        }
        write!(f, "ArgProcessor({})", opts_vec.join(", "))
    }
}

pub struct ArgOpt {
    short: Option<char>,
    long: String,
    process: Option<bool>,
}

impl ArgOpt {
    /**
     * Create new Argument Option with option string ("h/help").
     */
    pub fn new(opt_str: &str) -> ArgOpt {
        let (short, long) = ArgOpt::parse(opt_str);
        ArgOpt { short: short, long: long, process: None }
    }
    fn new_processed(opt_str: &str, process: Option<bool>) -> ArgOpt {
        let (short, long) = ArgOpt::parse(opt_str);
        ArgOpt { short: short, long: long, process: process }
    }

    /**
     * Create generic help Argument Option.
     */
    pub fn help() -> ArgOpt {
        ArgOpt::new("h/help")
    }

    /**
     * Parse option string ("h/help").
     */
    pub fn parse(opt_str: &str) -> (Option<char>, String) {
        let mut short = None;
        let mut long = String::new();
        let mut is_short = true;
        for ch in opt_str.chars() {
            if ch == '/' { is_short = false; continue; }
            if !(ch.is_ascii_alphabetic() || ch.is_ascii_digit()) {
                panic!("'{ch:?}' in '{opt_str:?}' is not a digit nor alphabetic.");
            }
            if is_short {
                if short.is_some() {
                    panic!("The short in '{opt_str:?}' have more then a characthers.");
                }
                short = Some(ch);
                continue;
            }
            long.push(ch);
        }
        if long.is_empty() { panic!("'{opt_str:?}' have no long."); }
        (short, long)
    }
    
    /**
     * Get if the ArgOpt is "proc" by ArgProcessor.process().
     * return Some(true) - if it is "proc" by ArgProcessor.process(),
     * return Some(false) - if it did not get "proc" by ArgProcessor.process(),
     * return None - if the ArgOpt has not been through ArgProcessor.process().
     */
    pub fn processed(&self) -> bool {
        match self.process {
            Some(v) => v,
            None => panic!("'${self:?}' did not get processed yet."),
        }
    }
}

impl fmt::Display for ArgOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let short = match self.short {
            Some(s) => s.to_string(),
            None => String::new(),
        };
        write!(f, "{}/{}", short, self.long)
    }
}
impl fmt::Debug for ArgOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ArgOpt('{}')", self.to_string())
    }
}
