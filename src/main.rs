#![feature(asm_experimental_arch)]
#![no_std]
#![no_main]

use core::arch::asm;

#[inline]
pub fn c_write(c: u8) {
    // "naive" syscall wrapper
    // This *should* work, and should give the compiler flexibility
    // to apply different optimizations in different situations!
    unsafe {asm!(
        // CP/M places a general syscall function at address 0x0005
        "call 5",
        // Syscall number in c (2 for "console write")
        in("c") 2u8,
        // Argument in e
        in("e") c,
        // CP/M syscalls tend to trash the rest of the registers;
        // let's let rustc know
        out("b") _,
        out("d") _,
        out("hl") _,
        out("a") _,
        // In fact, we can't even trust the input registers
        lateout("c") _,
        lateout("e") _,
    );}
}

#[inline(never)]
fn my_print(chars: &[u8]) {
    // Simple print function.
    // inline(never) keeps generated code tidy for this example
    for &c in chars {
        c_write(c);
    }
}

#[inline]
pub fn c_write_pushpop(c: u8) {
    // The same syscall wrapper, but we manually push registers
    // to the stack every time.
    // This is not a good solution! This wastes time in cases where
    // we don't have anything useful in any registers.
    // However, it still somehow leads to better loop code than what
    // rustc generates for the above "c_write" function.
    unsafe {asm!(
        "
        push af
        push bc
        push hl
        ld c, 2
        call 5
        pop hl
        pop bc
        pop af
        ",
        in("e") c,
        lateout("e") _,
        out("d") _,
    );}
}

#[inline(never)]
fn my_print_pushpop(chars: &[u8]) {
    for &c in chars {
        c_write_pushpop(c);
    }
}

#[inline(never)]
pub fn my_print_handmade(c: &[u8]) {
    // Handmade print implementation using the stack.
    // Still not as good as static frame allocation, but I was
    // hoping rustc would generate something like this.
    unsafe {asm!(
        "
        push af
        push bc
        2:
            push de
            ld e, (hl)
            push hl
            ld c, 2
            call 5
            pop hl
            pop de
            inc hl
            dec de
            ld a, e
            or d
            jr nz, 2b
        pop bc
        pop af
        ",
        in("hl") c.as_ptr(),
        in("de") c.len() as u8 as u16
    );}
}

#[inline]
pub fn c_write_shadow(c: u8) {
    // Syscall wrapper that uses shadow registers.
    // This only works if they aren't used by ISRs or CP/M itself!
    // In practice, it works great -- but it can't be the only
    // available implementation.
    unsafe {asm!(
        "exx",
        "ld c, 2",
        "ld e, a",
        "call 5",
        "exx",
        in("a") c,
        // a itself may have been modified by the syscall
        lateout("a") _,
    );}
}

#[inline(never)]
fn my_print_shadow(chars: &[u8]) {
    for &c in chars {
        c_write_shadow(c);
    }
}

fn main() {
    let test_str = b"Hello, World!\r\n";
    my_print_shadow(test_str);
    my_print_handmade(test_str);
    my_print_pushpop(test_str);
    my_print(test_str);
}

#[unsafe(link_section = ".text.init")]
#[unsafe(naked)]
#[unsafe(no_mangle)]
unsafe extern "C" fn start() -> ! {
    // Handwritten init code.
    // Sets the stack pointer to the highest address
    // not used by the application or CP/M itself.
    core::arch::naked_asm!(
        "
        ld hl, (0x0006)
        dec hl
        ld sp, hl
        call {main}
        rst 0
        ",
        main = sym main,
    );
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop{}
}
