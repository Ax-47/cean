pub mod configuration {
    use dotenv::dotenv;
    use std::env;
    pub struct Config {
        pub discord_token: String,
        pub auth: String,
        pub ip: String,
        pub port: u16,
        pub debug: bool,
        pub worker: u8,
    }
    impl Config {
        pub fn from_env() -> Self {
            dotenv().ok();
            let discord_token =
                env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set in .env");
            let auth = env::var("AUTH").expect("AUTH must be set in .env");
            let ip = env::var("IP").expect("IP must be set in .env");
            let port = env::var("PORT")
                .expect("CHANNEL_ID must be set in .env")
                .parse::<u16>()
                .expect("PORT must be a valid u16 integer");
            let debug = env::var("DEBUG")
                .expect("DEBUG must be set in .env")
                .parse::<bool>()
                .expect("PORT must be a valid u16 integer");
            let worker = env::var("WORKER")
                .expect("DEBUG must be set in .env")
                .parse::<u8>()
                .expect("PORT must be a valid u16 integer");
            Self {
                discord_token,
                auth,
                ip,
                port,
                debug,
                worker,
            }
        }
    }
}
