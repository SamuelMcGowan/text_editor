use std::time::Duration;

use text_editor::editor::Editor;
use text_editor::ui::*;

fn main() {
    setup_logger().expect("failed to initialize logger");

    let refresh_rate = Duration::from_millis(17);

    // let widget = Root::new(VSplit::new(
    //     Editor::default(),
    //     Editor::default(),
    //     None,
    //     Some(1),
    // ));

    let widget = Editor::default();

    let app = App::new(widget, refresh_rate).expect("couldn't create app");
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
