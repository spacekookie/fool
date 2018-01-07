extern crate cursive;


use cursive::Cursive;
use cursive::event::{Event, Key};
use cursive::traits::*;
use cursive::views::{Dialog, Panel, OnEventView, TextArea, TextView, BoxView, Menubar};
use cursive::menu::MenuTree;
use cursive::align::*;

use cursive::theme::*;

fn main() {
    let mut siv = Cursive::new();
    siv.load_theme_file("assets/style.toml").unwrap();

    // The main dialog will just have a textarea.
    // Its size expand automatically with the content.
    // siv.add_layer(
    //     Dialog::new()
    //         .title("Describe your issue")
    //         .padding((1, 1, 1, 0))
    //         .content(TextArea::new().with_id("text"))
    //         .button("Ok", Cursive::quit),
    // );

    let size = siv.screen_size();
    let view = BoxView::with_fixed_size((size.x - 8, size.y - 4), Panel::new(TextArea::new().content(" Local:    master ~/Projects/code/fool
 Head:     8ef7c41 Miep


 Changes:
    Modified   Cargo.lock
    Modified   Cargo.toml
    Modified   src/main.rs

 # Cheat Sheet
 #    s = stage file/section, S = stage all unstaged files
 #    c = commit, C = commit -a (add unstaged)
 #    P = push to upstream
    ")));
    siv.add_layer(view);

    // The menubar is a list of (label, menu tree) pairs.
    siv.menubar()
        .add_subtree("Help", MenuTree::new())
        .add_subtree("Quit", MenuTree::new());

    siv.set_autohide_menu(false);
    siv.add_global_callback(Key::Esc, |s| s.select_menubar());
    siv.run();
}

// Dialog::new()
//     .title("Changes in the repo")
//     .padding((1, 1, 1, 1))
//     .h_align(HAlign::Center)
//     .content(TextArea::new().content("--------------------------------------------------------"))
//     .button("Ok", Cursive::quit),

// fn main() {
//     // Read some long text from a file.
//     let content = "Leverage agile frameworks to provide a robust
//     synopsis for high level overviews. Iterative approaches to corporate strategy foster collaborative thinking to further
// the overall value proposition. Organically grow the holistic world view of disruptive innovation via workplace diversity and empowerment.";
//     let mut siv = Cursive::new();

//     // We can quit by pressing q
//     siv.add_global_callback('q', |s| s.quit());

//     // The text is too long to fit on a line, so the view will wrap lines,
//     // and will adapt to the terminal size.
//     siv.add_layer(
//         Dialog::around(Panel::new(TextView::new(content)))
//             .h_align(HAlign::Center)
//             .button("Quit", |s| s.quit())
//             .full_screen(),
//     );
//     // Show a popup on top of the view.
//     siv.add_layer(Dialog::info(
//         "Try resizing the terminal!\n(Press 'q' to \
//          quit when you're done.)",
//     ));

//     siv.run();
// }
