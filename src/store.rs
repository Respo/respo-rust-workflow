use std::rc::Rc;

use respo::{RespoAction, RespoStore};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use respo::states_tree::{RespoStateBranch, RespoStatesTree};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Store {
  pub counted: i32,
  pub states: RespoStatesTree,
}

#[derive(Clone, Debug)]
pub enum ActionOp {
  Increment,
  Decrement,
  /// contains State and Value
  StatesChange(Vec<Rc<str>>, Option<RespoStateBranch>, Option<Value>),
  Noop,
}

/// TODO added to pass type checking, maybe we can remove it
impl Default for ActionOp {
  fn default() -> Self {
    ActionOp::Noop
  }
}

impl RespoAction for ActionOp {
  fn wrap_states_action(cursor: &[Rc<str>], a: Option<RespoStateBranch>) -> Self {
    // val is a backup value from DynEq to Json Value
    let val = match &a {
      None => None,
      Some(v) => v.0.as_ref().backup(),
    };
    Self::StatesChange(cursor.to_vec(), a, val)
  }
}

impl RespoStore for Store {
  type Action = ActionOp;

  fn update(&mut self, op: Self::Action) -> Result<(), String> {
    match op {
      ActionOp::Noop => {
        // nothing to to
      }
      ActionOp::Increment => {
        self.counted += 1;
      }
      ActionOp::Decrement => {
        self.counted -= 1;
      }
      ActionOp::StatesChange(path, new_state, val) => {
        self.states.set_in_mut(&path, new_state, val);
      }
    }
    Ok(())
  }

  fn to_string(&self) -> String {
    serde_json::to_string(&self).expect("to json")
  }

  fn try_from_string(s: &str) -> Result<Self, String>
  where
    Self: Sized,
  {
    match serde_json::from_str(s) {
      Ok(s) => Ok(s),
      Err(e) => Err(format!("{:?}", e)),
    }
  }
}
