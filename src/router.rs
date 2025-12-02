use ruled_router::error::RouteState;
use ruled_router::prelude::*;
use ruled_router_derive::{QueryDerive, RouterData, RouterMatch};
use serde::{Deserialize, Serialize};

use respo::DispatchFn;

use crate::router_util::navigate_to;
use crate::store::ActionOp;

/// Top-level route matcher (enum for matching different routes)
/// Note: Order matters! More specific routes should come first.
#[derive(RouterMatch, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AppRouterMatch {
  Counter(CounterModuleRoute),
  Home(HomeRoute),
}

impl Default for AppRouterMatch {
  fn default() -> Self {
    AppRouterMatch::Home(HomeRoute::default())
  }
}

/// Home route (root path)
#[derive(RouterData, Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[router(pattern = "/")]
pub struct HomeRoute {
  #[query]
  pub query: HomeQuery,
}

/// Home query parameters
#[derive(QueryDerive, Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct HomeQuery {
  #[query(name = "welcome")]
  pub welcome: Option<String>,
}

/// Counter module route with fixed prefix pattern
#[derive(RouterData, Debug, Clone, PartialEq)]
#[router(pattern = "/counter")]
pub struct CounterModuleRoute {
  #[query]
  pub query: CounterQuery,
  #[sub_router]
  pub sub_router: RouteState<CounterSubRouterMatch>,
}

// Serialize as URL string
impl Serialize for CounterModuleRoute {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(&self.format())
  }
}

// Deserialize from URL string
impl<'de> Deserialize<'de> for CounterModuleRoute {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    <Self as RouterData>::parse_route(&s).map_err(serde::de::Error::custom)
  }
}

impl Default for CounterModuleRoute {
  fn default() -> Self {
    Self {
      query: CounterQuery::default(),
      sub_router: RouteState::NoSubRoute,
    }
  }
}

/// Sub-router matcher for counter routes
#[derive(RouterMatch, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CounterSubRouterMatch {
  Detail(CounterDetailRoute),
}

/// Detail route with dynamic parameter
#[derive(RouterData, Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[router(pattern = "/:id")]
pub struct CounterDetailRoute {
  #[serde(default)]
  pub id: u32,
  #[query]
  pub query: CounterQuery,
}

/// Query parameters for counter routes
#[derive(QueryDerive, Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct CounterQuery {
  #[query(name = "tab")]
  pub tab: Option<String>,
  #[query(name = "page", default = "1")]
  pub page: u32,
}

// ============================================================================
// App-specific navigation helpers
// ============================================================================

/// Navigate to home page
pub fn navigate_home(dispatch: &DispatchFn<ActionOp>) -> Result<(), String> {
  navigate_to(dispatch, AppRouterMatch::Home(HomeRoute::default()))
}

/// Navigate to counter page
pub fn navigate_counter(dispatch: &DispatchFn<ActionOp>) -> Result<(), String> {
  navigate_to(dispatch, AppRouterMatch::Counter(CounterModuleRoute::default()))
}

/// Navigate to counter detail page with id
pub fn navigate_counter_detail(dispatch: &DispatchFn<ActionOp>, id: u32) -> Result<(), String> {
  navigate_to(
    dispatch,
    AppRouterMatch::Counter(CounterModuleRoute {
      query: CounterQuery::default(),
      sub_router: RouteState::SubRoute(CounterSubRouterMatch::Detail(CounterDetailRoute {
        id,
        query: CounterQuery::default(),
      })),
    }),
  )
}
