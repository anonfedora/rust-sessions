//mod strings;
mod float;
mod signed;
mod string;
mod unsigned;

#[derive(Debug)]
struct User {
    id: u128,
    name: String,
    gender: Sex,
    marital_status: Status,
}

#[derive(Debug)]
enum Status {
    Married,
    Single,
    Divorce,
    Widow,
}

#[derive(Debug)]
enum Sex {
    Male,
    Female,
}

fn convert_usize_to_u128(len: usize) -> u128 {
    len as u128
}

fn user_checker(users_data: &mut Vec<User>, id: u128) -> Option<&mut User> {
    users_data.iter_mut().find(|x| x.id == id)
}

impl User {
    fn new_user(users_data: &Vec<User>, name: String, gender: Sex, marital_status: Status) -> User {
        User {
            id: convert_usize_to_u128(users_data.len()) + 1,
            name,
            gender,
            marital_status,
        }
    }

    fn update_user_name(name: String, users_data: &mut Vec<User>, id: u128) {
        // let updated_name = users_data.iter_mut().find(|x| x.id == id ).expect("invalid user id");
        let updated_name = user_checker(users_data, id);

        match updated_name {
            Some(user) => user.name = name,
            _ => println!("user with id {} does not exist", id),
        }
    }
    fn update_user_sex(gender: Sex, users_data: &mut Vec<User>, id: u128) {
        let updated_sex = user_checker(users_data, id);

        match updated_sex {
            Some(user) => user.gender = gender,
            _ => println!("user with id {} does not exist", id),
        }
    }
    fn update_user_marital_status(status: Status, users_data: &mut Vec<User>, id: u128) {
        let updated_status = user_checker(users_data, id);

        match updated_status {
            Some(user) => user.marital_status = status,
            _ => println!("user with id {} does not exist", id),
        }
    }
}

fn main() {
    unsigned::intro_to_u();
    signed::intro_to_i();
    float::intro_to_float();
    string::strings();
    let mut users_data: Vec<User> = Vec::new();
    users_data.push(User::new_user(
        &users_data,
        "yunus".to_string(),
        Sex::Male,
        Status::Single,
    ));
    users_data.push(User::new_user(
        &users_data,
        "Titilola".to_string(),
        Sex::Female,
        Status::Divorce,
    ));
    users_data.push(User::new_user(
        &users_data,
        "yunus".to_string(),
        Sex::Male,
        Status::Married,
    ));
    users_data.push(User::new_user(
        &users_data,
        "funke".to_string(),
        Sex::Female,
        Status::Widow,
    ));

    User::update_user_name("Iliya".to_string(), &mut users_data, 5);
    User::update_user_marital_status(Status::Married, &mut users_data, 344);
    User::update_user_sex(Sex::Female, &mut users_data, 3);
    User::update_user_name("kemi".to_string(), &mut users_data, 2);

    //User::update_user_name(&mut user_one, "Abdul".to_string());
    println!("user {:#?}", users_data);
}
