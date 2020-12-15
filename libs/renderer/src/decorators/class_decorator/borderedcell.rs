use crate::decorators::{ClassDecorator, CssMunger};
use grids::{Color, GridTrait};

static BCD_ONCE: std::sync::Once = std::sync::Once::new();
static TB_ONCE: std::sync::Once = std::sync::Once::new();

pub struct BorderedCellDecorator;

impl Default for BorderedCellDecorator {
    fn default() -> Self {
        BorderedCellDecorator
    }
}

impl ClassDecorator for BorderedCellDecorator {
    fn register(&self, munger: &CssMunger) {
        BCD_ONCE.call_once(|| munger.insert_rule(".bdrcell { border: 1px solid black }"));
    }
    fn cell_class(
        &self,
        _grid: &dyn GridTrait,
        _row: usize,
        _col: usize,
        _contents: Color,
    ) -> Vec<&'static str> {
        vec!["bdrcell"]
    }
}

pub struct ThickBorders;

impl Default for ThickBorders {
    fn default() -> Self {
        ThickBorders
    }
}

impl ClassDecorator for ThickBorders {
    fn register(&self, munger: &CssMunger) {
        TB_ONCE.call_once(|| {
            // The rule is 'td.tckleft', including the element type in order to override any
            // 'border' rules.
            munger.insert_rule("td.tckleft { border-left: 3px solid black}");
            munger.insert_rule("td.tcktop { border-top: 3px solid black}");
            munger.insert_rule("td.tckright { border-right: 3px solid black}");
            munger.insert_rule("td.tckbottom { border-bottom: 3px solid black}");
        })
    }
    fn cell_class(
        &self,
        grid: &dyn GridTrait,
        row: usize,
        col: usize,
        _contents: Color,
    ) -> Vec<&'static str> {
        let mut ret = vec![];

        // These are 1-indexed from the bottom-right.
        let row_p = grid.num_rows() - row;
        let col_p = grid.num_cols() - col;

        if row_p % 5 == 0 {
            ret.push("tcktop")
        }
        if col_p % 5 == 0 {
            ret.push("tckleft")
        }

        ret
    }
}
