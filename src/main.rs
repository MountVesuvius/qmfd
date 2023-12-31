mod schema;
mod copy_image;

extern crate regex;
extern crate rusqlite;

use std::fmt;
use std::fs;
use std::path::Path;

use rusqlite::{params, Connection, Result};
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

// Implementation of images
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
            println!("{}\n", image?);
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

    fn search_image_by_name(&self, image_name: &str) -> Result<Vec<Image>> {
        let search_term = format!("%{}%", image_name);
        let mut stmt = self.conn.prepare(schema::images::NAME_BASED_SEARCH)?;
        let image_iter = stmt.query_map([search_term], |row| {
            Ok(Image {
                id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                added_date: row.get(3)?
            })
        })?;

        Ok(image_iter.collect::<Result<Vec<_>>>()?)
    }

    // Doesn't reference self, so not sure if this should be here. it does pertain to images though
    fn select_image_for_copy(images: Vec<Image>) -> Result<()> {
        for image in images {
            println!("{}", image);
        };


        Ok(())
    }

    fn _search_image_by_name(&self, image_name:&str) -> Result<Vec<Image>> {
        let search_term = format!("%{}%", image_name);
        let mut stmt = self.conn.prepare(schema::images::NAME_BASED_SEARCH)?;
        let image_iter = stmt.query_map([search_term], |row| {
            Ok(Image {
                id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                added_date: row.get(3)?
            })
        })?;

        let images = image_iter.collect::<Result<Vec<_>>>()?;

        // Not sure if this is good convention, i just have been trying to learn `match` more
        // If this is terrible code, i will change it in future
        match images.len() {
            0 => println!("No images found under that name"),
            1 => {
                copy_image::clipboard_image_copy(&images[0].path);
                println!("Image copied to clipboard");
            },
            _ => {
                // TODO: Future implement that they can just choose a number
                println!("Too many similar image names, please narrow down selection from this list");
                for (index, image) in images.iter().enumerate() {
                    println!("{}. {}", index, image.name);
                }
            }
        }

        Ok(images)
    }
}

// Implementation of tags
impl Database {
    fn show_tags(&self) -> Result<()> {
        let mut stmt = self.conn.prepare(schema::tags::GET_ALL_TAGS)?;
        let tag_iter = stmt.query_map([], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        for tag in tag_iter {
            println!("{}\n", tag?);
        }

        Ok(())
    }
    
    fn create_tag(&self, tag_name: &str) -> Result<()> {
        self.conn.execute(schema::tags::CREATE_TAG, [tag_name])?;

        Ok(())
    }

    fn remove_tag(&self, tag_name:&str) -> Result<()> {
        self.conn.execute(schema::tags::REMOVE_TAG, [tag_name])?;
        Ok(())
    }
}



const DATABASE:&str = "storage.db";
fn main() -> Result<()> {
    let database = Database::new(DATABASE.to_string())?;

    /*
    // Tag Testing
    database.create_tag("boom")?;
    database.show_tags()?;
    database.remove_tag("boom")?;
    database.show_tags()?;

    // Image Testing
    database.add_image("example_images/hecooks.png")?;
    database.add_image("example_images/maps.png")?;

    database.show_images()?;

    database.search_image_by_name("hecooks")?;

    database.remove_image("hecooks.png")?;
    database.remove_image("maps.png")?;

     */

    // database.show_images()?;
    let images = database.search_image_by_name("s")?;
    select_image_for_copy(&images);
    
    Ok(())
}