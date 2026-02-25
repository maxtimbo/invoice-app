pub struct Config {
    pub id: i64,
    pub smtp_server: String,
    pub port: u16,
    pub tls: bool,
    pub username: String,
    pub password: String,
    pub fromname: String,
}
