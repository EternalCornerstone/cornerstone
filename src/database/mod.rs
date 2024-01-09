pub mod postgresql;
pub mod seaorm;
pub mod sqlx;

#[derive(Debug)]
pub enum Database {
    PostgreSQL,
    SeaORM,
    SQLx,
    Nothing,
}

impl Database {
    pub fn as_database(str: &str) -> Database {
        match str { 
            "PostgreSQL" => Database::PostgreSQL,
            "SeaORM" => Database::SeaORM,
            "SQLx" => Database::SQLx,
            _ => Database::Nothing,
        }
    }

    pub fn config_variants_as_vec() -> Vec<Database> {
        vec![Database::PostgreSQL, Database::SeaORM, Database::SQLx, Database::Nothing]
    }

    pub fn give_users_database_options() {
        let variants = Database::config_variants_as_vec();
        let mut options = String::from("Choose a database: ");
        for (i, variant) in variants.iter().enumerate() {
            if variant.as_str() != "" {
                options += &format!("{} [{}]{}", variant.as_str(), variant.as_command(), if i < variants.len() - 2 { ", " } else { "" });
            }
        }
        println!("{}", options);
    }
    pub fn get_config(input: &str) -> &'static str {
        let trimmed = input.trim();
        match trimmed {
            "p" => Database::PostgreSQL.as_str(),
            "s" => Database::SeaORM.as_str(),
            "x" => Database::SQLx.as_str(),
            _ => Database::Nothing.as_str()
        }
    }
    
    pub fn as_string(&self) -> String {
        match self {
            Database::PostgreSQL => "PostgreSQL",
            Database::SeaORM => "SeaORM",
            Database::SQLx => "SQLx",
            Database::Nothing => "",
        }.to_string()
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Database::PostgreSQL => "PostgreSQL",
            Database::SeaORM => "SeaORM",
            Database::SQLx => "SQLx",
            Database::Nothing => "",
        }
    }
    pub fn as_command(&self) -> &'static str {
        match self {
            Database::PostgreSQL => "p", 
            Database::SeaORM => "s",
            Database::SQLx => "x",
            Database::Nothing => ""
        }
    }
}
