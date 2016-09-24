/// Return the value of the register specified by `$name`.
macro_rules! reg{
    ( $name:expr ) => {{
        let val: u64;
        asm!(concat!("mov $0, ", $name) : "=r"(val));
        val
    }}
}

/// Return the value of the system register specified by `$name`.
macro_rules! sysreg{
    ( $name:expr ) => {{
        let val: u64;
        asm!(concat!("mrs $0, ", $name) : "=r"(val));
        val
    }}
}

/// Set the system register specified by `$name` to `$val`.
macro_rules! set_sysreg{
    ( $name:expr, $val:expr ) => {{
        asm!(concat!("msr ", $name, ", $0") :: "r"($val));
    }}
}
