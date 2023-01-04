extern crate crossbeam_channel;
use mech_core::*;
use mech_utilities::*;
use crossbeam_channel::Sender;

lazy_static! {
  static ref SYSTEM_EXIT: u64 = hash_str("system/exit");
}

export_machine!(system_exit, system_exit_reg);

extern "C" fn system_exit_reg(registrar: &mut dyn MachineRegistrar, outgoing: Sender<RunLoopMessage>) -> String {
  registrar.register_machine(Box::new(Exit{outgoing}));
  "#system/exit = [|exit-code<_>|]".to_string()
}

#[derive(Debug)]
pub struct Exit {
  outgoing: Sender<RunLoopMessage>,
}

impl Machine for Exit {

  fn name(&self) -> String {
    "system/exit".to_string()
  }

  fn id(&self) -> u64 {
    hash_str(&self.name())
  }

  fn on_change(&mut self, table: &Table) -> Result<(), MechError> {
    match table.get(&TableIndex::Index(1),&TableIndex::Index(1))? {
      Value::F32(value) => {
        let exit_code: i32 = <F32>::into(value);
        self.outgoing.send(RunLoopMessage::Exit(exit_code));
      }
      Value::Bool(value) => {
        let exit_code = if value == true {0} else {1};
        self.outgoing.send(RunLoopMessage::Exit(exit_code));
      }
      _ => {self.outgoing.send(RunLoopMessage::Exit(0));}
    }
    Ok(())
  }
}