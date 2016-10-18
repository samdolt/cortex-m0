#![feature(lang_items)]
#![feature(asm)]

// We won't use the usual `main` function. We are going to use a different "entry point".
#![no_main]

// We won't use the standard library because it requires OS abstractions like threads and files and
// those are not available in this platform.
#![no_std]



mod exception {
    pub fn handler() -> ! {
        unsafe {
            asm!("bkpt");
        }

        loop {}
    }

    pub fn irq() {
        return;
    }

    pub fn irq0() {}
    pub fn irq1() {}
    pub fn irq2() {}
    pub fn irq3() {}
    pub fn irq4() {}
    pub fn irq5() {}
    pub fn irq6() {}
    pub fn irq7() {}
    pub fn irq8() {}
    pub fn irq9() {}
    pub fn irq10() {}
    pub fn irq11() {}
    pub fn irq12() {}
    pub fn irq13() {}
    pub fn irq14() {}
    pub fn irq15() {}
    pub fn irq16() {}
    pub fn irq17() {}
    pub fn irq18() {}
    pub fn irq19() {}
    pub fn irq20() {}
    pub fn irq21() {}
    pub fn irq22() {}
    pub fn irq23() {}
    pub fn irq24() {}
    pub fn irq25() {}
    pub fn irq26() {}
    pub fn irq27() {}
    pub fn irq28() {}
    pub fn irq29() {}
    pub fn irq30() {}
    pub fn irq31() {}
}

mod vector_table {
    #[link_section = ".reset"]
    static RESET: fn() -> ! = ::start; // 1 RESET

    #[link_section = ".exceptions"]
    static EXCEPTIONS: [Option<fn() -> !>; 14] = [Some(::exception::handler), // 2 NMI
                                                  Some(::exception::handler), // 3 Hard fault
                                                  None, // 4 Reserved
                                                  None, // 5 Reserved
                                                  None, // 6 Reserved
                                                  None, // 7 Reserved
                                                  None, // 8 Reserved
                                                  None, // 9 Reserved
                                                  None, // 10 Reserved
                                                  Some(::exception::handler), // 11 SVCall
                                                  None, // 12 Reserved
                                                  None, // 13 Reserved
                                                  Some(::exception::handler), // 14 PendSV
                                                  Some(::exception::handler)]; // 15 Systick

    #[link_section = ".irqs"]
    static IRQS: [Option<fn()>; 32] = [Some(::exception::irq0),
                                       Some(::exception::irq1),
                                       Some(::exception::irq2),
                                       Some(::exception::irq3),
                                       Some(::exception::irq4),
                                       Some(::exception::irq5),
                                       Some(::exception::irq6),
                                       Some(::exception::irq7),
                                       Some(::exception::irq8),
                                       Some(::exception::irq9),
                                       Some(::exception::irq10),
                                       Some(::exception::irq11),
                                       Some(::exception::irq12),
                                       Some(::exception::irq13),
                                       Some(::exception::irq14),
                                       Some(::exception::irq15),
                                       Some(::exception::irq16),
                                       Some(::exception::irq17),
                                       Some(::exception::irq18),
                                       Some(::exception::irq19),
                                       Some(::exception::irq20),
                                       Some(::exception::irq21),
                                       Some(::exception::irq22),
                                       Some(::exception::irq23),
                                       Some(::exception::irq24),
                                       Some(::exception::irq25),
                                       Some(::exception::irq26),
                                       Some(::exception::irq27),
                                       Some(::exception::irq28),
                                       Some(::exception::irq29),
                                       Some(::exception::irq30),
                                       Some(::exception::irq31)];
}




#[no_mangle]
pub fn start() -> ! {
    turn_on_gpioc();
    put_pc8_in_output_mode();

    let mut ticks = 100_000;
    loop {
        set_pc8_high();
        delay(ticks);
        set_pc8_low();
        delay(ticks);
    }
}

fn delay(n: u32) {
    for _ in 0..n {}
}


fn turn_on_gpioc() {
    /// Start address of the RCC register block
    const RCC: u32 = 0x4002_1000;


    const RCC_AHBENR: u32 = 0x14;

    /// IOPCEN bit mask
    const RCC_IOPCEN: u32 = 1 << 19;

    unsafe {
        // Pointer to the APB2ENR register
        let ahbenr = (RCC + RCC_AHBENR) as *mut u32;

        // IOPECN = 1
        *ahbenr |= RCC_IOPCEN;
    }
}

/// Start address of the GPIOC register block
const GPIOC: u32 = 0x4800_0800;

/// Offset address of the BSRR register
const GPIOC_BSRR: u32 = 0x18;

fn put_pc8_in_output_mode() {
    /// Offset address of the CRH register
    const GPIOC_MODER: u32 = 0x00;

    unsafe {
        // Pointer to the CRH register
        let moder = (GPIOC + GPIOC_MODER) as *mut u32;

        // CNF8 = 0b00, MODE8 = 0b10
        *moder = 0b01 << 16;
    }
}

fn set_pc8_high() {
    unsafe {
        // Pointer to the BSRR register
        let bsrr = (GPIOC + GPIOC_BSRR) as *mut u32;

        // BS8 = 1
        *bsrr = 1 << 8;
    }
}

fn set_pc8_low() {
    unsafe {
        // Pointer to the BSRR register
        let bsrr = (GPIOC + GPIOC_BSRR) as *mut u32;

        // BR8 = 1
        *bsrr = 1 << (16 + 8);
    }
}

// Finally, we need to define the panic_fmt "lang item", which is just a function. This specifies
// what the program should do when a `panic!` occurs. Our program won't panic, so we can leave the
// function body empty for now.
mod lang_items {
    #[lang = "panic_fmt"]
    extern "C" fn panic_fmt() {}
}
