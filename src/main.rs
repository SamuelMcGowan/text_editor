use std::time::Duration;

use text_editor::editor::vsplit::VSplit;
use text_editor::editor::Editor;
use text_editor::ui::*;

const REFRESH_RATE: Duration = Duration::from_millis(17);

fn main() {
    setup_logger().expect("failed to initialize logger");

    let widget = VSplit::new(Editor::default(), Editor::default(), None, None);
    let app = App::new(widget, REFRESH_RATE).expect("couldn't create app");
    app.run().expect("IO error");
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file("output.log")?)
        .apply()?;

    Ok(())
}
