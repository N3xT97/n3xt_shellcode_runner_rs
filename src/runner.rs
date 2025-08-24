use crate::{
    error::ShellcodeRunnerError,
    logger::{Level, Logger, format_size, preview_hex_bytes},
    shellcode::{Loaded, Shellcode, Unloaded},
    thread::{Thread, ThreadState},
};
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
                    capacity: size,
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
        let log = Logger::new();

        log.hr("Load");
        let sc_loaded = self.load_shellcode()?;
        log.println(Level::Ok, "Shellcode loaded into VirtualMemory");

        let addr = sc_loaded.start_ptr() as usize;
        let aligned = sc_loaded.vm().size();
        let total = sc_loaded.bytes().len();
        let preview = preview_hex_bytes(sc_loaded.bytes(), 16);

        println!(
            "  {:<24} {}",
            "Entry Address",
            format!("{:#X} ({})", addr, addr)
        );
        println!("  {:<24} {}", "Aligned Size", format_size(aligned));
        println!("  {:<24} {}", "Payload Size", format_size(total));
        println!("  {:<24} {}", "Content Preview", preview);

        log.hr("Spawn");
        let th = Self::create_shellcode_thread_suspended(&sc_loaded)?;
        log.println(Level::Ok, "Thread created (suspended)");
        println!("  {:<24} {:#X} ({})", "Thread ID", th.tid(), th.tid());

        log.hr("Debug");
        log.println(Level::Warn, "Shellcode thread is suspended.");
        log.println(Level::Warn, "Attach a debugger NOW if you want to analyze.");
        log.println(Level::Step, "Press ENTER to resume execution…");
        Self::wait_enter()?;

        log.hr("Run");
        log.println(Level::Info, "Resuming thread");
        th.resume()?;
        log.println(Level::Info, "Waiting for shellcode thread to exit…");
        th.join()?;
        log.println(Level::Ok, "Thread exited");

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
