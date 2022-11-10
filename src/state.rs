//string n'implemente pas copy, donc on peut juste utiliser clone
#[derive(Debug, Clone)]
pub struct State {
    name: String,
}

impl State {
    
    /// Créer un State
    ///
    /// # Arguments
    ///
    /// * `_name` - Le nom du state
    ///
    /// # Examples
    ///
    /// ```
    /// use automate::State;
    /// fn main() {
    ///     let string_one : String = String::from("state one");
    ///     let state_one : State = State::new(string_one);
    /// }
    /// ```
    ///
    /// # Return
    ///
    /// * `State` - Le State qui a été créer
    ///
    pub fn new(_name : String) -> Self {
        State { name: _name }
    }


    /// Retourne le nom du State
    ///
    /// # Example
    ///
    /// ```
    /// use automate::State;
    /// fn main() {
    ///     let string_one : String = String::from("state one");
    ///     let state_one : State = State::new(string_one);
    ///     println!("{}", state_one.get_name());
    /// }
    /// ```
    ///
    /// # Return
    ///
    /// * `&String` - Le nom du State
    ///
    pub fn get_name(&self) -> &String {
        &self.name
    }
}

impl PartialEq<State> for State {
    fn eq(&self, other: &State) -> bool {
        self.get_name() == other.get_name()
    }

    fn ne(&self, other: &State) -> bool {
        self.get_name() != other.get_name()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creation_partial_eq_copy() {      
        let string_one : String = String::from("state");
        let string_two : String = String::from("state two");

        let state_one : State = State::new(string_one.clone());
        let state_one_bis : State = State::new(string_one.clone());
        let state_two : State = State::new(string_two);
        
        assert_eq!(state_one, state_one_bis);
        assert_eq!(state_one==state_two, false);
        assert_eq!(state_one.get_name(), &(string_one));

    }
}