fn main() {
    println!("{}", "Marie Curie".greet());
    println!("{}", "MARIE CURIE".greet());
    println!("{}", vec!["Jane Goodall", "Ada Lovelace"].greet());
}

pub fn is_uppercase(name: &str) -> bool {
    name.chars().all(|c| !c.is_lowercase())
}

trait Greet {
    fn greet(&self) -> String;
}

impl Greet for &str {
    fn greet(&self) -> String {
        if self.trim().is_empty() {
            return "Hello friend.".to_string();
        }

        let (hard_hearing, regular): (Vec<String>, Vec<String>) = self
            .split(',')
            .map(|name| match name.trim().is_empty() {
                true => "friend".to_string(),
                false => name.trim().to_string(),
            })
            .partition(|name| is_uppercase(name));

        match (combine_names(&regular), combine_names(&hard_hearing)) {
            (Some(greeting), Some(loud_greeting)) => {
                format!("{greeting} AND {}", loud_greeting.to_uppercase())
            }
            (_, Some(loud_greeting)) => loud_greeting.to_uppercase(),
            (Some(greeting), _) => greeting,
            _ => "Hello friend.".to_string(),
        }
    }
}

fn combine_names(names: &[String]) -> Option<String> {
    match names {
        [] => None,
        [name] => Some(format!("Hello {name}.")),
        [group @ .., last_name] => Some(format!("Hello {} and {last_name}.", group.join(", "))),
    }
}

impl Greet for Vec<&str> {
    fn greet(&self) -> String {
        self.join(",").as_str().greet()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_greet() {
        assert_eq!("Marie Curie".greet(), String::from("Hello Marie Curie."));
    }

    #[test]
    fn can_greet_stranger() {
        assert_eq!("".greet(), String::from("Hello friend."));
        assert_eq!(vec![].greet(), String::from("Hello friend."));
    }

    #[test]
    fn can_greet_loudly() {
        assert_eq!(
            "ROSALIND FRANKLIN".greet(),
            String::from("HELLO ROSALIND FRANKLIN.")
        );
    }

    #[test]
    fn can_greet_duo() {
        assert_eq!(
            vec!["Jane Goodall", "Ada Lovelace"].greet(),
            String::from("Hello Jane Goodall and Ada Lovelace.")
        );
    }

    #[test]
    fn can_greet_trio() {
        assert_eq!(
            vec!["Grace Hopper", "Emmy Noether", "Chien-Shiung Wu"].greet(),
            String::from("Hello Grace Hopper, Emmy Noether and Chien-Shiung Wu.")
        );
    }

    #[test]
    fn can_greet_hardhearing_people_separately() {
        assert_eq!(
            vec!["MARIE CURIE", "Rosalind Franklin", "JANE GOODALL", "Curie", "Ada Lovelace", "BARBARA MCCLINTOCK"].greet(),
            String::from("Hello Rosalind Franklin, Curie and Ada Lovelace. AND HELLO MARIE CURIE, JANE GOODALL AND BARBARA MCCLINTOCK.")
        );
    }

    #[test]
    fn can_separate_names_by_comma() {
        assert_eq!(
            vec!["Rachel Carson, Dorothy Crowfoot Hodgkin, Barbara McClintock", "Mae Jemison", "Rita Levi-Montalcini"].greet(),
            String::from("Hello Rachel Carson, Dorothy Crowfoot Hodgkin, Barbara McClintock, Mae Jemison and Rita Levi-Montalcini."),
        );
    }

    #[test]
    fn can_greet_mixed_group() {
        assert_eq!(
            vec!["LOUISE PEARCE", "Rosalind Franklin, Curie", "JANE GOODALL, RACHEL CARSON", "Dorothy Crowfoot Hodgkin, Chien-Shiung Wu", "", "Barbara McClintock"].greet(),
            String::from("Hello Rosalind Franklin, Curie, Dorothy Crowfoot Hodgkin, Chien-Shiung Wu, friend and Barbara McClintock. AND HELLO LOUISE PEARCE, JANE GOODALL AND RACHEL CARSON.")
        );
    }
}
