// Standard library
use std::cell::RefCell;
use std::rc::Rc;

// GTK
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label};

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

        let tamagotchi = Rc::new(RefCell::new(Tamagotchi::new()));

        // Creating the UI
        let label_name = Label::new(Some(&tamagotchi.borrow().get_name()));

        let label_age = Rc::new(RefCell::new(Label::new(Some(
            &tamagotchi.borrow().get_age().to_string(),
        ))));

        let button_feed = Button::with_label("Feed");
        let button_play = Button::with_label("Play");
        let button_sleep = Button::with_label("Sleep");

        let grid = gtk::Grid::new();

        // Connecting the signals
        let tamagotchi_clone = tamagotchi.clone();
        let label_age_clone = label_age.clone();
        button_feed.connect_clicked(move |_| {
            tamagotchi_clone.borrow_mut().feed();
            label_age_clone
                .borrow_mut()
                .set_text(&tamagotchi_clone.borrow().get_age().to_string());
        });

        let tamagotchi_clone = tamagotchi.clone();
        let label_age_clone = label_age.clone();
        button_play.connect_clicked(move |_| {
            tamagotchi_clone.borrow_mut().play();
            label_age_clone
                .borrow_mut()
                .set_text(&tamagotchi_clone.borrow().get_age().to_string());
        });

        let tamagotchi_clone = tamagotchi.clone();
        let label_age_clone = label_age.clone();
        button_sleep.connect_clicked(move |_| {
            tamagotchi_clone.borrow_mut().sleep();
            label_age_clone
                .borrow_mut()
                .set_text(&tamagotchi_clone.borrow().get_age().to_string());
        });

        // Add widget to the window
        grid.attach(&label_name, 0, 0, 1, 1);

        grid.attach(&button_feed, 0, 2, 1, 1);
        grid.attach(&button_play, 0, 3, 1, 1);
        grid.attach(&button_sleep, 0, 4, 1, 1);

        // Show the window
        window.add(&grid);
        window.show_all();
    });

    application.run();
}
