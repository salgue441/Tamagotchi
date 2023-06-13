// Standard library
use std::cell::RefCell;
use std::rc::Rc;

// GTK
use gtk::prelude::*;
use gtk::{Align, Application, ApplicationWindow, Button, Grid};

// Project files
mod tamagotchi;
use tamagotchi::Tamagotchi;

fn main() {
    let application = Application::builder()
        .application_id("carlossalguero.gtk-tamagotchi")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Tamagotchi")
            .default_width(800)
            .default_height(600)
            .build();

        // Modifiable structure object
        let tamagotchi = Rc::new(RefCell::new(Tamagotchi::new()));

        // Create a grid
        let grid = Grid::new();
        grid.set_row_spacing(10);
        grid.set_column_spacing(10);

        // Feed, Play, sleep buttons
        let feed_button = Button::with_label("Feed");
        feed_button.set_halign(Align::Center);
        feed_button.connect_clicked({
            let tamagotchi = Rc::clone(&tamagotchi);
            move |_| {
                tamagotchi.borrow_mut().feed();
            }
        });

        let play_button = Button::with_label("Play");
        play_button.set_halign(Align::Center);
        play_button.connect_clicked({
            let tamagotchi = Rc::clone(&tamagotchi);
            move |_| {
                tamagotchi.borrow_mut().play();
            }
        });

        let sleep_button = Button::with_label("Sleep");
        sleep_button.set_halign(Align::Center);
        sleep_button.connect_clicked({
            let tamagotchi = Rc::clone(&tamagotchi);
            move |_| {
                tamagotchi.borrow_mut().sleep();
            }
        });

        // Add buttons to grid
        grid.attach(&feed_button, 0, 0, 1, 1);
        grid.attach(&play_button, 1, 0, 1, 1);
        grid.attach(&sleep_button, 2, 0, 1, 1);

        // Add grid to window
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        vbox.pack_end(&grid, false, false, 0);

        window.add(&vbox);
        window.show_all();
    });

    application.run();
}
