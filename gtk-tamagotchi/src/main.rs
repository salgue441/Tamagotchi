// GTK
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Orientation, SpinButton, WindowPosition};

/// Represents a Tamagotchi
struct Tamagotchi {
    name: String,
    age: u8,
    hunger: u8,
    happiness: u8,
    health: u8,
    weight: u8,
    is_alive: bool,
}

impl Tamagotchi {
    /// Creates a new Tamagotchi
    fn new() -> self {
        Tamagotchi {
            name: String::from(""),
            age: 0,
            hunger: 0,
            happiness: 0,
            health: 0,
            weight: 0,
            is_alive: true,
        }
    }

    /// Feeds the Tamagotchi, decreasing its hunger and increasing its weight
    fn feed(&mut self) {
        self.hunger -= 1;
        self.weight += 1;
    }

    /// Plays with the Tamagotchi, increasing its happiness and decreasing its weight
    fn play(&mut self) {
        self.happiness += 1;
        self.weight -= 1;
    }

    /// Puts the Tamagotchi to sleep, decreasing happiness and increasing health.
    fn sleep(&mut self) {
        self.happiness -= 1;
        self.health += 1;
    }

    /// Updates the Tamagotchi's state based on time passing.
    fn update(&mut self) {
        self.age += 1;
        self.hunger += 1;
        self.happiness -= 1;
        self.health -= 1;
        self.weight -= 1;

        if self.hunger > 10 || self.happiness > 10 || self.health > 10 || self.weight > 10 {
            self.is_alive = false;
        }
    }

    /// Checks if the Tamagotchi is alive.
    ///
    /// Returns `true` if the Tamagotchi is alive, `false` otherwise.
    fn is_alive(&self) -> bool {
        self.is_alive
    }

    /// Retrieves the name of the Tamagotchi.
    ///
    /// Returns the name of the Tamagotchi as a `String`.
    fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Sets the name of the Tamagotchi.
    ///
    /// # Arguments
    ///
    /// * `name` - The new name for the Tamagotchi.
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

fn main() {
    gtk::init().expect("Failed to initialize GTK...");

    let app = Application::new(
        Some("com.carlossalguero.gtk-tamagotchi"),
        Default::default(),
    )
    .expect("Failed to create GTK Application...");

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);

        window.set_title("Tamagotchi");
        window.set_default_size(800, 600);

        window.show_all();
    });

    app.run(&[]);
}
