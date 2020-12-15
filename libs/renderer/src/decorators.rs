mod class_decorator;
mod cssmunger;
mod label_decorator;
mod style_decorator;

pub use class_decorator::ClassDecorator;
pub use label_decorator::LabelDecorator;
pub use style_decorator::StyleDecorator;

pub use class_decorator::{
    BorderedCellDecorator, PrintableColorDecorator, RegularSizedTableDecorator, ThickBorders,
};
pub use label_decorator::{EmptyLabels, FlatLabels};
pub use style_decorator::ColorDecorator;

pub use cssmunger::CssMunger;
