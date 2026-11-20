//! Crea un usuario admin directamente en la base (para el primer admin, o para
//! recuperar el acceso). `create_user` en la web exige un admin logueado.
//!
//!     cargo run --features ssr --bin create-admin -- <username> "<nombre completo>"
//!
//! La contraseña se lee de stdin (una línea), así no queda en el historial de
//! la shell ni en la lista de procesos.

use std::io::{self, BufRead, Write};

use patologias_rs::core::config::model::Config;
use patologias_rs::core::database::connection::get_connection_pool;
use patologias_rs::users::dto::{NewUserDto, RoleDto};
use patologias_rs::users::repository::{SessionRepository, UserRepository};
use patologias_rs::users::service::UserService;

const MIN_PASSWORD_LEN: usize = 8;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let (Some(username), Some(full_name), None) = (args.next(), args.next(), args.next()) else {
        anyhow::bail!("uso: create-admin <username> \"<nombre completo>\"");
    };

    eprint!("Contraseña para {username}: ");
    io::stderr().flush()?;
    let mut password = String::new();
    io::stdin().lock().read_line(&mut password)?;
    let password = password.trim_end_matches(['\r', '\n']).to_owned();
    anyhow::ensure!(
        password.chars().count() >= MIN_PASSWORD_LEN,
        "la contraseña debe tener al menos {MIN_PASSWORD_LEN} caracteres"
    );

    let pool = get_connection_pool(Config::from_env().database_url);
    tokio::runtime::Runtime::new()?.block_on(async {
        let users = UserRepository::new(pool.clone());
        anyhow::ensure!(
            users.find_by_username(&username).await?.is_none(),
            "ya existe un usuario llamado {username:?}"
        );

        let service = UserService::new(users, SessionRepository::new(pool));
        let user = service
            .create_user(
                NewUserDto {
                    username,
                    full_name,
                    role: RoleDto::Admin,
                    active: true,
                },
                password,
            )
            .await?;
        println!("Admin creado: {} (id {})", user.username, user.id);
        Ok(())
    })
}
