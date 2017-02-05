extern crate rss;
extern crate url;
extern crate curl;
extern crate chrono;
extern crate glob;
extern crate term;

#[macro_use]
pub mod streamrss;
pub mod terminal;

use streamrss::*;

use std::io;
use std::io::Write;

fn main() {
  let mut buffer = String::new();
  let mut term: terminal::Curses = terminal::Curses::new();
  streamrss::load_feeds(&mut term.streams);

  loop {
    println!("What do you want to do?");
    println!("[1] List currently saved feeds");
    println!("[2] Download a new feed");
    println!("[3] Remove a feed");
    println!("[4] Show news");
    println!("[5] Leave");
    print!("> ");
    io::stdout().flush().ok().expect("Failed to flush stdout");

    io::stdin().read_line(&mut buffer).ok().expect("Failed to read line");
    buffer = String::from(buffer.trim_right());
    match buffer.as_str() {
      "1" => term.print_list_feeds(),
      "2" => term.download_feed(),
      "3" => {
        println!("Which feed do you want to remove?");
        println!("(Enter -1 if you just want to go back and see the list)");
        let mut value = String::new();
        io::stdin().read_line(&mut value).ok().expect("Failed to read line");
        value.pop();
        match value.parse::<i64>() {
          Ok(x) => {
            if x >= 0 {
              term.remove_feed(x as usize);
            }
          }
          Err(why) => println!("[ERR] {}", why),
        }
      },
      "4" => {
        for elem in term.streams.iter() {
          for item in elem.get_unread_articles() {
            match item.title {
              Some(ref v) => {
                print!("{}", v);
                match item.content {
                  Some(ref v) => {
                    println!("  ==> {}", v);
                    io::stdin().read_line(&mut buffer).unwrap();
                  }
                  None => {
                    match item.description {
                      Some(ref v) => {
                        println!(" ==> {}", v);
                        io::stdin().read_line(&mut buffer).unwrap();
                      },
                      None => {}
                    }
                  }
                }
              },
              None => {}
            }
          }
        }
      },
      "5" => break,
      _ => println!("WRONG ANSWER :'( ")
    }
    buffer.clear();
  }
}
