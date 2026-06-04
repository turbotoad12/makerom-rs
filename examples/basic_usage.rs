//! Basic example of using the makerom bindings

use makerom::UserSettings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating makerom user settings...");
    
    let _settings = UserSettings::new()?;
    
    println!("Successfully created user settings!");
    println!("\nNext steps:");
    println!("1. Link against the compiled makerom C library");
    println!("2. Update build.rs with your makerom library path");
    println!("3. Implement additional safe wrappers as needed");
    
    Ok(())
}
