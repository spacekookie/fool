//! Handle all input given to fool

use cursive::Cursive;
use cursive::event::{Event, Key};
use cursive::traits::*;
use cursive::views::*;


use std::sync::{Arc, Mutex};

use super::workspace::Workspace;
use state::{Buffer, ChangeType, Git};

/// A simple enum that can represent different commands given
/// to other components
#[derive(Debug)]
pub enum Command {
    
    MoveUp,
    MoveDown,
    
    SelectUp,
    SelectDown,

    Dirty,
}

pub struct Input;
impl Input {
    pub fn register_quit(siv: &mut Cursive) {
        siv.add_global_callback('Q', |s| s.quit());
    }

    // pub fn register_push(siv: &mut Cursive, bf: Arc<Mutex<Workspace>>, ws: Arc<Mutex<Workspace>>) {
    //     siv.add_global_callback('P', move |siv| {
    //         Git::push();

    //         /* Resync the buffer */
    //         let mut buffer = bf.lock().unwrap();
    //         buffer.update();

    //         let mut workspace = ws.lock().unwrap();
    //         workspace.draw(&buffer, siv);
    //         drop(buffer);
    //         drop(workspace);

    //         siv.add_layer(
    //             Dialog::around(TextView::new("Successfully Pushed"))
    //                 .title("Status")
    //                 .button("Ok", |s| s.pop_layer()),
    //         );
    //     });
    // }

    pub fn register_move_up(siv: &mut Cursive, ws: Arc<Mutex<Workspace>>) {
        siv.add_global_callback(Key::Up, move |siv| {
            let mut workspace = ws.lock().unwrap();
            workspace.cmd(Command::MoveUp);
            workspace.draw(siv);
        });
    }

    pub fn register_move_down(siv: &mut Cursive, ws: Arc<Mutex<Workspace>>) {
        siv.add_global_callback(Key::Down, move |siv| {
            let mut workspace = ws.lock().unwrap();
            workspace.cmd(Command::MoveDown);
            workspace.draw(siv);
        });
    }

    pub fn register_select_up(siv: &mut Cursive, ws: Arc<Mutex<Workspace>>) {
        siv.add_global_callback(Event::Shift(Key::Up), move |siv| {
            let mut workspace = ws.lock().unwrap();
            workspace.cmd(Command::SelectUp);
            workspace.draw(siv);
        });
    }

    pub fn register_select_down(siv: &mut Cursive, ws: Arc<Mutex<Workspace>>) {
        siv.add_global_callback(Event::Shift(Key::Down), move |siv| {
            let mut workspace = ws.lock().unwrap();
            workspace.cmd(Command::SelectDown);
            workspace.draw(siv);
        });
    }

    pub fn register_refresh(siv: &mut Cursive, ws: Arc<Mutex<Workspace>>) {
        siv.add_global_callback('R', move |siv| {
            let mut workspace = ws.lock().unwrap();
            workspace.cmd(Command::Dirty);
            workspace.draw(siv);
        });
    }

    // pub fn register_move_down(
    //     siv: &mut Cursive,
    //     bf: Arc<Mutex<Workspace>>,
    //     ws: Arc<Mutex<Workspace>>,
    // ) {
    //     siv.add_global_callback(Key::Down, move |siv| {
    //         let mut workspace = ws.lock().unwrap();
    //         workspace.cmd(Command::Down);

    //         let mut buffer = bf.lock().unwrap();
    //         buffer.update();

    //         workspace.draw(&buffer, siv);
    //     });
    // }

    // pub fn register_stage(siv: &mut Cursive, bf: Arc<Mutex<Workspace>>, ws: Arc<Mutex<Workspace>>) {
    //     siv.add_global_callback('s', move |siv| {
    //         let mut buffer = bf.lock().unwrap();
    //         let mut workspace = ws.lock().unwrap();

    //         let pos = workspace.get_position();
    //         let (f, t) = buffer.get_element(pos);

    //         eprintln!("Staging {}", &f);

    //         if t == ChangeType::Conflicted {
    //             let dialog = Dialog::new()
    //                 .title("Can't stage conflicted file!")
    //                 .content(TextView::new("Quit fool and fix the issue first!"))
    //                 .padding((1, 1, 1, 0))
    //                 .button("Ok", |siv| siv.pop_layer());

    //             siv.add_layer(dialog);
    //             return;
    //         }

    //         Git::stage(&f);
    //         buffer.update();
    //         workspace.draw(&buffer, siv);
    //     });
    // }

    // pub fn register_unstage(siv: &mut Cursive, bf: Arc<Mutex<Workspace>>, ws: Arc<Mutex<Workspace>>) {
    //     siv.add_global_callback('u', move |siv| {
    //         let mut buffer = bf.lock().unwrap();
    //         let mut workspace = ws.lock().unwrap();

    //         let pos = workspace.get_position();
    //         let (f, _) = buffer.get_element(pos);
    //         workspace.cmd(Command::Down);

    //         Git::unstage(&f);
    //         buffer.update();
    //         workspace.draw(&buffer, siv);
    //     });
    // }

    // pub fn register_commit(siv: &mut Cursive, bf: Arc<Mutex<Workspace>>, ws: Arc<Mutex<Workspace>>) {
    //     siv.add_global_callback('c', move |siv| {
    //         let mut size = siv.screen_size();
    //         if size.x > 80 {
    //             size.x = 80;
    //         }
            
    //         let inner_bf = Arc::clone(&bf);
    //         let inner_ws = Arc::clone(&ws);

    //         let inner_inner_bf = Arc::clone(&bf);
    //         let inner_inner_ws = Arc::clone(&ws);

    //         let dialog = Dialog::new()
    //             .title("Enter a commit message")
    //             .padding((1, 1, 1, 0))
    //             .content(
    //                 EditView::new()
    //                     .on_submit(move |siv, txt| {
    //                         let mut buffer = inner_bf.lock().unwrap();
    //                         let mut workspace = inner_ws.lock().unwrap();
    //                         if txt == "" {
    //                             siv.pop_layer();
    //                             return;
    //                         }

    //                         Git::commit(txt);
    //                         buffer.update();
    //                         workspace.draw(&buffer, siv);
    //                         siv.pop_layer();
    //                     })
    //                     .with_id("commit")
    //                     .fixed_width(20),
    //             )
    //             .button("Ok", move |siv| {
    //                 let mut buffer = inner_inner_bf.lock().unwrap();
    //                 let mut workspace = inner_inner_ws.lock().unwrap();

    //                 let msg = siv.call_on_id("commit", |view: &mut EditView| view.get_content())
    //                     .unwrap();
    //                 if *msg == "".to_owned() {
    //                     buffer.update();
    //                     workspace.draw(&buffer, siv);
    //                     siv.pop_layer();
    //                     return;
    //                 }

    //                 Git::commit(&*msg);
    //                 buffer.update();
    //                 workspace.draw(&buffer, siv);
    //                 siv.pop_layer();
    //             });

    //         siv.add_layer(BoxView::with_fixed_size(
    //             (size.x - 2, size.y / 2),
    //             Panel::new(dialog),
    //         ));
    //     });
    // }
}
