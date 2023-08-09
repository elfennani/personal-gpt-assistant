use rusqlite::{Connection, Error};
use uuid::Uuid;

#[derive(Debug)]
pub struct Chat {
    pub id: String,
    pub title: String,
}

#[derive(Debug)]
pub struct Message {
    id: String,
    pub content: String,
    pub name: Option<String>,
    pub role: String,
}

impl Chat {
    pub fn new(db: &Connection) -> Result<Chat, Error> {
        let mut statement = db
            .prepare("INSERT INTO chat (id, title) VALUES (?, ?)")
            .unwrap();

        let uuid = Uuid::new_v4();
        let title = "New Chat";

        let insertion = statement.execute([uuid.to_string(), title.to_string()])?;

        Ok(Chat {
            id: uuid.to_string(),
            title: title.to_string(),
        })
    }

    pub fn load(db: &Connection, id: &str) -> Result<Vec<Message>, Error> {
        let mut statement = db.prepare("SELECT * FROM message WHERE id = ?").unwrap();

        let message_iter = statement.query_map([id], |row| {
            Ok(Message {
                id: row.get(0)?,
                content: row.get(1)?,
                name: row.get(2)?,
                role: row.get(3)?,
            })
        });

        let messages = message_iter?.map(|row| row).collect();

        messages
    }

    pub fn load_all(db: &Connection) -> Result<Vec<Chat>, Error> {
        let mut statement = db.prepare("SELECT * FROM chat").unwrap();

        let chat_iter = statement.query_map([], |row| {
            Ok(Chat {
                id: row.get(0)?,
                title: row.get(1)?,
            })
        });

        let chats = {
            match chat_iter {
                Ok(mapped_rows) => {
                    let chats: Result<Vec<Chat>, Error> = mapped_rows.map(|row| row).collect();
                    chats
                }
                Err(err) => Err(err),
            }
        };

        return chats;
    }
}

pub fn init(data_dir: &str) -> Connection {
    let db = Connection::open(format!("{}/assistant.db", data_dir)).unwrap();

    db.execute_batch(
        "BEGIN;
        CREATE TABLE IF NOT EXISTS chat (
            id TEXT PRIMARY KEY,
            title TEXT DEFAULT 'New Chat'
        );
        CREATE TABLE IF NOT EXISTS message (
            id TEXT PRIMARY KEY,
            content TEXT,
            name TEXT,
            role TEXT
        );
        
        COMMIT;
    ",
    )
    .unwrap();

    return db;
}
