#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[defmt_test::tests]
mod tests {
    use demt::assert;

    #[test]
    fn always_passes() {
        assert!(true);
    }
}
