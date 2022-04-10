use std::{env, io::Error, io::ErrorKind};

const username_env_key: &str = "ZKP_USERNAME";
const secret_env_key: &str = "ZKP_SECRET";

#[derive(Debug, PartialEq)]
pub struct UserInfo {
    username: String,
    secret: u128,
}

pub fn get_user_info_from_env_vars() -> Result<UserInfo, Vec<Error>> {
    let mut errors: Vec<Error> = Vec::new();

    let username = env::var(username_env_key);
    let secret = env::var(secret_env_key);

    if username.as_ref().is_err() {
        errors.push(Error::new(
            ErrorKind::NotFound,
            "Please make sure ZKP_USERNAME env variable is set",
        ));
    } else if &username.as_ref().unwrap().len() > &50 {
        errors.push(Error::new(
            ErrorKind::InvalidInput,
            "Please make sure ZKP_USERNAME is less than 50 characters",
        ));
    }

    if secret.as_ref().is_err() {
        errors.push(Error::new(
            ErrorKind::NotFound,
            "Please make sure ZKP_SECRET env variable is set",
        ));
    } else {
        let secret = secret.as_ref().unwrap().parse::<u128>();
        if secret.is_err() {
            errors.push(Error::new(
                ErrorKind::InvalidInput,
                "Please ensure ZKP_SECRET is a number (u128)",
            ));
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    let username = username.unwrap();
    let secret = secret.unwrap();
    Ok(UserInfo {
        username: username,
        secret: secret.parse::<u128>().unwrap(),
    })
}

#[cfg(test)]
mod unittests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn clear_env_vars() {
        env::remove_var(username_env_key);
        env::remove_var(secret_env_key);
    }

    #[test]
    fn username_and_secret_env_vars_must_exist() {
        // clear username and secret env vars upfront
        clear_env_vars();

        let res = get_user_info_from_env_vars();
        assert!(res.is_err());
        assert!(res
            .as_ref()
            .err()
            .unwrap()
            .iter()
            .any(|e| e.kind() == ErrorKind::NotFound
                && e.to_string() == "Please make sure ZKP_USERNAME env variable is set"));
        assert!(res
            .as_ref()
            .err()
            .unwrap()
            .iter()
            .any(|e| e.kind() == ErrorKind::NotFound
                && e.to_string() == "Please make sure ZKP_SECRET env variable is set"));
    }

    #[test]
    fn username_must_be_less_than_50_chars() {
        clear_env_vars();
        // provide an invalid username
        let string_longer_than_50_chars = (0..51).map(|_| "a").collect::<String>();
        env::set_var(username_env_key, string_longer_than_50_chars);
        let res = get_user_info_from_env_vars();
        assert!(res.is_err());
        assert!(res
            .err()
            .unwrap()
            .iter()
            .any(|e| e.kind() == ErrorKind::InvalidInput
                && e.to_string() == "Please make sure ZKP_USERNAME is less than 50 characters"));
    }

    #[test]
    fn secret_must_be_a_number() {
        clear_env_vars();
        // provide a non-number secret
        let invalid_secret = "invalid-string-secret";
        env::set_var(secret_env_key, invalid_secret);
        let res = get_user_info_from_env_vars();
        assert!(res.is_err());
        println!("{:?}", &res);
        assert!(res
            .err()
            .unwrap()
            .iter()
            .any(|e| e.kind() == ErrorKind::InvalidInput
                && e.to_string() == "Please ensure ZKP_SECRET is a number (u128)"));
    }

    #[test]
    fn valid_username_and_secret() {
        clear_env_vars();
        // provide a valid username and secret
        env::set_var(username_env_key, "testuser");
        env::set_var(secret_env_key, "1000");
        let res = get_user_info_from_env_vars();
        assert!(res.is_ok());
        assert_eq!(
            UserInfo {
                username: String::from("testuser"),
                secret: 1000
            },
            res.unwrap()
        );
    }
}
