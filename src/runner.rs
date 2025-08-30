use crate::logger::{info, key_value, ok, step, title, warn};
use crate::{
    error::ShellcodeRunnerError,
    shellcode::{Loaded, Shellcode, Unloaded},
    thread::{Thread, ThreadState},
};
use std::io::Write;
use std::{fs, io, path::Path, sync::Arc};

#[derive(Clone, Debug)]
pub struct Runner {
    bytes: Arc<[u8]>,
    start_offset: usize,
    memory_size: Option<usize>,
}

impl Runner {
    pub fn new(
        bytes: Arc<[u8]>,
        start_offset: usize,
        memory_size: Option<usize>,
    ) -> Result<Self, ShellcodeRunnerError> {
        let len = bytes.len();

        if start_offset >= len {
            return Err(ShellcodeRunnerError::InvalidOffset {
                offset: start_offset,
                len,
            });
        }

        if let Some(size) = memory_size {
            if size < len {
                return Err(ShellcodeRunnerError::InsufficientCapacity {
                    got: size,
                    required: len,
                });
            }
        }

        Ok(Self {
            bytes,
            start_offset,
            memory_size,
        })
    }

    pub fn from_file<P: AsRef<Path>>(
        file_path: P,
        start_offset: usize,
        memory_size: Option<usize>,
    ) -> Result<Self, ShellcodeRunnerError> {
        let p = file_path.as_ref();
        match fs::metadata(p) {
            Ok(md) if md.is_file() => {
                let bytes = fs::read(p)?;
                Self::from_bytes(bytes, start_offset, memory_size)
            }
            Ok(_) => Err(ShellcodeRunnerError::NotAFile {
                path: p.display().to_string(),
            }),
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                Err(ShellcodeRunnerError::InvalidFilePath {
                    path: p.display().to_string(),
                })
            }
            Err(e) => Err(ShellcodeRunnerError::IoError(e)),
        }
    }

    pub fn from_bytes<B: AsRef<[u8]>>(
        bytes: B,
        start_offset: usize,
        memory_size: Option<usize>,
    ) -> Result<Self, ShellcodeRunnerError> {
        let boxed: Box<[u8]> = bytes.as_ref().into();
        let arc: Arc<[u8]> = Arc::from(boxed);
        Self::new(arc, start_offset, memory_size)
    }

    pub fn run(&self) -> Result<(), ShellcodeRunnerError> {
        println!();
        title("Load");
        step("Loading shellcode…");
        let sc_loaded = self.load_shellcode()?;
        ok("Shellcode loaded.");

        let addr = sc_loaded.start_ptr() as usize;
        key_value("Entry Address", format!("{0:#X} ({0})", addr));
        key_value(
            "Aligned Size",
            format!("{0:#X} ({0})", sc_loaded.vm().size()),
        );
        key_value(
            "Payload Size",
            format!("{0:#X} ({0})", sc_loaded.bytes().len()),
        );
        key_value("Content Preview", preview_hex_bytes(sc_loaded.bytes(), 16));

        println!();
        title("Spawn");
        step("Creating suspended thread…");
        let th = Self::create_shellcode_thread_suspended(&sc_loaded)?;
        ok("Thread created (suspended).");
        key_value("Thread ID", format!("{:#X} ({})", th.tid(), th.tid()));

        println!();
        title("Debug");
        warn("Shellcode thread is suspended.");
        warn("Attach a debugger NOW if you want to analyze.");
        print!("[>] Press ENTER to resume execution… ");
        io::stdout().flush().ok();
        Self::wait_enter()?;
        ok("Resuming thread.");

        println!();
        title("Run");
        info("Waiting for shellcode thread to exit…");
        th.resume()?;
        th.join()?;
        ok("Thread exited.");

        Ok(())
    }

    fn load_shellcode(&self) -> Result<Shellcode<Loaded>, ShellcodeRunnerError> {
        let sc = Shellcode::<Unloaded>::new(self.bytes.clone(), self.start_offset)?;
        let loaded = match self.memory_size {
            Some(n) => sc.load_with_capacity(n)?,
            None => sc.load()?,
        };
        Ok(loaded)
    }

    fn create_shellcode_thread_suspended(
        sc_loaded: &Shellcode<Loaded>,
    ) -> Result<Thread, ShellcodeRunnerError> {
        let start_ptr = sc_loaded.start_ptr();
        Thread::spawn(start_ptr, None, ThreadState::Suspend)
    }

    fn wait_enter() -> Result<(), ShellcodeRunnerError> {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        Ok(())
    }
}

fn preview_hex_bytes(bytes: &[u8], max: usize) -> String {
    use std::fmt::Write;
    let mut s = String::new();
    for (i, b) in bytes.iter().take(max).enumerate() {
        if i > 0 {
            s.push(' ');
        }
        write!(s, "{:02X}", b).unwrap();
    }
    s
}
