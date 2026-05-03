#![forbid(unsafe_op_in_unsafe_fn)]

use core::slice;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct KudzuSliceU8 {
    pub data: *const u8,
    pub len: usize,
}

impl KudzuSliceU8 {
    /// # Safety
    ///
    /// The caller must ensure `data` points to `len` bytes that remain valid
    /// for the duration of the returned borrow.
    pub unsafe fn as_slice<'a>(self) -> &'a [u8] {
        if self.len == 0 {
            &[]
        } else {
            unsafe { slice::from_raw_parts(self.data, self.len) }
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KudzuStatusCode {
    Ok = 0,
    Error = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct KudzuStringView {
    pub data: *const u8,
    pub len: usize,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct KudzuStatus {
    pub code: KudzuStatusCode,
    pub message: KudzuStringView,
}

impl KudzuStatus {
    pub const fn ok() -> Self {
        Self {
            code: KudzuStatusCode::Ok,
            message: KudzuStringView {
                data: core::ptr::null(),
                len: 0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::KudzuSliceU8;

    #[test]
    fn zero_len_slice_is_empty() {
        let slice = unsafe {
            KudzuSliceU8 {
                data: core::ptr::null(),
                len: 0,
            }
            .as_slice()
        };
        assert!(slice.is_empty());
    }
}

