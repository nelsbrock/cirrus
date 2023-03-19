use anyhow::anyhow;
use cirrus::config::Config;
use cirrus::database::Database;
use cirrus::util;
use clap::{Parser, Subcommand};
use diesel::ExpressionMethods;
use diesel::{QueryDsl, RunQueryDsl};
use std::borrow::Cow;
use std::path::PathBuf;

#[derive(clap::Parser)]
#[command(author, version, about)]
struct Args {
    /// Use the specified configuration file.
    ///
    /// If this is not specified, cirrus will look for the configuration file in various places,
    /// depending on the OS.
    #[arg(long, value_parser, value_name = "PATH")]
    config: Option<PathBuf>,

    /// Use the specified database url.
    ///
    /// This overrides the database url specified in the configuration.
    #[arg(long, value_parser, value_name = "PATH")]
    database: Option<String>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run the cirrus server.
    Run,
    /// Create the configuration file.
    CreateConfig {
        path: PathBuf,

        /// Overwrite the configuration file if it already exists
        #[arg(long)]
        overwrite: Option<bool>,
    },
    /// Manage users.
    User {
        #[command(subcommand)]
        command: UserCommand,
    },
}

#[derive(Subcommand)]
enum UserCommand {
    /// Create a new user.
    Create {
        /// Name of the new user.
        name: String,

        /// Password of the new user. (Note: Only intended for usage in scripts.)
        ///
        /// If this is not specified, the password is prompted.
        #[arg(long)]
        password: Option<String>,
    },
    /// Delete a user.
    Delete {
        /// Name of the user to delete.
        name: String,
    },
    /// Lists all users.
    List,
    /// Set the password of a user.
    SetPassword {
        /// Name of the user.
        name: String,

        /// New password of the user. (Note: Only intended for usage in scripts.)
        ///
        /// If this is not specified, the password is prompted.
        #[arg(long)]
        password: Option<String>,
    },
}

async fn run(mut ctx: Context) -> anyhow::Result<()> {
    todo!()
}

fn user_create(mut ctx: Context, name: &str, password: Option<&str>) -> anyhow::Result<()> {
    use cirrus::database::{models::UserInsert, schema::users::{self, dsl}};

    let user_exists: bool =
        diesel::select(diesel::dsl::exists(dsl::users.filter(dsl::name.eq(name))))
            .get_result(ctx.database.conn())?;

    if user_exists {
        return Err(anyhow!("A user with name {name} already exists."));
    }

    let password = match password {
        None => Cow::Owned(rpassword::prompt_password(
            "Enter a password for the new user (hidden input): ",
        )?),
        Some(password) => Cow::Borrowed(password),
    };

    let user_insert =
        UserInsert::new_with_password(Cow::Borrowed(name), password.as_ref().as_ref())?;

    diesel::insert_into(users::table)
        .values(&user_insert)
        .execute(ctx.database.conn())?;

    Ok(())
}

fn user_delete(mut ctx: Context, name: &str) -> anyhow::Result<()> {
    use cirrus::database::schema::users::dsl;

    let user_exists: bool =
        diesel::select(diesel::dsl::exists(dsl::users.filter(dsl::name.eq(name))))
            .get_result(ctx.database.conn())?;

    if !user_exists {
        return Err(anyhow!("No user found with the name {name}."));
    }

    diesel::delete(dsl::users.filter(dsl::name.eq(name))).execute(ctx.database.conn())?;

    Ok(())
}

fn user_list(mut ctx: Context) -> anyhow::Result<()> {
    use cirrus::database::{models::User, schema::users::dsl};

    let result = dsl::users.load::<User>(ctx.database.conn())?;
    for user in result {
        println!("{}", user.name())
    }

    Ok(())
}

fn user_set_password(mut ctx: Context, name: &str, password: Option<&str>) -> anyhow::Result<()> {
    use cirrus::database::schema::users::dsl;

    let password = match password {
        None => Cow::Owned(rpassword::prompt_password(
            "Enter a password for the new user (hidden input): ",
        )?),
        Some(password) => Cow::Borrowed(password),
    };

    let new_password_hash = util::hash_password(password.as_ref().as_ref())?;

    diesel::update(dsl::users.find(name))
        .set(dsl::password_hash.eq(new_password_hash))
        .execute(ctx.database.conn())
        .map(|_| ())
        .map_err(anyhow::Error::msg)
}

struct Context {
    config: Config,
    database: Database,
}

impl Context {
    fn init(args: &Args) -> anyhow::Result<Self> {
        let config = Config::parse(args.config.as_deref())?;
        let database =
            Database::connect(args.database.as_ref().unwrap_or(config.database().url()))?;
        Ok(Self { config, database })
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = Args::parse();

    match args.command {
        Command::Run => run(Context::init(&args)?).await,

        Command::CreateConfig {
            ref path,
            overwrite,
        } => Config::create(path, overwrite),

        Command::User { ref command } => match command {
            UserCommand::Create {
                ref name,
                ref password,
            } => user_create(Context::init(&args)?, name, password.as_deref()),

            UserCommand::Delete { ref name } => user_delete(Context::init(&args)?, name),

            UserCommand::List => user_list(Context::init(&args)?),

            UserCommand::SetPassword {
                ref name,
                ref password,
            } => user_set_password(Context::init(&args)?, name, password.as_deref()),
        },
    }
}
