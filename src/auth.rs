use crate::database::*;
use crate::models::*;
use crate::password::verify_password;
use diesel::prelude::*;

pub fn validate_login(username_inc: &str, password_inc: &str) -> bool {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let result = users
        .filter(email.eq(username_inc))
        .first::<User>(connection)
        .optional();

    match result {
        Ok(Some(user)) => {
            println!("Verifying against hash: {}", &user.password);
            let result = verify_password(password_inc, &user.password);
            match result {
                Ok(true) => {
                    return true;
                }
                Ok(false) => {
                    return false;
                }
                Err(_) => {
                    return false;
                }
            }
        }
        Ok(None) => {
            return false;
        }
        Err(_) => {
            // TODO: Later on add logging
            return false;
        }
    }
}
