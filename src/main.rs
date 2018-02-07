extern crate cursive;
extern crate clap;

mod state;

mod ui;
use ui::*;


use clap::App;

const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const DEVELOPER: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {

    /* Print --help/ --version and ignore matches */
    let _ = App::new(APP_NAME)
        .version(VERSION)
        .author(DEVELOPER)
        .about(DESCRIPTION)
        .get_matches();

    // TODO: Handle config creation/ loading


    /* Initialise the main Ui (blocks) */
    let mut ui = Ui::new(FoolTheme::Dark);
    ui.run();
}
