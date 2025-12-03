//! Page components for route-based rendering

use respo::css::{CssColor, RespoStyle};
use respo::states_tree::RespoStatesTree;
use respo::ui::ui_button;
use respo::{button, div, span, DispatchFn, RespoElement, RespoEvent};
use ruled_router::error::RouteState;

use crate::counter::comp_counter;
use crate::router::{self, AppRouterMatch, CounterModuleRoute, CounterSubRouterMatch};
use crate::store::ActionOp;

/// Navigation links component
pub fn comp_nav_links() -> Result<RespoElement<ActionOp>, String> {
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

/// Route-based content component - renders different pages based on current route
pub fn comp_route_content(states: &RespoStatesTree, counted: i32, router: &AppRouterMatch) -> Result<RespoElement<ActionOp>, String> {
  match router {
    AppRouterMatch::Home(home_route) => comp_home_page(home_route),
    AppRouterMatch::Counter(counter_module) => comp_counter_page(states, counted, counter_module),
  }
}

/// Home page component
fn comp_home_page(home_route: &router::HomeRoute) -> Result<RespoElement<ActionOp>, String> {
  let welcome_msg = home_route.query.welcome.clone().unwrap_or_else(|| "Welcome Home!".to_string());
  Ok(
    div().style(RespoStyle::default().padding(16.0)).elements([span()
      .inner_text(format!("🏠 {welcome_msg}"))
      .style(RespoStyle::default().font_size(24.0).color(CssColor::Hsluv(200, 80, 50)))]),
  )
}

/// Counter page component - handles counter module routes
fn comp_counter_page(
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
          comp_counter(&states.pick("counter"), counted)?,
        ]),
      )
    }
    RouteState::ParseFailed {
      remaining_path,
      attempted_patterns,
      closest_match,
    } => {
      // Route parse failed - show error page
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
