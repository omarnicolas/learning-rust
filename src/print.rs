pub fn run() {
  // Print to console
  println!("Hello from the print.rs file");

  // Basic formatting
  println!("Number: {}", 1);
  println!("{} is from {}", "Omar", "Barcelona");

  // Positional arguments
  println!(
    "{0} is from {1} and {0} likes to {2}",
    "Omar", "Barcelona", "code"
  );

  // Named arguments
  println!(
    "{name} likes to play {activity}",
    name = "Omar",
    activity = "football"
  );

  // Placeholder traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

  // Placeholder for debug trait
  println!("{:?}", (12, true, "hello"));

  // Basic math
  println!("10 + 10 = {}", 10 + 10);
}
