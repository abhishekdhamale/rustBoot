#![no_std]
#![allow(warnings)]

#[cfg(feature = "nrf")]
pub mod nrf;

/// This is the main trait that abstracts the necessary HW-specific flash operations 
/// such as
/// - `writing to flash` - we can arbitrary blob of data to arbitrary location in flash
/// - `erasing a flash page` - erase a page of flash, given the address (i.e. first word) of the page 
/// to be erased and number of btyes to erase. 
pub trait FlashInterface {
    fn hal_init();
    fn hal_flash_unlock();
    fn hal_flash_lock();
    fn hal_flash_write(
        &self,
        addr: usize,
        data: *const u8,
        len: usize,
    );

    fn hal_flash_erase(
        &self,
        addr: usize,
        len: usize,
    );
}

// Arch-specific code
pub fn preboot() {}
pub fn boot_from(fw_base_address: usize) -> ! {
    #[cfg(feature = "nrf52840")]
    crate::nrf::nrf52840::boot_from(fw_base_address)
}