use std::process;

pub fn exit(msg: &str) {
    eprintln!("{}", msg);
    process::exit(1);
}

#[macro_export]
macro_rules! function_name {
    ($function_name:ident) => {
        $function_name
    };
}
