const SPANISH: &str = "Spanish";
const FRENCH: &str = "French";
const GREETING_PREFIX_FOR_ENGLISH: &str = "Hello, ";
const GREETING_PREFIX_FOR_SPANISH: &str = "Hola, ";
const GREETING_PREFIX_FOR_FRENCH: &str = "Bonjour, ";

pub fn greet(name: &str, language: &str) -> String {
    let name = if name.is_empty() { "World" } else { name };
    let prefix = determine_greeting_prefix(language);
    format!("{}{}!", prefix, name)
}

fn determine_greeting_prefix(language: &str) -> &str {
    match language {
        SPANISH => GREETING_PREFIX_FOR_SPANISH,
        FRENCH => GREETING_PREFIX_FOR_FRENCH,
        _ => GREETING_PREFIX_FOR_ENGLISH,
    }
}

#[cfg(test)]
mod specs_for_greet {
    use super::greet;

    #[test]
    fn sut_returns_hello_in_english_if_language_is_empty() {
        // Arrange
        let name = "Chris";
        let language = "";

        // Act
        let actual = greet(name, language);

        // Assert
        let expected = "Hello, Chris!";
        assert_eq!(expected, actual);
    }

    #[test]
    fn sut_returns_world_as_default_name_if_name_is_empty() {
        // Arrange
        let name = "";
        let language = "";

        // Act
        let actual = greet(name, language);

        // Assert
        let expected = "Hello, World!";
        assert_eq!(expected, actual);
    }

    #[test]
    fn sut_returns_hola_if_language_is_spanish() {
        // Arrange
        let name = "Elodie";
        let language = "Spanish";

        // Act
        let actual = greet(name, language);

        // Assert
        let expected = "Hola, Elodie!";
        assert_eq!(expected, actual);
    }

    #[test]
    fn sut_returns_bonjour_if_language_is_french() {
        // Arrange
        let name = "Arine";
        let language = "French";

        // Act
        let actual = greet(name, language);

        // Assert
        let expected = "Bonjour, Arine!";
        assert_eq!(expected, actual);
    }
}
