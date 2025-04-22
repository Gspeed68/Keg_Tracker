use std::io::{self, Write};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

struct Keg {
    id: u32,
    beer_type: String,
    size: f32,
    current_volume: f32,
    location: String,
    last_updated: u64,
}

struct KegTracker {
    kegs: HashMap<u32, Keg>,
    next_id: u32,
}

impl KegTracker {
    fn new() -> Self {
        KegTracker {
            kegs: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_keg(&mut self, beer_type: String, size: f32, location: String) {
        let keg = Keg {
            id: self.next_id,
            beer_type,
            size,
            current_volume: size,
            location,
            last_updated: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        self.kegs.insert(self.next_id, keg);
        self.next_id += 1;
        println!("Keg added successfully!");
    }

    fn update_keg(&mut self, id: u32, volume: f32) -> Result<(), &'static str> {
        if let Some(keg) = self.kegs.get_mut(&id) {
            if volume > keg.size {
                return Err("Volume cannot exceed keg size");
            }
            keg.current_volume = volume;
            keg.last_updated = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            Ok(())
        } else {
            Err("Keg not found")
        }
    }

    fn list_kegs(&self) {
        if self.kegs.is_empty() {
            println!("No kegs in the system.");
            return;
        }
        println!("\nCurrent Kegs:");
        println!("ID\tBeer Type\tSize\tCurrent\tLocation");
        println!("----------------------------------------");
        for keg in self.kegs.values() {
            println!(
                "{}\t{}\t{:.1}\t{:.1}\t{}",
                keg.id, keg.beer_type, keg.size, keg.current_volume, keg.location
            );
        }
    }
}

fn main() {
    let mut tracker = KegTracker::new();
    let mut input = String::new();

    loop {
        println!("\nKeg Tracker Menu:");
        println!("1. Add new keg");
        println!("2. Update keg volume");
        println!("3. List all kegs");
        println!("4. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => {
                print!("Enter beer type: ");
                io::stdout().flush().unwrap();
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let beer_type = input.trim().to_string();

                print!("Enter keg size (gallons): ");
                io::stdout().flush().unwrap();
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let size: f32 = input.trim().parse().unwrap_or(0.0);

                print!("Enter location: ");
                io::stdout().flush().unwrap();
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let location = input.trim().to_string();

                tracker.add_keg(beer_type, size, location);
            }
            "2" => {
                print!("Enter keg ID: ");
                io::stdout().flush().unwrap();
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let id: u32 = input.trim().parse().unwrap_or(0);

                print!("Enter new volume (gallons): ");
                io::stdout().flush().unwrap();
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let volume: f32 = input.trim().parse().unwrap_or(0.0);

                match tracker.update_keg(id, volume) {
                    Ok(_) => println!("Keg updated successfully!"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "3" => {
                tracker.list_kegs();
            }
            "4" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
