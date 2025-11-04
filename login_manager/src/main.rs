use clap::{Parser, Subcommand};
use authentication::{get_users, save_users, LoginRole};
use authentication::User;


#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all users.
    List,
    /// Add user.
    Add{
        /// The user login name
        username: String,
        /// The user password
        password: String,
        /// Option to make admin
        #[arg(long)]
        admin: Option<bool>,
    },
    /// Delete a user
    Delete {
        /// User to delete
        username: String,
    },
    /// Change Password
    ChangePassword {
        /// Users password to change
        username: String,
        /// New Password
        password: String,
    }

}

fn list_usrs() {
    println!("{:<20}{:<20}", "Username", "Role");
    println!("{:-<40}", "");

    let users = get_users();
    users
        .iter()
        .for_each(|(_, user)| {
            println!("{:<20}{:<20?}", user.username, user.role);
        })
}

fn add_user(username: String, password: String, admin: bool) {
    let mut users = get_users();
    let role = if admin {
        LoginRole::Admin
    } else {
        LoginRole::User
    };
    let user = User::new(&username, &password, role);
    users.insert(username, user);
    save_users(users);
}

fn delete_user(username: String) {
    let mut users = get_users();
    if users.contains_key(&username) {
        users.remove(&username);
        save_users(users);
    } else {
        println!("User {} not found", username);
    }
}
fn change_password(username: String, password: String) {
    let mut users = get_users();
    if let Some(user) = users.get_mut(&username) {
        user.password = authentication::hash_password(&password);
        save_users(users);
    } else {
        println!("User {} not found", username);
    }
}


fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => {
            list_usrs()
        }
        Some(Commands::Add{username, password, admin}) => {
            add_user(username, password, admin.unwrap_or(false))
        }
        Some(Commands::Delete{username, }) => {
            delete_user(username);
        }
        Some(Commands::ChangePassword{username, password}) => {
            change_password(username, password)
        }
        None => {
            println!("no command use --help for instructions")
        }
    }
}
