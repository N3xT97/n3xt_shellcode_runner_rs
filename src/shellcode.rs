use crate::{error::ShellcodeRunnerError, virtual_memory::VirtualMemory};
use std::{os::raw::c_void, sync::Arc};

pub struct Unloaded;
pub struct Loaded {
    vm: VirtualMemory,
}

pub struct Shellcode<S> {
    bytes: Arc<[u8]>,
    start_offset: usize,
    state: S,
}

impl<S> Shellcode<S> {
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Shellcode<Unloaded> {
    pub fn new<B: AsRef<[u8]>>(code: B, start_offset: usize) -> Result<Self, ShellcodeRunnerError> {
        let shellcode_len = code.as_ref().len();
        if shellcode_len <= start_offset {
            return Err(ShellcodeRunnerError::InvalidOffset {
                offset: start_offset,
                len: shellcode_len,
            });
        }
        Ok(Self {
            bytes: Arc::from(code.as_ref()),
            start_offset,
            state: Unloaded,
        })
    }

    pub fn load(self) -> Result<Shellcode<Loaded>, ShellcodeRunnerError> {
        let shellcode_size = self.bytes.len();
        self.load_with_capacity(shellcode_size)
    }

    pub fn load_with_capacity(
        self,
        capacity: usize,
    ) -> Result<Shellcode<Loaded>, ShellcodeRunnerError> {
        let n = self.bytes.len();
        if capacity < n {
            return Err(ShellcodeRunnerError::BufferTooSmall {
                needed: n,
                got: capacity,
            });
        }

        let mut vm = VirtualMemory::alloc(capacity)?;
        let vm_size = vm.size();

        let dst = vm
            .as_mut_slice()
            .get_mut(..n)
            .ok_or(ShellcodeRunnerError::BufferTooSmall {
                needed: n,
                got: vm_size,
            })?;
        dst.copy_from_slice(&self.bytes);

        if vm_size > n {
            vm.as_mut_slice()[n..].fill(0);
        }

        Ok(Shellcode {
            bytes: self.bytes,
            start_offset: self.start_offset,
            state: Loaded { vm },
        })
    }
}

impl Shellcode<Loaded> {
    pub fn start_ptr(&self) -> *mut c_void {
        self.state.vm.as_ptr().wrapping_add(self.start_offset)
    }
    pub fn vm(&self) -> &VirtualMemory {
        &self.state.vm
    }
}
