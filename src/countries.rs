pub struct Country {
    name: &'static str,
    code: &'static str,
}

pub const COUNTRIES: [Country; 16] = [
    Country {
        name: "Anguilla",
        code: "AI",
    },
    Country {
        name: "Antigua and Barbuda",
        code: "AG",
    },
    Country {
        name: "Aruba",
        code: "AW",
    },
    Country {
        name: "Bahamas",
        code: "BS",
    },
    Country {
        name: "Barbados",
        code: "BB",
    },
    Country {
        name: "Belize",
        code: "BZ",
    },
    Country {
        name: "Cayman Islands",
        code: "KY",
    },
    Country {
        name: "Curaçao",
        code: "CW",
    },
    Country {
        name: "Dominica",
        code: "DM",
    },
    Country {
        name: "Grenada",
        code: "GD",
    },
    Country {
        name: "Jamaica",
        code: "JM",
    },
    Country {
        name: "Saint Kitts and Nevis",
        code: "KN",
    },
    Country {
        name: "Saint Lucia",
        code: "LC",
    },
    Country {
        name: "Trinidad and Tobago",
        code: "TT",
    },
    Country {
        name: "Saint Vincent and the Grenadines",
        code: "VC",
    },
    Country {
        name: "Virgin Islands (British)",
        code: "VG",
    }
];

pub fn countries_labels() -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for country in COUNTRIES {
        result.push(format!("[{}] {}", country.code, country.name));
    }
    return result;
}

pub fn country_codes_by_indexes(indexes: Vec<usize>) -> Vec<String> {
    let mut result:Vec<String> = Vec::new();
    for index in indexes {
        let code= COUNTRIES.get(index).unwrap().code.to_string();
        result.push(code);
    }
    return result;
}