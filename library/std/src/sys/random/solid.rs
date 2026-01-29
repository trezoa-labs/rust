use crate::sys::pal::abi;

pub fn fill_bytes(bytes: &mut [u8]) {
    unsafe {
        let result = abi::TRZID_RNG_SampleRandomBytes(bytes.as_mut_ptr(), bytes.len());
        assert_eq!(result, 0, "failed to generate random data");
    }
}
