pub mod images {
    pub const CREATE_TABLE:&str = "
        CREATE TABLE IF NOT EXISTS images (
            image_id INTEGER PRIMARY KEY,
            image_name TEXT NOT NULL,
            image_path TEXT NOT NULL,
            added_date DATETIME DEFAULT CURRENT_TIMESTAMP
        )
    ";

    pub const CREATE_TAG_LINK_TABLE:&str = "
        CREATE TABLE IF NOT EXISTS image_tags (
            image_id INTEGER,
            tag_id INTEGER,
            FOREIGN KEY (image_id) REFERENCES images(image_id),
            FOREIGN KEY (tag_id) REFERENCES tags(tag_id),
            UNIQUE (image_id, tag_id)
        )
    ";

    pub const GET_ALL_IMAGES:&str = "
        SELECT * FROM images
    ";

    pub const REMOVE_IMAGE:&str = "
        DELETE FROM images WHERE image_name = ?1
    ";

    pub const ADD_IMAGE:&str = "
        INSERT INTO images (image_name, image_path)
        VALUES (?1, ?2)
    ";
    
    pub const FIND_IMAGE_COUNT:&str = "
        SELECT COUNT(*) FROM images WHERE image_name = ?1
    ";

    pub const SELECT_SPECIFIC_IMAGE:&str = "
        SELECT * FROM images WHERE image_name = ?1
    ";

    pub const NAME_BASED_SEARCH:&str = "
        SELECT * FROM images WHERE image_name LIKE ?1;
    ";
}

pub mod tags {
    pub const CREATE_TABLE:&str = "
        CREATE TABLE IF NOT EXISTS tags (
            tag_id INTEGER PRIMARY KEY,
            tag_name TEXT UNIQUE NOT NULL
        )
    ";

    pub const GET_ALL_TAGS:&str = "
        SELECT * FROM tags 
    ";

    pub const CREATE_TAG:&str = "
        INSERT OR IGNORE INTO tags (tag_name) VALUES (?1)
    ";

    pub const REMOVE_TAG:&str = "
        DELETE FROM tags WHERE tag_name = ?1
    ";

    pub const LINK_TAG_TO_IMAGE:&str = "
        INSERT INTO image_tags (image_id, tag_id) VALUES (?1, ?2)
    ";
}