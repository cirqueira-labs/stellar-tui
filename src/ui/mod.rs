use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};
use prettytable::{cell, row, Table};
use std::io::{stdout, Result};
const ASCII_LOGIN_ART: &'static str = r#"
      .   . .   .       ..  .      ..       .         . .                 ..       .  
  . . . .  .     ..              .       .... .. .      .       .  .  .        .      
. . ..           . .     .   .  . ...+@@@@@@@@@@*..   .+=.       .        .  .  .  .  
  .  ....       .    .     .     .:@@#=.  ..  .:..:-#@@%-..            .. . ..     .  
 .      .               ...     .*@#:  .  .  .:=#@@#+:.... .    .     ..           . .
 .  ...  ..           . .      .#@*       .=*%@%+:..-*%@+.     .  ... .     .         
.               ..      .     .-%@. . .-#%@@+:..-*%@@@#....              . .  .    .  
.       .                      +@# :*@@@*.. .*@@@#:.#@+      .    ...  .  .    .      
  .           ..             ..*@@@@#-..:+@@@#-.. ..@@-    .        ..                
    .        .   .   . .    .=@@#=..:=#@@#=.  .    +@#.         .                .  . 
        ..  .. . . .        ..:.:=#@@#=:.  .  .  :*@#.    .  .    ..    .     .       
   .     ..      . .  ..   ..-#@@%=:..:..     .-#@@-.  .  .        . .  .             
.       .  .    .           .-+..  ..+@@@@@@@@@@*...       .  .      .  . .    .      
.      .   .            .      . ..      .::.   .  . .      .. .         ..           
     .   .   .  .   .     .            .  . .        .   .             .              
"#;

pub fn clear_terminal() {
    execute!(stdout(), Clear(ClearType::All));
}

pub fn display_ascii_art() {
    execute!(stdout(), MoveTo(0, 0));

    println!("{}", ASCII_LOGIN_ART);
}

pub fn choose_network() {
    let mut table = Table::new();

    table.add_row(row!["Choose network:"]);

    table.add_row(row!["1", "Mainnet"]);
    table.add_row(row!["2", "Testnet"]);
    table.add_row(row!["3", "Futurenet"]);
    table.add_row(row!["X", "Cancel and Exit"]);

    table.printstd();
}

pub fn show_menu() {
    let mut table = Table::new();

    table.add_row(row!["Menu:"]);

    table.add_row(row!["1", "Get Health"]);
    table.add_row(row!["2", "Show Account Information"]);
    table.add_row(row!["X", "Exit"]);

    table.printstd();
}
