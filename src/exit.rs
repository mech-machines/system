extern crate crossbeam_channel;
use mech_core::{hash_string, TableIndex, Table, Value, ValueType, ValueMethods, Transaction, Change, TableId, Register};
use mech_utilities::{Machine, MachineRegistrar, RunLoopMessage};
//use std::sync::mpsc::{self, Sender};
use std::thread::{self};
use crossbeam_channel::Sender;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

lazy_static! {
  static ref SYSTEM_EXIT: u64 = hash_string("system/exit");
}

export_machine!(system_exit, system_exit_reg);

extern "C" fn system_exit_reg(registrar: &mut dyn MachineRegistrar, outgoing: Sender<RunLoopMessage>) -> String {
  registrar.register_machine(Box::new(Exit{outgoing}));
  "#system/exit = [|exit-code|]".to_string()
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
    Register{table_id: TableId::Global(*SYSTEM_EXIT), row: TableIndex::All, column: TableIndex::All}.hash()
  }

  fn on_change(&mut self, table: &Table) -> Result<(), String> {
    let (value, _) = table.get_unchecked(1,1);
    match value.value_type() {
      ValueType::Quantity => {
        let exit_code = value.as_i64().unwrap() as i32;
        self.outgoing.send(RunLoopMessage::Exit(exit_code));
      }
      ValueType::Boolean => {
        let exit_code = value.as_bool().unwrap() as i32;
        self.outgoing.send(RunLoopMessage::Exit(exit_code));
      }
      ValueType::NumberLiteral => {
        // TODO print number literals
        self.outgoing.send(RunLoopMessage::Exit(0));
      }
      _ => {self.outgoing.send(RunLoopMessage::Exit(0));}
    }
    Ok(())
  }
}