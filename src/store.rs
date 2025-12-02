use respo::{states_tree::RespoUpdateState, util, RespoAction, RespoStore};
use serde::{Deserialize, Serialize};

use respo::states_tree::RespoStatesTree;

use crate::router::AppRouterMatch;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Store {
  pub counted: i32,
  pub states: RespoStatesTree,
  /// Router state - stores the current matched route
  pub router: AppRouterMatch,
}

#[derive(Clone, Debug, Default)]
pub enum ActionOp {
  Increment,
  Decrement,
  /// contains State and Value
  StatesChange(RespoUpdateState),
  /// Route change action - navigates and pushes history state
  RouteChange(AppRouterMatch),
  /// Route restore action - from browser back/forward, skips pushState
  RouteRestore(AppRouterMatch),
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
      ActionOp::RouteChange(route) => {
        util::log!("[store] RouteChange: {:?}", route);
        self.router = route;
      }
      ActionOp::RouteRestore(route) => {
        // Same as RouteChange but triggered by popstate
        // The URL is already correct, no need to push history
        util::log!("[store] RouteRestore: {:?}", route);
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
    serde_json::from_str(s).map_err(|e| format!("{:?}", e))
  }
}
