/// Represents a Tamagotchi
pub struct Tamagotchi {
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
    pub fn new() -> Self {
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
    pub fn feed(&mut self) {
        self.hunger -= 1;
        self.weight += 1;
    }

    /// Plays with the Tamagotchi, increasing its happiness and decreasing its weight
    pub fn play(&mut self) {
        self.happiness += 1;
        self.weight -= 1;
    }

    /// Puts the Tamagotchi to sleep, decreasing happiness and increasing health.
    pub fn sleep(&mut self) {
        self.happiness -= 1;
        self.health += 1;
    }

    /// Updates the Tamagotchi's state based on time passing.
    pub fn update(&mut self) {
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
    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    /// Retrieves the name of the Tamagotchi.
    ///
    /// Returns the name of the Tamagotchi as a `String`.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Retrieves the age of the Tamagotchi.
    ///
    /// Returns the age of the Tamagotchi as a `u8`.
    pub fn get_age(&self) -> u8 {
        self.age
    }

    /// Sets the name of the Tamagotchi.
    ///
    /// # Arguments
    ///
    /// * `name` - The new name for the Tamagotchi.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
