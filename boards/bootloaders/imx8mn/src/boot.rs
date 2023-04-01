//! Architectural boot code.

use core::arch::global_asm;

use crate::kernel_init;
use crate::{
    clocks, exception,
    mux::{uart2grp::uart2_mux_mmio_set, usdhc2grp::usdhc2_mux_mmio_set},
    start_system_counter,
};

// Assembly counterpart to this file.
global_asm!(include_str!("entry.S"));

/// The Rust entry of the `kernel` binary.
///
/// The function is called from the assembly `_start` function.
///
#[no_mangle]
pub unsafe extern "C" fn _start_rust() -> ! {
    // set the vector base address for register handlers
    exception::exception::handling_init();
    // enable Uart and uSDHC clock and ungate sys_counter clock
    clocks::uartclks::enable_uart_clk(1);
    clocks::usdhcclks::enable_usdhc_clk(2);
    clocks::scntrclk::enable_sctr();
    // start the system counter, this allows us to access ARM's architectural counter - CNTPCT_EL0
    start_system_counter();
    // set mux state for UART2 and uSDHC2 peripherals.
    uart2_mux_mmio_set();
    usdhc2_mux_mmio_set();
    // jump to next init stage.
    kernel_init()
}