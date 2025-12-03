extern crate console_error_panic_hook;

mod counter;
mod pages;
mod router;
mod store;

use std::cell::{Ref, RefCell};
use std::panic;
use std::rc::Rc;

use web_sys::Node;

use respo::css::RespoStyle;
use respo::ui::ui_global;
use respo::{div, util::query_select_node, RespoApp, RespoNode, RespoStore};

use crate::pages::{comp_nav_links, comp_route_content};
use crate::router::{get_current_route, setup_router_listener};
pub use crate::store::ActionOp;
use crate::store::Store;

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

  fn dispatch(store: Rc<RefCell<Self::Model>>, op: <Self::Model as RespoStore>::Action) -> Result<(), String> {
    store.borrow_mut().update(op)
  }

  fn view(store: Ref<Self::Model>) -> Result<RespoNode<<Self::Model as RespoStore>::Action>, String> {
    Ok(
      div()
        .class(ui_global())
        .style(RespoStyle::default().padding(12.0))
        .children([
          comp_nav_links()?.to_node(),
          comp_route_content(&store.states, store.counted, &store.router)?.to_node(),
        ])
        .to_node(),
    )
  }
}

fn main() {
  panic::set_hook(Box::new(console_error_panic_hook::hook));

  // Get initial route from URL - this takes priority over saved state
  let initial_route = get_current_route();

  let app = App {
    mount_target: query_select_node(".app").expect("mount target"),
    store: Rc::new(RefCell::new(Store::default())),
  };

  // Load storage (may contain outdated router state)
  let _ = app.try_load_storage();

  // Override stored router with URL-based route
  // URL should always be the source of truth for routing
  app.store.borrow_mut().router = initial_route;

  app.backup_model_beforeunload().expect("backup model");

  // Setup popstate listener for browser back/forward navigation
  {
    let store = app.store.clone();
    setup_router_listener(move |route| {
      let _ = store.borrow_mut().update(ActionOp::RouteRestore(route));
      respo::request_rerender();
    });
  }

  app.render_loop().expect("app render");
}
