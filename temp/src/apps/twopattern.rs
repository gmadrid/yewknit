use crate::bootstrap;

use crate::grids::GridTrait;
use crate::grids::{Color, FlippedGrid};
use crate::simplegrid::SimpleGrid;

use crate::tablerender::SimpleRenderer;
use crate::tablerender::{InputRenderer, PatternRenderer};

use bootstrap::empty;
use serde::{Deserialize, Serialize};
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::Area;
use yew::services::StorageService;

const GRID_HEIGHT: usize = 15;
const GRID_WIDTH: usize = 15;

const STORAGE_KEY: &str = "SAVED_CHART";
const VERSION_NUMBER: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize)]
struct Stored {
    front: SimpleGrid,
    back: SimpleGrid,
}

impl Default for Stored {
    fn default() -> Self {
        Stored {
            front: SimpleGrid::new(GRID_HEIGHT, GRID_WIDTH),
            back: SimpleGrid::new(GRID_HEIGHT, GRID_WIDTH),
        }
    }
}

pub struct TwoPattern {
    link: ComponentLink<Self>,
    stored: Stored,
    value: Option<Color>,
    printable_page: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GridId {
    Front,
    Back,
}

pub enum Msg {
    // Mouse events
    Down(GridId, usize, usize), // (id, row, col)
    Up,
    Enter(GridId, usize, usize), // (id, row, col)
    Exit,

    // User actions
    TogglePrintable,
    Clear(GridId),
}

impl TwoPattern {
    fn grid_by_id(&self, id: GridId) -> &SimpleGrid {
        match id {
            GridId::Front => &self.stored.front,
            GridId::Back => &self.stored.back,
        }
    }

    fn grid_by_id_mut(&mut self, id: GridId) -> &mut SimpleGrid {
        match id {
            GridId::Front => &mut self.stored.front,
            GridId::Back => &mut self.stored.back,
        }
    }

    fn grid_table(&self, grid_id: GridId) -> Html {
        let grid = self.grid_by_id(grid_id);
        InputRenderer::<SimpleGrid>::render_table(&self.link, grid_id, grid)
    }

    fn pattern_table(&self) -> Html {
        PatternRenderer::<SimpleGrid, SimpleGrid>::render_table(
            &self.link,
            GridId::Front,
            &self.stored.front,
            GridId::Back,
            &self.stored.back,
        )
    }

    fn reference_card(&self) -> Html {
        if !self.printable_page {
            empty()
        } else {
            let flipped = FlippedGrid::new(&self.stored.back);
            bootstrap::concat(
                bootstrap::spacer(),
                bootstrap::card(
                    "Reference",
                    "",
                    bootstrap::row(bootstrap::concat(
                        bootstrap::col(bootstrap::concat(
                            bootstrap::h5("Layer 1"),
                            SimpleRenderer::<SimpleGrid>::render_table(&self.stored.front),
                        )),
                        bootstrap::col(bootstrap::concat(
                            bootstrap::h5("Layer 2"),
                            SimpleRenderer::<FlippedGrid<SimpleGrid>>::render_table(&flipped),
                        )),
                    )),
                ),
            )
        }
    }

    fn msg_down(&mut self, id: GridId, row: usize, col: usize) -> bool {
        let grid = self.grid_by_id_mut(id);
        let value = !grid.cell(row, col);
        grid.set_cell(row, col, value);
        self.value = Some(value);

        true
    }

    fn msg_enter(&mut self, id: GridId, row: usize, col: usize) -> bool {
        if let Some(value) = self.value {
            let grid = self.grid_by_id_mut(id);
            grid.set_cell(row, col, value);
            true
        } else {
            false
        }
    }

    fn msg_exit(&self) -> bool {
        false
    }

    fn msg_up(&mut self) -> bool {
        self.value = None;
        false
    }

    fn msg_clear(&mut self, grid_id: GridId) -> bool {
        let grid = self.grid_by_id_mut(grid_id);
        grid.clear();
        true
    }

    fn msg_toggle_printable(&mut self) -> bool {
        self.printable_page = !self.printable_page;
        true
    }

    fn display_nav(&self) -> Html {
        if self.printable_page {
            return bootstrap::empty();
        }
        html! {
          <>
            <nav class="navbar navbar-expand-md">
              <a style="color:black" class="navbar-brand">{"Two-pattern double-knitting chart generator"}</a>
              <small>{"This tool can be used to plan out a two-pattern chart during Alasdair Post-Quinn's workshop, \"Two-pattern Double-knitting\". The handout for that workshop will further explain what to do."}</small>
            </nav>
          </>
        }
    }

    fn display_inputs(&self) -> Html {
        if self.printable_page {
            return bootstrap::empty();
        }

        let click_front_callback = self.link.callback(|_| Msg::Clear(GridId::Front));
        let click_back_callback = self.link.callback(|_| Msg::Clear(GridId::Back));

        html! {
          <>
            { bootstrap::spacer() }

            { bootstrap::row(bootstrap::concat(
                bootstrap::col(
                    bootstrap::card("Layer 1", "Transcribe a chart from Page 5 here.",
                      bootstrap::concat(
                          self.grid_table(GridId::Front),
                          html!{<a href="#" class="btn btn-primary" onclick=click_front_callback>{"Clear"}</a>}))),
                bootstrap::col(
                    bootstrap::card("Layer 2", "Transcribe a chart from Page 6 here.",
                      bootstrap::concat(
                          self.grid_table(GridId::Back),
                          html!{<a href="#" class="btn btn-primary" onclick=click_back_callback>{"Clear"}</a>}))),
            ))}

            { bootstrap::spacer() }
          </>
        }
    }

    fn display_footer(&self) -> Html {
        if self.printable_page {
            return bootstrap::empty();
        }

        html! {
          <footer class="footer">
            <div class="container">
               <small>{"Version "}{VERSION_NUMBER}</small>
               <a href="https://double-knitting.com/" class="text-muted">{"Fallingblox Designs"}</a>
            </div>
          </footer>
        }
    }
}

impl Component for TwoPattern {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let Json(stored) = storage.restore(STORAGE_KEY);
        TwoPattern {
            link,
            stored: stored.unwrap_or_else(|_| Stored::default()),
            value: None::<Color>,
            printable_page: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let (save, should_render) = match msg {
            Msg::Down(id, row, col) => (false, self.msg_down(id, row, col)),
            Msg::Enter(id, row, col) => (false, self.msg_enter(id, row, col)),
            Msg::Exit => (false, self.msg_exit()),
            Msg::Up => (true, self.msg_up()),

            Msg::Clear(grid_id) => (true, self.msg_clear(grid_id)),
            Msg::TogglePrintable => (false, self.msg_toggle_printable()),
        };
        if save {
            let mut storage = StorageService::new(Area::Local).unwrap();
            storage.store(STORAGE_KEY, Json(&self.stored));
        }
        should_render
    }

    fn change(&mut self, _: <Self as yew::Component>::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let printable_callback = self.link.callback(|_| Msg::TogglePrintable);
        let printable_text = if self.printable_page {
            "Return to input page"
        } else {
            "Show printable pattern"
        };

        html! {
          <>
            { self.display_nav() }

            <main class="main container">
              { self.display_inputs() }

              { bootstrap::row(bootstrap::col(
                  bootstrap::concat(
                    bootstrap::card(
                      html! {
                        <>
                          <span>{"Chart 2"}</span>

                          <a class="noprint"
                              onclick=printable_callback
                              style="float:right" href="#"><small>{printable_text}</small></a>
                        </>
                      },
                      "Follow this chart within the green box in Chart 1 on Page 7.",
                      self.pattern_table()),
                    self.reference_card())
              ))}
            </main>
            { self.display_footer() }
          </>
        }
    }
}
