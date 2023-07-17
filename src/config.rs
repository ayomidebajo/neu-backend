use serde_aux::field_attributes::deserialize_number_from_string;
use serde_derive::Deserialize;
use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgSslMode;
use sqlx::ConnectOptions;
use sqlx::PgPool;

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigJwt {
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
}

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub config: ConfigJwt,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    // Determine if we demand the connection to be encrypted or not
    pub require_ssl: bool,
}

impl DatabaseSettings {
    // Renamed from `connection_string_without_db`
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            // Try an encrypted connection, fallback to unencrypted if it fails
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .ssl_mode(ssl_mode)
    }
    // Renamed from `connection_string`
    pub fn with_db(&self) -> PgConnectOptions {
        let options = self.without_db().database(&self.database_name);
       options.clone().log_statements(tracing::log::LevelFilter::Trace);
        println!("options {:?}", &options.get_database());

        options
    }

    pub fn connection_string(&self) -> String {
        let connection_string;
        if self.require_ssl {
            connection_string = format!(
                "postgres://{}:{}@{}:{}/{}?sslmode={}",
                self.username,
                self.password,
                self.host,
                self.port,
                self.database_name,
                "disable".to_owned()
            );

            return connection_string;
        } else {
            connection_string = format!(
                "postgres://{}:{}@{}:{}/{}?sslmode={}",
                self.username,
                self.password,
                self.host,
                self.port,
                self.database_name,
                "disable".to_owned()
            );
            return connection_string;
        }
        // connection_string
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings_dev = config::Config::builder();
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");
    // let environment: String = std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "local".into());
    let environment = String::from("production");
    // println!("environment {:?}", environment.as_str());
    settings_dev
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(format!("{}.yaml", environment.as_str())),
        ))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()
        .expect("error loading source")
        .try_deserialize()
}

/// The possible runtime environment for our application.
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}

pub struct AppState {
    // pub db: PgPool,
    pub config: ConfigJwt,
}
