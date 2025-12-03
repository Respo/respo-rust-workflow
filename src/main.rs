extern crate console_error_panic_hook;

mod counter;
mod router;
mod router_util;
mod store;

use std::cell::{Ref, RefCell};
use std::panic;
use std::rc::Rc;

use ruled_router::error::RouteState;
use web_sys::Node;

use respo::ui::ui_global;
use respo::{css::RespoStyle, util, RespoApp, RespoNode, RespoStore};
use respo::{div, span, util::query_select_node};

use self::counter::comp_counter;
use self::router::{AppRouterMatch, CounterModuleRoute, CounterSubRouterMatch};
use self::router_util::{get_current_route, setup_router_listener};
pub use self::store::ActionOp;
use self::store::*;

const APP_STORE_KEY: &str = "demo_respo_store";

struct App {
  store: Rc<RefCell<Store>>,
  mount_target: Node,
}

impl RespoApp for App {
  type Model = Store;

  fn get_store(&self) -> &Rc<RefCell<Self::Model>> {
    &self.store
  }
  fn get_mount_target(&self) -> &web_sys::Node {
    &self.mount_target
  }
  fn pick_storage_key() -> &'static str {
    APP_STORE_KEY
  }

  fn dispatch(store_to_action: Rc<RefCell<Self::Model>>, op: <Self::Model as RespoStore>::Action) -> Result<(), String> {
    let mut store = store_to_action.borrow_mut();
    store.update(op)
  }

  fn view(store: Ref<Self::Model>) -> Result<RespoNode<<Self::Model as RespoStore>::Action>, String> {
    let states = &store.states;
    let router = &store.router;

    util::log!("[view] rendering with router: {:?}", router);

    Ok(
      div()
        .class(ui_global())
        .style(RespoStyle::default().padding(12.0))
        .children([
          // Navigation links
          comp_nav_links()?.to_node(),
          // Route-based content
          comp_route_content(states, store.counted, router)?.to_node(),
        ])
        .to_node(),
    )
  }
}

use respo::css::CssColor;
use respo::states_tree::RespoStatesTree;
use respo::ui::ui_button;
use respo::{button, DispatchFn, RespoElement, RespoEvent};

/// Navigation links component
fn comp_nav_links() -> Result<RespoElement<ActionOp>, String> {
  let on_home = move |e, dispatch: DispatchFn<ActionOp>| -> Result<(), String> {
    if let RespoEvent::Click { original_event, .. } = e {
      original_event.prevent_default();
    }
    router::navigate_home(&dispatch)
  };

  let on_counter = move |e, dispatch: DispatchFn<ActionOp>| -> Result<(), String> {
    if let RespoEvent::Click { original_event, .. } = e {
      original_event.prevent_default();
    }
    router::navigate_counter(&dispatch)
  };

  let on_counter_detail = move |e, dispatch: DispatchFn<ActionOp>| -> Result<(), String> {
    if let RespoEvent::Click { original_event, .. } = e {
      original_event.prevent_default();
    }
    router::navigate_counter_detail(&dispatch, 42)
  };

  Ok(
    div().style(RespoStyle::default().margin(8.0)).elements([
      button()
        .class(ui_button())
        .inner_text("Home")
        .style(RespoStyle::default().margin(4.0))
        .on_click(on_home),
      button()
        .class(ui_button())
        .inner_text("Counter")
        .style(RespoStyle::default().margin(4.0))
        .on_click(on_counter),
      button()
        .class(ui_button())
        .inner_text("Counter Detail (42)")
        .style(RespoStyle::default().margin(4.0))
        .on_click(on_counter_detail),
    ]),
  )
}

/// Route-based content component
fn comp_route_content(states: &RespoStatesTree, counted: i32, router: &AppRouterMatch) -> Result<RespoElement<ActionOp>, String> {
  match router {
    AppRouterMatch::Home(home_route) => {
      let welcome_msg = home_route.query.welcome.clone().unwrap_or_else(|| "Welcome Home!".to_string());
      Ok(
        div().style(RespoStyle::default().padding(16.0)).elements([span()
          .inner_text(format!("🏠 {welcome_msg}"))
          .style(RespoStyle::default().font_size(24.0).color(CssColor::Hsluv(200, 80, 50)))]),
      )
    }
    AppRouterMatch::Counter(counter_module) => comp_counter_route(states, counted, counter_module),
  }
}

/// Counter route content
fn comp_counter_route(
  states: &RespoStatesTree,
  counted: i32,
  counter_module: &CounterModuleRoute,
) -> Result<RespoElement<ActionOp>, String> {
  match &counter_module.sub_router {
    RouteState::SubRoute(CounterSubRouterMatch::Detail(detail)) => {
      // Counter detail view
      Ok(
        div().style(RespoStyle::default().padding(16.0)).elements([
          div().elements([span()
            .inner_text(format!("📊 Counter Detail - ID: {}", detail.id))
            .style(RespoStyle::default().font_size(20.0).color(CssColor::Hsluv(120, 80, 50)))]),
          div()
            .style(RespoStyle::default().margin(8.0))
            .elements([span().inner_text(format!("Tab: {:?}", detail.query.tab))]),
          div()
            .style(RespoStyle::default().margin(8.0))
            .elements([span().inner_text(format!("Page: {}", detail.query.page))]),
          // Include the counter component
          comp_counter(&states.pick("counter"), counted)?,
        ]),
      )
    }
    RouteState::NoSubRoute => {
      // Counter module main view
      Ok(
        div().style(RespoStyle::default().padding(16.0)).elements([
          div().elements([span()
            .inner_text("📊 Counter Module")
            .style(RespoStyle::default().font_size(20.0).color(CssColor::Hsluv(60, 80, 50)))]),
          div()
            .style(RespoStyle::default().margin(8.0))
            .elements([span().inner_text(format!("Tab: {:?}", counter_module.query.tab))]),
          div()
            .style(RespoStyle::default().margin(8.0))
            .elements([span().inner_text(format!("Page: {}", counter_module.query.page))]),
          // Include the counter component
          comp_counter(&states.pick("counter"), counted)?,
        ]),
      )
    }
    RouteState::ParseFailed {
      remaining_path,
      attempted_patterns,
      closest_match,
    } => {
      // Parse failed view
      Ok(
        div().style(RespoStyle::default().padding(16.0)).elements([
          span()
            .inner_text("⚠️ Route Parse Failed")
            .style(RespoStyle::default().font_size(20.0).color(CssColor::Hsluv(0, 80, 50))),
          div()
            .style(RespoStyle::default().margin(8.0))
            .elements([span().inner_text(format!("Remaining path: {remaining_path}"))]),
          div()
            .style(RespoStyle::default().margin(8.0))
            .elements([span().inner_text(format!("Attempted patterns: {attempted_patterns:?}"))]),
          div()
            .style(RespoStyle::default().margin(8.0))
            .elements([span().inner_text(format!("Closest match: {closest_match:?}"))]),
        ]),
      )
    }
  }
}

fn main() {
  panic::set_hook(Box::new(console_error_panic_hook::hook));

  // Get initial route from URL
  let initial_route = get_current_route();

  let app = App {
    mount_target: query_select_node(".app").expect("mount target"),
    store: Rc::new(RefCell::new(Store {
      router: initial_route,
      ..Store::default()
    })),
  };

  app.try_load_storage().expect("load storage");
  app.backup_model_beforeunload().expect("backup model");

  // Setup popstate listener for browser back/forward navigation
  // Uses RouteRestore to skip pushState since URL is already updated
  {
    let store = app.store.clone();
    setup_router_listener(move |route| {
      util::log!("[main] popstate callback: updating store with {:?}", route);
      let mut store = store.borrow_mut();
      let result = store.update(ActionOp::RouteRestore(route));
      util::log!("[main] popstate callback: store.update result = {:?}", result);
      // Request rerender after store update
      respo::request_rerender();
    });
  }

  util::log!("store: {:?}", app.store.as_ref());

  app.render_loop().expect("app render");
}
