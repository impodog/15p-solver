use crate::*;

impl Puzzle {
    pub fn heu(&self) -> usize {
        let mut value = 0;
        let mut pos = 0;
        for db in 0..DB_NUMBER {
            let mut key = DbKey::default();
            for index in 0..DB_SIZE {
                key.0[index + 1] = self.slider_to_pos()[Puzzle::initial_slider(pos + index)];
            }
            key.0[0] = self.slider_to_pos()[0];

            // Changes the last value of key when it represents a duplicate space
            if db + 1 == DB_NUMBER {
                key.0[DB_SIZE] = SQUARE_LENGTH - 1;
            }

            // println!("Database {db}, key {key:?}");
            let db = DB_LIST
                .get(db)
                .expect("database should before index DB_SIZE");
            value += db.get(&key).copied().unwrap_or_else(|| {
                println!("Query failed with {key:?}");
                0
            });
            pos += DB_SIZE;
        }
        value
    }
}
