use respo::{states_tree::RespoUpdateState, RespoAction, RespoStore};
use serde::{Deserialize, Serialize};

use respo::states_tree::RespoStatesTree;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Store {
  pub counted: i32,
  pub states: RespoStatesTree,
}

#[derive(Clone, Debug, Default)]
pub enum ActionOp {
  Increment,
  Decrement,
  /// contains State and Value
  StatesChange(RespoUpdateState),
  #[default]
  Noop,
}

impl RespoAction for ActionOp {
  fn states_action(a: respo::states_tree::RespoUpdateState) -> Self {
    Self::StatesChange(a)
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
      ActionOp::StatesChange(a) => {
        self.states.set_in_mut(a);
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