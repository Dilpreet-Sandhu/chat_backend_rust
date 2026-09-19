use bcrypt::{DEFAULT_COST, hash};
use sqlx::{Pool, Postgres};

use crate::{
    auth::{dto::{UserLoginResponse, UserResponse}, extra::sign_access_token, model::User}, error::AppError,
};

pub struct AuthService;

impl AuthService {
    pub async fn create_user(
        (username, email, password,avatar ): (String, String, String,String),
        db_pool: &Pool<Postgres>,
    ) -> Result<(), AppError> {
        let hashed_pass = hash(password, DEFAULT_COST)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        let id = uuid::Uuid::new_v4();

        sqlx::query("INSERT INTO USERS (id,email,username,password,avatar) VALUES ($1,$2,$3,$4,$5)")
            .bind(id)
            .bind(email)
            .bind(username)
            .bind(hashed_pass)
            .bind(avatar)
            .execute(db_pool)
            .await?;

        Ok(())
    }
    pub async fn login_user(
        (identifier, password): (String, String),
        db_pool: &Pool<Postgres>,
        secret: &str,
    ) -> Result<UserLoginResponse, AppError> {

        println!("{}",&identifier);

        let db_user:Option<User>  = sqlx::query_as::<Postgres, User>(
            "SELECT id,username,email,password,avatar FROM USERS
        WHERE username = $1 OR email = $1",
        )
        .bind(identifier)
        .fetch_optional(db_pool)
        .await?;

        if db_user.is_none() {
            return Err(AppError::NotFound("user not found".to_string()))
        }

        let user = db_user.unwrap();

        let valid = bcrypt::verify(password, &user.password)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if !valid {
            return Err(AppError::WrongPassword);
        }

        let access_token = sign_access_token(user.id, secret)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;



        Ok(UserLoginResponse { access_token })
    }
    pub async fn get_users(db_pool : &Pool<Postgres>) -> Result<Vec<UserResponse>,AppError> {


        let users = sqlx::query_as::<Postgres,UserResponse>
        ("SELECT id,username,email FROM USERS")
        .fetch_all(db_pool)
        .await?;


        Ok(users)

    }
    pub async fn get_user(id : String,db_pool : &Pool<Postgres>) -> Result<UserResponse,AppError> {

        let user = sqlx::query_as::<Postgres,UserResponse>
        ("SELECT * FROM USERS WHERE id = $1")
        .bind(id)
        .fetch_one(db_pool)
        .await?;

        Ok(user)

    }
    
}
