//! Application state management

use respo::states_tree::{RespoStatesTree, RespoUpdateState};
use respo::{RespoAction, RespoStore};
use serde::{Deserialize, Serialize};

use crate::router::AppRouterMatch;

/// Application store - single source of truth
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Store {
  pub counted: i32,
  pub states: RespoStatesTree,
  /// Current matched route
  pub router: AppRouterMatch,
}

/// Application actions
#[derive(Clone, Debug, Default)]
pub enum ActionOp {
  Increment,
  Decrement,
  StatesChange(RespoUpdateState),
  /// Route change - navigates and pushes history state
  RouteChange(AppRouterMatch),
  /// Route restore - from browser back/forward, skips pushState
  RouteRestore(AppRouterMatch),
  #[default]
  Noop,
}

impl RespoAction for ActionOp {
  type Intent = ();

  fn states_action(a: RespoUpdateState) -> Self {
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
      ActionOp::Noop => {}
      ActionOp::Increment => self.counted += 1,
      ActionOp::Decrement => self.counted -= 1,
      ActionOp::StatesChange(a) => self.update_states(a),
      ActionOp::RouteChange(route) | ActionOp::RouteRestore(route) => {
        self.router = route;
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
    serde_json::from_str(s).map_err(|e| format!("{e:?}"))
  }
}
