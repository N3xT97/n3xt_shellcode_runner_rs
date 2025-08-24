use crate::error::ShellcodeRunnerError;
use std::os::raw::c_void;
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE, WAIT_FAILED},
    System::Threading::{
        CreateThread, INFINITE, LPTHREAD_START_ROUTINE, ResumeThread, THREAD_CREATE_SUSPENDED,
        WaitForSingleObject,
    },
};
use windows::core::Error as WindowsError;

pub enum ThreadState {
    Suspend,
}

pub struct Thread {
    handle: HANDLE,
    tid: u32,
}

impl Thread {
    pub fn spawn(
        start: *const c_void,
        param: Option<*const c_void>,
        state: ThreadState,
    ) -> Result<Self, ShellcodeRunnerError> {
        let creation_flag = match state {
            ThreadState::Suspend => THREAD_CREATE_SUSPENDED,
        };

        let mut tid = 0;
        let thread_proc =
            unsafe { std::mem::transmute::<*const c_void, LPTHREAD_START_ROUTINE>(start) };
        let handle =
            unsafe { CreateThread(None, 0, thread_proc, param, creation_flag, Some(&mut tid))? };
        if handle.is_invalid() {
            return Err(ShellcodeRunnerError::WindowsError(
                WindowsError::from_win32(),
            ));
        }
        Ok(Self { handle, tid })
    }

    pub fn resume(&self) -> Result<(), ShellcodeRunnerError> {
        let res = unsafe { ResumeThread(self.handle) };
        if res == u32::MAX {
            return Err(ShellcodeRunnerError::WindowsError(
                WindowsError::from_win32(),
            ));
        }
        Ok(())
    }

    pub fn join(self) -> Result<(), ShellcodeRunnerError> {
        let res = unsafe { WaitForSingleObject(self.handle, INFINITE) };
        if res == WAIT_FAILED {
            return Err(ShellcodeRunnerError::WindowsError(
                WindowsError::from_win32(),
            ));
        }
        Ok(())
    }

    pub fn tid(&self) -> u32 {
        self.tid
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        let _ = unsafe { CloseHandle(self.handle) };
    }
}
