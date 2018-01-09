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


fn register_callbacks(siv: &mut Cursive) {

    siv.add_global_callback('S', |_| {
        eprintln!("Staging ALL the file");
    });

    siv.add_global_callback('c', |_| {
        eprintln!("Creating a commit");
    });

    siv.add_global_callback('C', |_| {
        eprintln!("Ammend committing");
    });

    siv.add_global_callback('p', |_| {
        eprintln!("Pushing to origin");
    });
}


fn main() {

    let mut b = Buffer::new();
    for (t, f, s) in Git::get_status() {
        // println!("{} {} {}", t, f, s);

        if s {
            b.stage(f, t);
        } else {
            match t {
                ChangeType::Untracked => b.add_untracked(f),
                _ => b.add_unstaged(f, t),
            }
        }
    }

    // return;
    // Deleted log.err false
    // Modified src/buffer.rs false
    // Modified src/git.rs false
    // Modified src/main.rs true
    // Modified src/main.rs false
    // Untracked .vscode/ false
    // Untracked meow false


    let mut siv = Cursive::new();
    siv.load_theme_file("assets/style.toml").unwrap();

    // b.add_untracked("src/control.rs".to_owned());
    // b.add_unstaged("log.err".to_owned(), ChangeType::Deleted);
    // b.add_unstaged("src/buffer.rs".to_owned(), ChangeType::Modified);
    // b.add_unstaged("src/test.rs".to_owned(), ChangeType::Added);
    // b.stage("src/main.rs".to_owned(), ChangeType::Modified);
    let buffer = Arc::new(Mutex::new(b));

    {
        let b = Arc::clone(&buffer);
        siv.add_global_callback('o', move |_| {
            let buffer = b.lock().unwrap();
            eprintln!("{:?}", buffer.get_selection());
        });
    }

    {
        let b = Arc::clone(&buffer);
        siv.add_global_callback('s', move |_| {
            let buffer = b.lock().unwrap();
            Git::stage(&buffer.get_selection().0);
            // eprintln!("{:?}", buffer.get_selection());
        });
    }

    {
        let b = Arc::clone(&buffer);
        siv.add_global_callback(Key::Up, move |siv| {
            let mut buffer = b.lock().unwrap();
            buffer.move_up();

            let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
            (&mut *tv).set_content(buffer.render());
        });
    }

    {
        let b = Arc::clone(&buffer);
        siv.add_global_callback(Key::Down, move |siv| {
            let mut buffer = b.lock().unwrap();
            buffer.move_down();

            let mut tv: ViewRef<TextView> = siv.find_id("text_area").unwrap();
            (&mut *tv).set_content(buffer.render());
        });
    }

    /* Register keybinding callbacks */
    register_callbacks(&mut siv);
    let size = siv.screen_size();
    
    {
        let b = Arc::clone(&buffer);
        let mut text_view = TextView::new(b.lock().unwrap().render()); //with_id("text_area");
        text_view.set_scrollable(false);
        let view = BoxView::with_fixed_size(
            (size.x - 8, size.y - 4),
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
