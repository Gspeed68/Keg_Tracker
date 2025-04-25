//! # Keg Tracker
//!
//! A command-line application for tracking beer kegs and their contents.
//! This module provides functionality to manage keg inventory, including
//! adding new kegs, updating volumes, and listing current inventory.

use std::io::{self, Write};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a single beer keg with its properties and current state.
///
/// # Fields
///
/// * `id` - Unique identifier for the keg
/// * `beer_type` - Type of beer contained in the keg
/// * `size` - Total capacity of the keg in gallons
/// * `current_volume` - Current volume of beer in gallons
/// * `location` - Physical location of the keg
/// * `last_updated` - Unix timestamp of the last update
struct Keg {
    id: u32,
    beer_type: String,
    size: f32,
    current_volume: f32,
    location: String,
    last_updated: u64,
}

/// Manages a collection of kegs and provides operations for keg tracking.
///
/// # Fields
///
/// * `kegs` - HashMap storing all kegs with their IDs as keys
/// * `next_id` - Counter for generating unique IDs for new kegs
struct KegTracker {
    kegs: HashMap<u32, Keg>,
    next_id: u32,
}

impl KegTracker {
    /// Creates a new empty KegTracker instance.
    ///
    /// # Returns
    ///
    /// A new KegTracker with an empty collection of kegs and next_id set to 1.
    fn new() -> Self {
        KegTracker {
            kegs: HashMap::new(),
            next_id: 1,
        }
    }

    /// Adds a new keg to the tracker.
    ///
    /// # Arguments
    ///
    /// * `beer_type` - Type of beer in the keg
    /// * `size` - Total capacity of the keg in gallons
    /// * `location` - Physical location of the keg
    ///
    /// # Notes
    ///
    /// The new keg is initialized with:
    /// - A unique ID
    /// - Current volume equal to size (full)
    /// - Current timestamp
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

    /// Updates the volume of a specific keg.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the keg to update
    /// * `volume` - New volume in gallons
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the update was successful
    /// * `Err(&'static str)` - If the keg doesn't exist or the volume is invalid
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The keg ID doesn't exist
    /// - The new volume exceeds the keg's size
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

    /// Displays all kegs in a formatted table.
    ///
    /// # Output Format
    ///
    /// Prints a table with columns:
    /// - ID
    /// - Beer Type
    /// - Size (gallons)
    /// - Current Volume (gallons)
    /// - Location
    ///
    /// If no kegs exist, displays an appropriate message.
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

/// Main entry point of the application.
///
/// Provides a command-line interface for:
/// 1. Adding new kegs
/// 2. Updating keg volumes
/// 3. Listing all kegs
/// 4. Exiting the application
///
/// # User Interface
///
/// The application runs in a loop, presenting a menu and processing user input
/// until the user chooses to exit.
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
