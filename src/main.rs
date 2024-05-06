mod password_generator;

use clipboard::{ClipboardContext, ClipboardProvider};
use password_generator::PasswordGenerator;

fn main() {
    let generator = PasswordGenerator::new(20, true, true, true, true);
    let password = generator.generate();
    println!("Generated password: {}", password);

    // Copy password to the clipboard
    let mut clipboard = ClipboardContext::new().expect("Failed to initialize clipboard");
    clipboard
        .set_contents(password)
        .expect("Failed to copy to clipboard");
    println!("Password has been copied to clipboard!");
}
