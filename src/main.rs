use std::time::Duration;

use text_editor::widget::root::Root;
use text_editor::widget::{App, InputPrinter};

fn main() {
    let refresh_rate = Duration::from_millis(17);

    let widget = Root::new(InputPrinter::default());
    let app = App::new(widget, refresh_rate).expect("couldn't create app");
    app.run().expect("IO error");
}
