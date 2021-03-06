use yew::prelude::*;

mod input;
mod pattern;
mod simple;

pub use input::InputRenderer;
pub use pattern::PatternRenderer;
pub use simple::SimpleRenderer;

fn no_dot() -> Html {
    html! {
        <svg width="1em" height="1em" viewBox="0 0 15 15" class="bi bi-circle-fill" fill="black" xmlns="http://www.w3.org/2000/svg">
        </svg>
    }
}

pub fn render_table(renderer: impl TableRenderer, class: Option<&str>) -> Html {
    let mut classes = vec!["user-select-none"];
    if let Some(class) = class {
        classes.push(class);
    }
    html! {
        <table class=classes>
        {for (0..renderer.num_data_rows()).map(|rn| {
            {render_full_row(&renderer, rn)}
        })}
            {renderer.render_footer_row()}
        </table>
    }
}

fn render_full_row(renderer: &impl TableRenderer, row_num: usize) -> Html {
    render_row(renderer, row_num)
}

fn render_row(renderer: &impl TableRenderer, row_num: usize) -> Html {
    html! {<tr>
        {renderer.render_left_cell(row_num)}
        {renderer.render_data_row(row_num)}
        {renderer.render_right_cell(row_num)}
    </tr>
    }
}

pub trait TableRenderer {
    fn num_data_rows(&self) -> usize;
    fn num_data_cols(&self) -> usize;

    fn render_data_row(&self, row: usize) -> Html;
    fn render_data_cell(&self, row: usize, col: usize) -> Html; // ???

    fn render_left_cell(&self, row: usize) -> Html;
    fn render_right_cell(&self, row: usize) -> Html;

    fn render_footer_row(&self) -> Html;
    fn render_footer_cell(&self, col: usize) -> Html; // ???
}
