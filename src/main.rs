use std::time::Duration;


use text_editor::widget::{App, InputPrinter};

fn main() {
    let app = App::new(InputPrinter::default(), Duration::from_millis(17)).expect("couldn't create app");
    app.run().expect("IO error");

    // let mut term = Term::new(libc::STDIN_FILENO).expect("couldn't enter raw mode");

    // loop {
    //     let size = term.size().expect("couldn't get terminal size");
    //     if size.0 == 0 || size.1 == 0 {
    //         continue;
    //     }

    //     let mut buffer = Buffer::filled(size.0, size.1, Cell::default());

    //     let c = 'X';

    //     buffer[[0, 0]].c = c;
    //     buffer[[0, size.1 - 1]].c = c;
    //     buffer[[size.0 - 1, 0]].c = c;
    //     buffer[[size.0 - 1, size.1 - 1]].c = c;

    //     term.render_buffer(&buffer).expect("couldn't render buffer");

    //     std::thread::sleep(Duration::from_millis(17));
    // }
}
