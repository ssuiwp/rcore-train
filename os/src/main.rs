#![no_std]
#![no_main]
//fn main() {
//    println!("Hello, world!");
//}

use core::fmt::{self, Write};

mod lang_items;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id,
        )
    }
    ret
}

const SYSCALL_EXIT: usize = 93;

pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}
const SYSCALL_WRITE: usize = 64;
pub fn sys_write(fd:usize, buffer:&[u8])->isize{
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}
struct Stdout;
impl fmt::Write for Stdout{
    fn write_str(&mut self, s: &str)->fmt::Result{
        sys_write(1, s.as_bytes());
        Ok(())
    }
}
pub fn print(args: fmt::Arguments){
    Stdout.write_fmt(args).unwrap()
}
#[macro_export]
macro_rules! print{
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $(arg)+)?));
    }
}
#[macro_export]
macro_rules! println{
    ($fmt: literal $(, $($args: tt)+)?) => {
        print(format_args!(concat!($fmt, "\n") $(, $($args)+)?));
    }
}

#[no_mangle]
extern "C" fn _start() {
    println!("hello, world!");
    sys_exit(9);
}
