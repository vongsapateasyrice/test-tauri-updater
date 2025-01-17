#[tauri::command]
pub fn greet(name: &str) -> String {
  println!("Hello, {}! You've been greeted from Rust!", name);
  return format!("Hello, {}! You've been greeted from Rust!", name);
}

#[tauri::command]
pub fn add_number(num1: usize, num2: usize) -> usize {
  println!("called add number adding {} with {} equal {}", num1, num2, num1 + num2);
  return num1 + num2;
}