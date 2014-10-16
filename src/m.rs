use libc::{c_double, c_int};

#[link_name = "m"]
extern {
    pub fn erf(x: c_double) -> c_double;
    pub fn erfc(x: c_double) -> c_double;
    pub fn exp(x: c_double) -> c_double;
    pub fn lgamma_r(x: c_double, sign: &mut c_int) -> c_double;
    pub fn log(x: c_double) -> c_double;
    pub fn pow(x: c_double, y: c_double) -> c_double;
    pub fn sqrt(x: c_double) -> c_double;
}