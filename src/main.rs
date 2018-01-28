extern crate cursive;
extern crate clap;

mod ui;
use ui::*;

mod state;
use state::Buffer;

use std::sync::{Arc, Mutex};
use clap::App;


const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const DEVELOPER: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// fn register_callbacks(siv: &mut Cursive, buffer: &Arc<Mutex<Buffer>>) {
//     {
//         // ARROW UP
//         let b = Arc::clone(buffer);
//         siv.add_global_callback(Key::Up, move |siv| {
//             let mut buffer = b.lock().unwrap();
//             buffer.move_up();

//             let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
//             (&mut *tv).set_content(buffer.render());
//         });
//     }

//     {
//         // ARROW DOWN
//         let b = Arc::clone(buffer);
//         siv.add_global_callback(Key::Down, move |siv| {
//             let mut buffer = b.lock().unwrap();
//             buffer.move_down();

//             let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
//             (&mut *tv).set_content(buffer.render());
//         });
//     }

//     {
//         // STAGE A SINGLE FILE
//         let b = Arc::clone(buffer);
//         siv.add_global_callback('s', move |siv| {
//             let mut buffer = b.lock().unwrap();

//             /* Small hack: Don't let people stage conflicts */
//             let changed = buffer.get_selection();
//             if changed.1 == ChangeType::Conflicted {
//                 let dialog = Dialog::new()
//                     .title("Can't stage conflicted file!")
//                     .content(TextView::new("Quit fool, open your favourite text editor\nand fix the issue first!"))
//                     .padding((1, 1, 1, 0))
//                     .button("Ok", |siv| siv.pop_layer());

//                 siv.add_layer(dialog);
//                 return;
//             }

//             Git::stage(&changed.0);
//             let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
//             update_from_git(&mut buffer, &mut tv);
//         });
//     }

//     {
//         // UNSTAGE A SINGLE FILE
//         let b = Arc::clone(buffer);
//         siv.add_global_callback('u', move |siv| {
//             let mut buffer = b.lock().unwrap();
//             Git::unstage(&buffer.get_selection().0);
//             let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
//             update_from_git(&mut buffer, &mut tv);
//         });
//     }

//     {
//         // STAGE ALL
//         let b = Arc::clone(buffer);
//         siv.add_global_callback('S', move |siv| {
//             let mut buffer = b.lock().unwrap();
//             Git::stage_all();
//             let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
//             update_from_git(&mut buffer, &mut tv);
//         });
//     }

//     {
//         // UNSTAGE ALL
//         let b = Arc::clone(buffer);
//         siv.add_global_callback('U', move |siv| {
//             let mut buffer = b.lock().unwrap();
//             Git::unstage_all();
//             let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
//             update_from_git(&mut buffer, &mut tv);
//         });
//     }

//     {
//         // Create a commit
//         let b = Arc::clone(buffer);
//         siv.add_global_callback('c', move |siv| {
//             let b = Arc::clone(&b);
//             let b2 = Arc::clone(&b);

//             let mut size = siv.screen_size();
//             if size.x > 80 {
//                 size.x = 80;
//             }

//             let dialog = Dialog::new()
//                 .title("Enter a commit message")
//                 .padding((1, 1, 1, 0))
//                 .content(
//                     EditView::new()
//                         .on_submit(move |siv, txt| {
//                             let mut buffer = b2.lock().unwrap();
//                             if txt == "" {
//                                 siv.pop_layer();
//                                 let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
//                                 update_from_git(&mut buffer, &mut tv);
//                                 return;
//                             }

//                             Git::commit(txt);
//                             siv.pop_layer();
//                             let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
//                             update_from_git(&mut buffer, &mut tv);
//                         })
//                         .with_id("commit")
//                         .fixed_width(20),
//                 )
//                 .button("Ok", move |siv| {
//                     let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
//                     let mut bfo = b.lock().unwrap();
//                     let msg = siv.call_on_id("commit", |view: &mut EditView| view.get_content())
//                         .unwrap();
//                     if *msg == "".to_owned() {
//                         siv.pop_layer();
//                         let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
//                         update_from_git(&mut bfo, &mut tv);
//                         return;
//                     }

//                     Git::commit(&*msg);
//                     update_from_git(&mut bfo, &mut tv);
//                     siv.pop_layer();
//                 });

//             siv.add_layer(BoxView::with_fixed_size(
//                 (size.x - 2, size.y / 2),
//                 Panel::new(dialog),
//             ));
//         });

//         {
//             let b = Arc::clone(buffer);
//             siv.add_global_callback('U', move |siv| {
//                 let mut buffer = b.lock().unwrap();
//                 Git::unstage_all();
//                 let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
//                 update_from_git(&mut buffer, &mut tv);
//             });
//         }
//     }

//     {
//         let b = Arc::clone(buffer);
//         siv.add_global_callback('P', move |siv| {
//             let mut buffer = b.lock().unwrap();
//             Git::push();
//             let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
//             update_from_git(&mut buffer, &mut tv);

//             siv.add_layer(
//                 Dialog::around(TextView::new("Successfully Pushed"))
//                     .title("Status")
//                     .button("Ok", |s| s.pop_layer()),
//             );
//         });
//     }
// }

// pub fn update_from_git(buffer: &mut Buffer, tv: &mut TextView) {
//     buffer.clear();
//     buffer.set_remote(&format!("Remote: \t{}", Git::get_remote()));
//     buffer.set_local(&format!(
//         "Local:  \t{} {}",
//         Git::get_branch_data().0,
//         Git::get_directory()
//     ));
//     buffer.set_head(&format!("Head:   \t{}", Git::get_branch_data().1));

//     for (t, f, s) in Git::get_status() {
//         if s {
//             buffer.stage(f, t);
//         } else {
//             match t {
//                 ChangeType::Untracked => buffer.add_untracked(f),
//                 _ => buffer.add_unstaged(f, t),
//             }
//         }
//     }

//     tv.set_content(buffer.render());
// }

fn main() {

    /* Print --help/ --version and ignore matches */
    let _ = App::new(APP_NAME)
        .version(VERSION)
        .author(DEVELOPER)
        .about(DESCRIPTION)
        .get_matches();

    // TODO: Handle config creation/ loading

    /* Create a buffer */
    let buffer = Buffer::new();

    /* Initialise the main Ui (blocks) */
    let mut ui = Ui::new(FoolTheme::Dark, buffer);
    ui.run();

    // println!("Creating buffer...");
    // buffer.update();

    // /* Set the theme to our custom one */
    // siv.set_theme(theme::setup());

    // /* Register keybinding callbacks */
    // // register_callbacks(&mut siv, &buffer);
    // let size = siv.screen_size();

    // {
    //     let b = Arc::clone(&buffer);
    //     let mut text_view = TextView::new("<PLACEHOLDER>"); //with_id("text_area");
    //     // update_from_git(&mut buffer.lock().unwrap(), &mut text_view);

    //     text_view.set_scrollable(false);
    //     let view = BoxView::with_fixed_size(
    //         (size.x - 2, size.y - 2),
    //         Panel::new(text_view.with_id("text_area")),
    //     );

    //     siv.add_layer(view);
    // }

    // siv.add_global_callback('Q', |s| s.quit());
    // siv.run();
}
