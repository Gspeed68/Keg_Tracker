# Keg Tracker

A robust command-line application for tracking beer kegs, their contents, and current volumes. This application helps manage inventory and monitor the status of beer kegs in real-time.

## Features

- Add new kegs with detailed information (beer type, size, location)
- Update keg volumes with validation
- List all kegs with formatted display
- Track last update time for each keg
- Simple and intuitive command-line interface

## Requirements

- Rust 1.70.0 or later
- Cargo (Rust's package manager)

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/keg_tracker.git
   cd keg_tracker
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   cargo run
   ```

## Code Structure

The application is organized into two main structures:

### 1. Keg Structure
```rust
struct Keg {
    id: u32,              // Unique identifier for the keg
    beer_type: String,    // Type of beer in the keg
    size: f32,            // Total capacity in gallons
    current_volume: f32,  // Current volume in gallons
    location: String,     // Physical location of the keg
    last_updated: u64,    // Unix timestamp of last update
}
```

### 2. KegTracker Structure
```rust
struct KegTracker {
    kegs: HashMap<u32, Keg>,  // Collection of all kegs
    next_id: u32,             // Next available ID for new kegs
}
```

## Implementation Details

### Key Functions

1. **Adding a Keg**
   - Generates a unique ID
   - Initializes current volume to full capacity
   - Records creation timestamp
   - Stores in the HashMap

2. **Updating Keg Volume**
   - Validates keg existence
   - Ensures new volume doesn't exceed keg size
   - Updates timestamp
   - Returns Result for error handling

3. **Listing Kegs**
   - Displays formatted table of all kegs
   - Shows ID, beer type, size, current volume, and location
   - Handles empty collection case

## Usage Examples

### Adding a New Keg
```
Keg Tracker Menu:
1. Add new keg
2. Update keg volume
3. List all kegs
4. Exit
Enter your choice: 1

Enter beer type: IPA
Enter keg size (gallons): 15.5
Enter location: Bar 1
Keg added successfully!
```

### Updating Keg Volume
```
Enter your choice: 2

Enter keg ID: 1
Enter new volume (gallons): 10.5
Keg updated successfully!
```

### Viewing All Kegs
```
Enter your choice: 3

Current Kegs:
ID      Beer Type        Size    Current Location
----------------------------------------
1       IPA             15.5    10.5    Bar 1
```

## Error Handling

The application includes robust error handling:
- Volume updates cannot exceed keg size
- Invalid keg IDs are caught and reported
- Input parsing errors are handled gracefully
- Empty collections are handled appropriately

## Data Persistence

Note: The current implementation stores data in memory only. For production use, consider adding:
- File-based storage
- Database integration
- Serialization support

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

This project is open source and available under the MIT License.

## Author

Your Name <your.email@example.com> 