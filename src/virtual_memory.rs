use super::error::ShellcodeRunnerError;
use std::{os::raw::c_void, ptr::NonNull, slice};
use windows::{
    Win32::System::{
        Memory::{
            MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_EXECUTE_READWRITE, VirtualAlloc, VirtualFree,
        },
        SystemInformation::{GetSystemInfo, SYSTEM_INFO},
    },
    core::Error as WindowsError,
};

pub struct VirtualMemory {
    ptr: NonNull<c_void>,
    size: usize,
}

impl VirtualMemory {
    pub fn alloc(size: usize) -> Result<Self, ShellcodeRunnerError> {
        let page = Self::page_size();
        let aligned = (size + (page - 1)) & !(page - 1);

        let ptr = unsafe {
            VirtualAlloc(
                None,
                aligned,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_EXECUTE_READWRITE,
            )
        };
        let Some(nn) = NonNull::new(ptr) else {
            return Err(ShellcodeRunnerError::WindowsError(
                WindowsError::from_win32(),
            ));
        };
        Ok(Self {
            ptr: nn,
            size: aligned,
        })
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr() as *mut u8, self.size) }
    }

    pub fn as_ptr(&self) -> *mut c_void {
        self.ptr.as_ptr()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    fn page_size() -> usize {
        let mut si = SYSTEM_INFO::default();
        unsafe { GetSystemInfo(&mut si) }
        si.dwPageSize as usize
    }
}

impl Drop for VirtualMemory {
    fn drop(&mut self) {
        let _ = unsafe { VirtualFree(self.ptr.as_ptr(), 0, MEM_RELEASE) };
    }
}
