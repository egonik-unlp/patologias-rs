use crate::{
    core::database::repository::AsyncLayer,
    users::{
        dto::NewUserDto,
        model::{NewUser, User},
        repository::{SessionRepository, UserRepository},
    },
};

use anyhow::Context;
use argon2::{
    password_hash::{phc::PasswordHash, PasswordHasher, PasswordVerifier},
    Argon2,
};

#[derive(Debug, Clone)]
pub struct UserService {
    users: UserRepository,
    sessions: SessionRepository,
}

impl UserService {
    pub fn new(users: UserRepository, sessions: SessionRepository) -> Self {
        Self { users, sessions }
    }

    pub async fn create_user(&self, user: NewUserDto, password: String) -> anyhow::Result<User> {
        let password_hash = hash_password(&password).context("Hashing password")?;
        let NewUserDto {
            username,
            full_name,
            role,
            active,
        } = user;
        let user = NewUser {
            username,
            full_name,
            role: role.into(),
            password_hash,
            active,
        };
        self.users.create(user).await
    }

    /// `Ok(None)` si el usuario no existe, está inactivo o la contraseña no coincide.
    pub async fn check_password(
        &self,
        username: &str,
        password: &str,
    ) -> anyhow::Result<Option<User>> {
        let Some(user) = self.users.find_by_username(username).await? else {
            return Ok(None);
        };
        let ok = user.active && verify_password(password, &user.password_hash);
        Ok(ok.then_some(user))
    }

    pub async fn login_user(&self, user_name: String, password: &str) -> anyhow::Result<()> {
        todo!()
    }
}

fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    // Argon2id por defecto; genera una sal aleatoria (feature `getrandom`).
    let hash = Argon2::default().hash_password(password.as_bytes())?;
    Ok(hash.to_string())
}

fn verify_password(password: &str, stored_hash: &str) -> bool {
    let Ok(parsed_hash) = PasswordHash::new(stored_hash) else {
        return false;
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
