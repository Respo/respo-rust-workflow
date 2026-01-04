//! Router definitions and utilities
//!
//! This module contains:
//! - Route definitions using ruled_router derive macros
//! - Navigation helper functions
//! - Browser history management
//! - Popstate event handling

use respo::DispatchFn;
use ruled_router::error::RouteState;
use ruled_router::prelude::*;
use ruled_router_derive::{QueryDerive, RouterData, RouterMatch};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::store::ActionOp;

// ============================================================================
// Route Definitions
// ============================================================================

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

impl Serialize for CounterModuleRoute {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    // Serialize as URL string for simpler storage format
    serializer.serialize_str(&self.format())
  }
}

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
// Navigation Helpers
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

/// Navigate to a new route by dispatching an action
/// This follows the unidirectional data flow pattern:
/// 1. Format route to URL path
/// 2. Push to browser history
/// 3. Dispatch action to update store
pub fn navigate_to(dispatch: &DispatchFn<ActionOp>, route: AppRouterMatch) -> Result<(), String> {
  let path = route.format();
  push_history_state(&path);
  dispatch.run(ActionOp::RouteChange(route))
}

// ============================================================================
// Browser History Management
// ============================================================================

/// Push a new state to browser history
fn push_history_state(path: &str) {
  if let Some(window) = web_sys::window() {
    if let Ok(history) = window.history() {
      let _ = history.push_state_with_url(&JsValue::NULL, "", Some(path));
    }
  }
}

/// Get current route from browser URL
pub fn get_current_route() -> AppRouterMatch {
  web_sys::window()
    .and_then(|w| {
      let pathname = w.location().pathname().ok()?;
      let search = w.location().search().unwrap_or_default();
      let full_path = format!("{pathname}{search}");
      Some(parse_route(&full_path))
    })
    .unwrap_or_default()
}

/// Parse URL path and return matched route
fn parse_route(path: &str) -> AppRouterMatch {
  AppRouterMatch::try_parse(path).unwrap_or_default()
}

// ============================================================================
// Popstate Event Handling
// ============================================================================

/// Setup popstate event listener to handle browser back/forward navigation
pub fn setup_router_listener<F>(on_route_change: F)
where
  F: Fn(AppRouterMatch) + 'static,
{
  let callback = Closure::wrap(Box::new(move |_event: web_sys::PopStateEvent| {
    let route = get_current_route();
    on_route_change(route);
  }) as Box<dyn Fn(_)>);

  if let Some(window) = web_sys::window() {
    let _ = window.add_event_listener_with_callback("popstate", callback.as_ref().unchecked_ref());
  }

  // Leak the closure to keep it alive for the lifetime of the app
  callback.forget();
}
