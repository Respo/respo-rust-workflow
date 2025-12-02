use respo::{util, DispatchFn};
use ruled_router::RouteMatcher;

use crate::router::AppRouterMatch;
use crate::store::ActionOp;

/// Parse the current URL path and return the matched route
pub fn parse_route(path: &str) -> AppRouterMatch {
  util::log!("[router] parse_route input: {}", path);
  match AppRouterMatch::try_parse(path) {
    Ok(route) => {
      util::log!("[router] parse_route success: {:?}", route);
      route
    }
    Err(e) => {
      util::log!("[router] parse_route error: {:?}, using default", e);
      AppRouterMatch::default()
    }
  }
}

/// Format a route back to a URL path string
pub fn format_route(route: &AppRouterMatch) -> String {
  let path = route.format();
  util::log!("[router] format_route: {:?} -> {}", route, path);
  path
}

/// Navigate to a new route by dispatching an action
/// This follows the unidirectional data flow pattern
pub fn navigate_to(dispatch: &DispatchFn<ActionOp>, route: AppRouterMatch) -> Result<(), String> {
  util::log!("[router] navigate_to: {:?}", route);
  let path = format_route(&route);

  // Update browser history
  push_history_state(&path);

  // Dispatch route change action
  dispatch.run(ActionOp::RouteChange(route))
}

/// Push a new state to browser history
pub fn push_history_state(path: &str) {
  util::log!("[router] push_history_state: {}", path);
  if let Some(window) = web_sys::window() {
    if let Ok(history) = window.history() {
      let _ = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(path));
    }
  }
}

/// Replace current state in browser history
pub fn replace_history_state(path: &str) {
  if let Some(window) = web_sys::window() {
    if let Ok(history) = window.history() {
      let _ = history.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(path));
    }
  }
}

/// Get current route from browser URL
pub fn get_current_route() -> AppRouterMatch {
  if let Some(window) = web_sys::window() {
    if let Ok(pathname) = window.location().pathname() {
      let search = window.location().search().unwrap_or_default();
      let full_path = format!("{pathname}{search}");
      util::log!("[router] get_current_route: full_path = {}", full_path);
      return parse_route(&full_path);
    }
  }
  util::log!("[router] get_current_route: using default");
  AppRouterMatch::default()
}

/// Get current pathname from browser
pub fn get_current_pathname() -> String {
  web_sys::window()
    .and_then(|w| w.location().pathname().ok())
    .unwrap_or_else(|| "/".to_string())
}

/// Get current search query from browser
pub fn get_current_search() -> String {
  web_sys::window().and_then(|w| w.location().search().ok()).unwrap_or_default()
}

/// Setup popstate event listener to handle browser back/forward navigation
/// Returns the initial route from the current URL
pub fn setup_router_listener<F>(on_route_change: F) -> AppRouterMatch
where
  F: Fn(AppRouterMatch) + 'static,
{
  use wasm_bindgen::prelude::*;
  use wasm_bindgen::JsCast;

  util::log!("[router] setup_router_listener: registering popstate listener");

  let callback = Closure::wrap(Box::new(move |_event: web_sys::PopStateEvent| {
    util::log!("[router] popstate event triggered!");
    let route = get_current_route();
    util::log!("[router] popstate: calling on_route_change with {:?}", route);
    on_route_change(route);
    util::log!("[router] popstate: on_route_change completed");
  }) as Box<dyn Fn(_)>);

  if let Some(window) = web_sys::window() {
    let result = window.add_event_listener_with_callback("popstate", callback.as_ref().unchecked_ref());
    util::log!("[router] add_event_listener result: {:?}", result);
  }

  // Leak the closure to keep it alive for the lifetime of the app
  callback.forget();

  // Return the initial route
  get_current_route()
}
