use std::io::{stdin, stdout, Write};
fn main() {
    main_menu();
}

//The main menu function that shows on startup to select the node.
fn main_menu(){
  print!("Welcome to Dialogger, would you like to create a (n)ew file, (l)oad, or (q)uit?\n");
  let choice = read_input();
  //Note, we need to convert our "String" to an &str pointer for this switch to work.
  match choice.as_str(){
    "n" => new_dialogue(),
    "l" => load_dialogue(),
    "q" => print!("Quitting Dialogger...\n"),
    _ => print!("Unknown command, quitting Dialogger...\n"),
  }

}

fn new_dialogue(){
  //pass
}
fn load_dialogue(){
  //pass
}

fn dialogue_adder(){
  //pass
}

//Reads in the input from the terminal, returns a string.
fn read_input() -> String{
  let mut s=String::new();
  let _=stdout().flush();

  //Read the line.
  stdin().read_line(&mut s).expect("Did not enter a correct string");
  if let Some('\n')=s.chars().next_back() {
    s.pop();
  }
  if let Some('\r')=s.chars().next_back() {
    s.pop();
  }

  return s;
}
