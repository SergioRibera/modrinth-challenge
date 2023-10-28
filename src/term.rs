use std::io::{self, Read, Write};
use std::str::FromStr;

use colored::Colorize;

use crate::error::Modrinth as MError;
use crate::models::project::Project;

pub fn ask<T>(question: &str, validation: impl Fn(&str) -> bool) -> Result<T, MError>
where
    T: FromStr,
{
    let mut is_valid = false;
    let mut answer = String::new();
    let mut stdin = io::stdin();

    while !is_valid {
        sentence(question);
        let mut buf = [0u8; 1024];
        io::stdout().flush()?;
        if let Ok(c) = stdin.read(&mut buf) {
            if c > 0 {
                answer = String::from_utf8(buf[..c - 1].to_vec())?;
                is_valid = validation(answer.trim());
            } else {
                println!("\t{}", "You need write something".magenta());
            }
        } else {
            println!("\tAn error ocurred");
        }
    }

    T::from_str(answer.as_str()).map_err(|_| MError::RequestInvalidParse(answer))
}

pub fn sentence(s: &str) {
    print!("\n\t{}{}: ", "::".bold().blue(), s.bright_blue());
}

pub fn note(n: &str) {
    print!("\n{} {}", "::".bold().blue(), n.bold().white());
}

pub fn welcome() {
    print!(
        "\t\t\t{} to {}\n\n",
        "Welcome".bold().blue(),
        "modrinth".bold().bright_green()
    );
}

pub fn show_project(p: &Project) {
    print!(
        "\n\t\t{} {} {}",
        "::".bold().blue(),
        "Name:".bold().white(),
        p.title
    );
    print!(
        "\n\t\t{} {} {}",
        "::".bold().blue(),
        "Description:".bold().white(),
        p.description
    );
    print!(
        "\n\t\t{} {} {}",
        "::".bold().blue(),
        "Downloads:".bold().white(),
        p.downloads
    );
    print!(
        "\n\t\t{} {} {}\n",
        "::".bold().blue(),
        "Categories:".bold().white(),
        p.categories.join(",")
    );
}

//
// Validators section
//
const ACCEPT: &[&str] = &["Yes", "yes", "Y", "y"];
const DECLINE: &[&str] = &["No", "no", "N", "n"];

pub struct MyBool(pub bool);

impl FromStr for MyBool {
    type Err = MError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if ACCEPT.contains(&s) {
            Ok(Self(true))
        } else if DECLINE.contains(&s) {
            Ok(Self(false))
        } else {
            Err(MError::RequestInvalidParse(s.to_string()))
        }
    }
}

pub fn accept(s: &str) -> bool {
    !s.is_empty() || MyBool::from_str(s).map_or(false, |_| true)
}
