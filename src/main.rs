// There are excessive comments in this project, because this is my first proper rust project and idk what i'm doing.
// Trying to make it a learning experience
mod schema;

use std::fmt;
use std::fs;
use std::path::Path;

extern crate rusqlite;
use rusqlite::{params, Connection, Result};
use chrono::NaiveDate;

extern crate regex;
use regex::Regex;


struct Database {
    conn: Connection
}

struct Tag {
    id: usize,
    name: String
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "id: {}\nname: {}", self.id, self.name)
    }
}

struct Image {
    id: usize,
    name: String,
    path: String,
    added_date: String // TODO: I'll figure out date types a little later
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "id: {}\nname: {}\npath: {}\ndate: {}", self.id, self.name, self.path, self.added_date)
    }
}

// fn copy_files(source: String, location: String) -> std::io::Result<()> {
//     fs::copy(source, location)?;
// }

impl Database {
    fn new(database: String) -> Result<Self> {
        let conn = Connection::open(&database)?;

        conn.execute(schema::images::CREATE_TABLE, [])?;
        conn.execute(schema::images::CREATE_TAG_LINK_TABLE, [])?;
        conn.execute(schema::tags::CREATE_TABLE, [])?;

        Ok(Database {conn})
    }

    fn show_images(&self) -> Result<()> {
        let mut stmt = self.conn.prepare(schema::images::GET_ALL_IMAGES)?;

        let image_iter = stmt.query_map([], |row| {
            Ok(Image {
                id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                added_date: row.get(3)?
            })
        })?;

        for image in image_iter {
            println!("{}", image?);
        }

        Ok(())
    }

    fn add_image(&self, init_path: &str) -> Result<()> {
        // Check that the image exists first
        if !Path::new(init_path).exists() {
            println!("Specified file could not be found");
            return Ok(()) // Things are not ok, but idk how to return an error here. can't find it in the rusqlite docs
        }

        // Copy image into storage folder
        let re = Regex::new(r"[^/\\]*$").unwrap();
        let filename:Option<String> = match re.captures(init_path) {
            Some(caps) => Some(caps.get(0).unwrap().as_str().to_owned()),
            _ => return Ok(())
        };
        let destination = "storage/".to_owned() + filename.as_ref().unwrap();

        // I don't know what happens if the file is already called that so i am a little worried
        // Todo: Figure out a better option than just booting the file from being copied
        if !Path::new(&destination).exists() {
            self.conn.execute(schema::images::ADD_IMAGE, params![&filename, destination])?;
            let _ = fs::copy(init_path, destination);
        } else {
            println!("This file already exists in storage (or a file of similar name). Please rename the selected file, or choose to rename when adding this file");
        }

        Ok(())

    }

    fn remove_image(&self, image_name: &str) -> Result<()> {
        let count: usize = self.conn.query_row(schema::images::FIND_IMAGE_COUNT, [image_name], |row| row.get(0))?;

        if count == 1 {
            let image = self.conn.query_row(schema::images::SELECT_SPECIFIC_IMAGE, params![image_name], |row| {
                Ok(Image {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    path: row.get(2)?,
                    added_date: row.get(3)?
                })
            })?;

            self.conn.execute(schema::images::REMOVE_IMAGE, [image.name])?;
            let _ = fs::remove_file(image.path);
            println!("Object removed");
        } else if count > 1 {
            println!("There are too many objects registered under that name");
        } else {
            println!("No such object exists");
        }

        Ok(())
    }

    // fn search_images_by_name(&self, image_name: &str) -> Result<()> {
    //     let stmt
    // }
}

const DATABASE:&str = "storage.db";
fn main() -> Result<()> {
    let database = Database::new(DATABASE.to_string())?;

    // database.add_image("example.txt")?;
    database.show_images()?;
    // database.remove_image("example.txt")?;

    // println!("\n\n");

    // database.show_images()?;
    Ok(())
}