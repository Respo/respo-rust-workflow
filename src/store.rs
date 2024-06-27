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
  type Intent = ();

  fn states_action(a: respo::states_tree::RespoUpdateState) -> Self {
    Self::StatesChange(a)
  }
}

impl RespoStore for Store {
  type Action = ActionOp;

  fn get_states(&mut self) -> &mut RespoStatesTree {
    &mut self.states
  }

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
      ActionOp::StatesChange(a) => self.update_states(a),
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
    serde_json::from_str(s).map_err(|e| format!("{:?}", e))
  }
}
