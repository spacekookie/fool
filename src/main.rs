extern crate cursive;

mod buffer;
use buffer::{Buffer, ChangeType};

mod git;
use git::Git;

use std::sync::{Mutex, Arc};

// Cursive UI includes
use cursive::Cursive;
use cursive::menu::MenuTree;
use cursive::event::Key;
use cursive::traits::*;
use cursive::views::*;



fn register_callbacks(siv: &mut Cursive, buffer: &Arc<Mutex<Buffer>>) {

    {
        // ARROW UP
        let b = Arc::clone(buffer);
        siv.add_global_callback(Key::Up, move |siv| {
            let mut buffer = b.lock().unwrap();
            buffer.move_up();

            let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
            (&mut *tv).set_content(buffer.render());
        });
    }

    {
        // ARROW DOWN
        let b = Arc::clone(buffer);
        siv.add_global_callback(Key::Down, move |siv| {
            let mut buffer = b.lock().unwrap();
            buffer.move_down();

            let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
            (&mut *tv).set_content(buffer.render());
        });
    }

    {
        // STAGE A SINGLE FILE
        let b = Arc::clone(buffer);
        siv.add_global_callback('s', move |siv| {
            let mut buffer = b.lock().unwrap();
            Git::stage(&buffer.get_selection().0);
            let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
            update_from_git(&mut buffer, &mut tv);
        });
    }

    {
        // UNSTAGE A SINGLE FILE
        let b = Arc::clone(buffer);
        siv.add_global_callback('u', move |siv| {
            let mut buffer = b.lock().unwrap();
            Git::unstage(&buffer.get_selection().0);
            let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
            update_from_git(&mut buffer, &mut tv);
        });
    }


    {
        // STAGE ALL
        let b = Arc::clone(buffer);
        siv.add_global_callback('S', move |siv| {
            let mut buffer = b.lock().unwrap();
            Git::stage_all();
            let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
            update_from_git(&mut buffer, &mut tv);
        });
    }

    {
        // UNSTAGE ALL
        let b = Arc::clone(buffer);
        siv.add_global_callback('U', move |siv| {
            let mut buffer = b.lock().unwrap();
            Git::unstage_all();
            let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
            update_from_git(&mut buffer, &mut tv);
        });
    }


    {
        // UNSTAGE ALL
        let b = Arc::clone(buffer);
        siv.add_global_callback('c', move |siv| {
            let b = Arc::clone(&b);
            let b2 = Arc::clone(&b);

            siv.add_layer(
                Dialog::new()
                    .title("Enter a commit message")
                    .padding((1, 1, 1, 0))
                    .content(
                        EditView::new()
                            .on_submit(move |siv, txt| {
                                let mut buffer = b2.lock().unwrap();
                                if txt == "" {
                                    siv.pop_layer();
                                    let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
                                    update_from_git(&mut buffer, &mut tv);
                                    return;    
                                }

                                Git::commit(txt);
                                siv.pop_layer();
                                let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
                                update_from_git(&mut buffer, &mut tv);
                            })
                            .with_id("commit")
                            .fixed_width(20),
                    )
                    .button("Ok", move |siv| {
                        let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
                        let mut bfo = b.lock().unwrap();
                        let msg =
                            siv.call_on_id("commit", |view: &mut EditView| view.get_content()).unwrap();
                            if *msg == "".to_owned() {
                                siv.pop_layer();
                                let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
                                update_from_git(&mut bfo, &mut tv);
                                return;    
                            }

                        Git::commit(&*msg);
                        update_from_git(&mut bfo, &mut tv);
                        siv.pop_layer();
                    }),
            );
        });

        {
            let b = Arc::clone(buffer);
            siv.add_global_callback('U', move |siv| {
                let mut buffer = b.lock().unwrap();
                Git::unstage_all();
                let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
                update_from_git(&mut buffer, &mut tv);
            });
        }
    }



    siv.add_global_callback('C', |siv| {
        eprintln!("Ammend committing");
    });

    siv.add_global_callback('p', |_| {
        eprintln!("Pushing to origin");
    });
}


pub fn update_from_git(buffer: &mut Buffer, tv: &mut TextView) {
    buffer.clear();
    buffer.set_remote(&format!("Remote: \t{}", Git::get_remote()));
    buffer.set_local(&format!(
        "Local:  \t{} {}",
        Git::get_branch_data().0,
        Git::get_directory()
    ));
    buffer.set_head(&format!("Head:   \t{}", Git::get_branch_data().1));

    for (t, f, s) in Git::get_status() {

        if s {
            buffer.stage(f, t);
        } else {
            match t {
                ChangeType::Untracked => buffer.add_untracked(f),
                _ => buffer.add_unstaged(f, t),
            }
        }
    }

    tv.set_content(buffer.render());
}


fn main() {
    let buffer = Arc::new(Mutex::new(Buffer::new()));
    let mut siv = Cursive::new();
    siv.load_theme_file("assets/style.toml").unwrap();

    /* Register keybinding callbacks */
    register_callbacks(&mut siv, &buffer);
    let size = siv.screen_size();

    {
        let b = Arc::clone(&buffer);
        let mut text_view = TextView::new("<PLACEHOLDER>"); //with_id("text_area");
        update_from_git(&mut buffer.lock().unwrap(), &mut text_view);

        text_view.set_scrollable(false);
        let view = BoxView::with_fixed_size(
            (size.x - 2, size.y - 2),
            Panel::new(text_view.with_id("text_area")),
        );

        siv.add_layer(view);
    }

    // The menubar is a list of (label, menu tree) pairs.
    siv.menubar()
        .add_subtree("Help", MenuTree::new())
        .add_subtree("Quit", MenuTree::new());

    siv.set_autohide_menu(false);
    siv.add_global_callback('Q', |s| s.quit());

    siv.run();
}


// This should be the header
// " Local:    master ~/Projects/code/fool
//  Head:     8ef7c41 Miep


//  Changes:
// ==> Modified   Cargo.lock
//     Modified   Cargo.toml
//     Modified   src/main.rs

//  # Cheat Sheet
//  #    s = stage file/section, S = stage all unstaged files
//  #    c = commit, C = commit -a (add unstaged)
//  #    P = push to upstream
//     "
