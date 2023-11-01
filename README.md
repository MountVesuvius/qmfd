# qmfd
`qmfd` is a **Q**uick **M**eme **F**inding **D**evice to help me organise and search reaction images and memes at a faster rate. i hate how discord handles gifs and doesn't really let you store images. I don't want to resort to using all these different keyboard applications as that is a right pain. therefore, i'm building this system so i don't need to spend 3 mins scrolling through 1000s of images trying to find what i want.

reaction images are only funny if you're fast and accurate.

## Feature Checklist
This is just so i can check things off as i build them and give myself a sense of achomplishment. I know, very professional indeed

> ---
> **Features**
> - [ ] Search for image based on name
> - [ ] Search for image based on tag name
> - [x] Add a new image
> - [ ] Add a new image and rename the image
> - [ ] Add a new image and assign preexisting tags
> - [ ] Create a new tag
> - [ ] Rename image
> - [ ] Rename tag
> - [x] Delete image
> - [ ] Delete tag
> ---


## Usability & Commands
Initially this will just be a command line tool, until i bother getting around to making a GUI (thinking Tauri + Svelte would be kinda cool).

will be used to search for images quickly using the tags or names. here are the commands

| Feature                                               | Short Form                           | Long Form                                                 |
|-------------------------------------------------------|--------------------------------------|----------------------------------------------------------|
| Search for image based on name                        | `-sn <image_name>`                   | `--search-name <image_name>`                              |
| Search for image based on tag name                    | `-st <tag_name>`                     | `--search-tag <tag_name>`                                 |
| Add a new image                                       | `-a <path_to_image>`                 | `--add <path_to_image>`                                   |
| Add a new image and rename the image                  | `-a <path_to_image> -n <new_image_name>` | `--add <path_to_image> --name <new_image_name>`       |
| Create a new tag                                      | `-ct <tag_name>`                     | `--create-tag <tag_name>`                                 |
| Rename image                                          | `-rn <old_name> <new_name>`          | `--rename-name <old_name> <new_name>`                     |
| Rename tag                                            | `-rt <old_tag> <new_tag>`            | `--rename-tag <old_tag> <new_tag>`                        |
| Delete image                                          | `-dn <image_name>`                   | `--delete-name <image_name>`                              |
| Delete tag                                            | `-dt <tag_name>`                     | `--delete-tag <tag_name>`                                 |



#### `qmfd`
Alone it will just check if a database has been initialized, and spit out some basic details if it has.
- if a database has been initialized
- how many images are currently stored
- some help text to the degree of 'for more commands/help use qmdf --help'


# DB Diagram
```
+-----------+        +-------------+        
|  images   |        | image_tags  |        +--------+ 
+-----------+        +-------------+        | tags   |
| image_id  |<------>| image_id    |        +--------+ 
| image_name|        | tag_id      |<------>| tag_id |
| image_path|        +-------------+        |tag_name|
| added_date|                               +--------+
+-----------+
```
note: i had a tool make this, but then had to line up all the arrows :sob:

## basic idea
- the images table contains details about each image, including its ID, name, path, and the date it was added.
- the tags table contains a list of unique tags, each with an ID and a name.
- the image_tags junction table establishes the many-to-many relationship between images and tags. Each row in this table links a specific image with a specific tag.

wildcards and LIKE should be enough to handle basic name, and tag searching

i can add functional ui and the likes when this works

## SQL commands
### Creation
#### images
```sql
CREATE TABLE IF NOT EXISTS images (
    image_id INTEGER PRIMARY KEY,
    image_name TEXT NOT NULL,
    image_path TEXT NOT NULL,
    added_date DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

#### image_tags
```sql
CREATE TABLE IF NOT EXISTS image_tags (
    image_id INTEGER,
    tag_id INTEGER,
    FOREIGN KEY (image_id) REFERENCES images(image_id),
    FOREIGN KEY (tag_id) REFERENCES tags(tag_id),
    UNIQUE (image_id, tag_id)
);
```

#### tags
```sql
CREATE TABLE IF NOT EXISTS tags (
    tag_id INTEGER PRIMARY KEY,
    tag_name TEXT UNIQUE NOT NULL
);
```




---
note to reader: this is my first rust project, coming from a JS/Python dev. if i'm doing something wrong, please let me know. idk any crates or fancy rust tech