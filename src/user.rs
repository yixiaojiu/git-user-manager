use dirs::data_dir;
use serde::{Deserialize, Serialize};
use tokio::fs::{create_dir_all, File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub alias: String,
    pub name: String,
    pub email: String,
}

pub struct UserOperator {
    users: Vec<User>,
    file: File,
}

impl UserOperator {
    pub async fn new() -> Self {
        let mut path_buf = data_dir().unwrap();
        path_buf.push("gum");

        if !path_buf.exists() {
            create_dir_all(&path_buf).await.unwrap();
        }

        path_buf.push("data.json");

        let mut file: File;
        let users: Vec<User>;
        if !path_buf.exists() {
            file = File::create(&path_buf).await.unwrap();
            users = vec![];
        } else {
            file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(&path_buf)
                .await
                .unwrap();
            let mut json_str = String::new();
            file.read_to_string(&mut json_str).await.unwrap();
            users = serde_json::from_str(&json_str).unwrap();
        }

        Self { file, users }
    }

    pub async fn sync_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(&self.users)?;
        self.file.seek(std::io::SeekFrom::Start(0)).await?;
        self.file.set_len(0).await?;
        self.file.write_all(json.as_bytes()).await?;
        self.file.sync_data().await?;
        Ok(())
    }

    pub async fn set_user(&mut self, user: User) -> Result<(), Box<dyn std::error::Error>> {
        self.users.push(user);
        self.sync_config().await?;
        Ok(())
    }

    pub fn get_user(&self, alias: &str) -> &User {
        self.users.iter().find(|user| user.alias == alias).unwrap()
    }

    pub fn get_users(&self) -> &Vec<User> {
        &self.users
    }

    pub async fn remove_user(&mut self, alias: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.users.retain(|user| user.alias != alias);
        self.sync_config().await?;
        Ok(())
    }
}
