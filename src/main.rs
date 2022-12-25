use dialoguer::{
    Select,
    MultiSelect,
    Input,
    theme::ColorfulTheme
};
use console::Term;
use std::fmt;
mod enums;
use enums::KeyType;
mod countries;
use countries::{COUNTRIES, countries_labels, country_codes_by_indexes};
mod crypto;
use crypto::encrypt;
mod models;
use models::KeyDataModel;
use std::io::{stdin, stdout, Read, Write};

fn main() {
    println!("Hash Generator");
    let key_type = get_key_type().unwrap();
    let env_name = get_env_name().unwrap();

    match key_type {
        KeyType::Unlimited => {},
        KeyType::Standard => {
            let start_date = get_start_date().unwrap();
            let end_date = get_end_date().unwrap();
            let countries = get_countries().unwrap();
        }
    }

    let data = KeyDataModel {
        keyType: key_type,
        envName: env_name
    } ;
    println!("\nHash Key:-");
    println!("{}", encrypt(data));

    stdin().read(&mut [0]).unwrap();
}


fn get_key_type() -> std::io::Result<KeyType> {
    let mut result: KeyType = KeyType::Unlimited;

    let items = vec![
        "1. Unlimited (give unlimited usage to the system, where there are no limitation on dates or new countries being added to the system)",
        "2. Standard (it limits the time of usage with a start and end date and adding of new countries)"
    ];

    let selected_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the type of Hash key")
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selected_index {
        Some(index) => {
            if index == 0 {
                result = KeyType::Unlimited;
            } else if index == 1 {
                result = KeyType::Standard;
            }
        },
        None => println!("You should select the Hash key type first!")
    }

    println!("\n'{}' key type is selected\n", result.to_string());

    Ok(result)
}

fn get_env_name() -> std::io::Result<String> {

    let input: String = Input::new()
        .with_prompt("Type environment name")
        .interact_text()?;

    Ok(input)
}

fn get_start_date() -> std::io::Result<String> {

    let input: String = Input::new()
        .with_prompt("Type valid Start Date ex. [mm/dd/yyyy] [dd mm yyyy]")
        .interact_text()?;

    Ok(input)
}

fn get_end_date() -> std::io::Result<String> {

    let input: String = Input::new()
        .with_prompt("Type valid End Date ex. [mm/dd/yyyy] [dd mm yyyy]")
        .interact_text()?;

    Ok(input)
}

fn get_countries() -> std::io::Result<Vec<String>> {

    let indexes = MultiSelect::new()
        .with_prompt("Please choose your Countries")
        .items(&countries_labels())
        .interact_on(&Term::stderr())?;

    let selected_codes = country_codes_by_indexes(indexes);
    Ok(selected_codes)
}