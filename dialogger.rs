use std::io::{stdin, stdout, Write}; 
use dict::{ Dict, DictIface };

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
  //Initialize our level's data structure, a dictionary.
  let level = read_input("What is the name of your level?\n");
  let mut root = Dict::<String>::new();
  root.add("level".to_string(), level);
  //Iteratively add each character.
  let new_char = read_input("Do you want to add a new character? (y/n)\n");
  while (new_char => "y"){
    let name = read_input("What is the name of the character?\n");
    root.add(name, character_maker(name));
    let new_char = read_input("Do you want to add a new character? (y/n)\n");
  }
  let directory = "/home/fin/Development/Godot/ProjectWired/dialogue"
}
fn load_dialogue(){
  //pass
}

fn dialogue_adder(){
  //pass
}

//Creates character script.
fn character_maker() -> String{
  return None;
}


//Reads in the input from the terminal, returns a string.
fn read_input() -> String{
  let mut s=String::new();
  let _=stdout().flush();

  //Read the line.
  stdin().read_line(&mut s).expect("Did not enter a correct string.\n");
  if let Some('\n')=s.chars().next_back() {
    s.pop();
  }
  if let Some('\r')=s.chars().next_back() {
    s.pop();
  }

  return s;
}
