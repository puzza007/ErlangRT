//!
//! Implements Erlang process, an independent computing unit of Erlang with
//! heap, stack, registers, and message queue.
//!

use rt_defs::{Word, ExceptionType};
use emulator::code_srv::CodeServer;
use emulator::heap::{Heap, DEFAULT_PROC_HEAP};
use emulator::mfa::MFArity;
use emulator::runtime_ctx;
use emulator::scheduler;
use fail::Hopefully;
use term::lterm::*;

use std::fmt;


fn module() -> &'static str { "process: " }


#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ProcessError {
  None,
  Exception(ExceptionType, LTerm),
}


impl fmt::Display for ProcessError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ProcessError::None => write!(f, "NoError"),
      ProcessError::Exception(exc_type, exc_reason) => {
        match exc_type {
          ExceptionType::Exit => write!(f, "exit({})", exc_reason),
          ExceptionType::Throw => write!(f, "throw({})", exc_reason),
          ExceptionType::Error => write!(f, "error({})", exc_reason),
        }
      },
    }
  }
}

pub struct Process {
  pub pid: LTerm,
  //parent_pid: LTerm,

  //
  // Scheduling and fail state
  //

  /// Scheduling priority (selects the runqueue when this process is scheduled)
  pub prio: scheduler::Prio,
  /// Current scheduler queue where this process is registered
  pub current_queue: scheduler::Queue,

  /// Runtime context with registers, instruction pointer etc
  pub context: runtime_ctx::Context,
  /// How many X registers in the context are currently used
  pub live: Word,

  pub heap: Heap,

  //
  // Error handling
  //
  /// Record result of last scheduled timeslice for this process
  /// (updated by the vm loop)
  pub timeslice_result: scheduler::SliceResult,
  pub error: ProcessError,
}


impl Process {
  // Call this only from VM, the new process must be immediately registered
  // in proc registry for this VM
  pub fn new(pid: LTerm, _parent_pid: LTerm, mfarity: &MFArity,
             prio: scheduler::Prio, code_server: &mut CodeServer) -> Hopefully<Process> {
    assert!(pid.is_local_pid());
    assert!(_parent_pid.is_local_pid() || _parent_pid.is_nil());

    // Process must start with some code location
    match code_server.lookup_and_load(mfarity) {
      Ok(ip) => {
        let p = Process {
          pid,
          //parent_pid: nil(),
          prio,
          current_queue: scheduler::Queue::None,
          timeslice_result: scheduler::SliceResult::None,
          heap: Heap::new(DEFAULT_PROC_HEAP),

          context: runtime_ctx::Context::new(ip),
          live: 0,

          error: ProcessError::None,
        };
        Ok(p)
        //Ok(sync::Arc::new(sync::RwLock::new(p)))
      },
      Err(e) => Err(e)
    }
  }


  /// Returns true if there was an error or exception during the last timeslice.
  #[inline]
  pub fn is_failed(&self) -> bool {
    self.error != ProcessError::None
  }


  #[allow(dead_code)]
  pub fn jump(&mut self, mfarity: &MFArity, code_server: &mut CodeServer) -> Hopefully<()> {
    // TODO: Find mfa in code server and set IP to it
    match code_server.lookup_and_load(mfarity) {
      Ok(ip) => {
        self.context.ip = ip;
        Ok(())
      },
      Err(e) => Err(e)
    }
  }


  pub fn exception(&mut self, exc: ExceptionType, rsn: LTerm) -> LTerm {
    self.set_error(ProcessError::Exception(exc, rsn))
  }


  /// Sets error state from an opcode or a BIF. VM will hopefully check this
  /// immediately and finish the process or catch the error.
  fn set_error(&mut self, e: ProcessError) -> LTerm {
    println!("{}{} set_error {}", module(), self.pid, e);
    self.error = e;
    LTerm::non_value()
  }


//  pub fn clear_error(&mut self) {
//    self.error = ProcessError::None;
//  }
}
