use super::{State, Symbol};
use std::collections::HashSet;
use std::fs;
use serde_json::{Value, from_str, map::Map};


/// Machine à état fini 
#[derive(Debug, Clone)]
pub struct FiniteStateMachine {
    states: HashSet<State>, //set des states de la machine
    alphabet: HashSet<Symbol>,//set de symbole
    ends: HashSet<State>,//set des etats finaux de la machine
}

impl FiniteStateMachine {
    /// Creer une machine à etat fini
    /// 
    /// # Arguments
    ///
    /// * `_value` - Le symbole de Self
    ///
    /// # Examples
    /// 
    /// Le contenu du json
    /// 
    /// ```json
    /// {
    ///     "states" : ["q_0","q_1"],
    ///     "alphabet" : ["b","a"],
    ///     "ends" : ["q_0"],
    /// }
    /// 
    /// ```
    /// 
    /// Le chargement dans le code
    /// 
    /// ```
    /// use automate::*;
    /// use serde_json::{Value, from_str, map::Map};
    /// fn main() {
    ///     let link_file: &str = "src/automates/DFA1.json";
    ///     let content_json: Value = {
    ///         // Charge le contenu du fichier en tant que String
    ///         let content : String = fs::read_to_string(link_file).unwrap();
    ///         // Parse le texte en structure Json
    ///         from_str::<Value>(&content).unwrap()
    ///     };
    ///     //creation depuis un lien
    ///     let fsm : FiniteStateMachine = FiniteStateMachine::from_json_file(link_file);  
    ///     //creation depuis du json
    ///     let fsm2 : FiniteStateMachine = FiniteStateMachine::from_json(content_json.as_object().unwrap());
    ///     //creation depuis new
    ///     let fsm3 : FiniteStateMachine = FiniteStateMachine::new(fsm.get_states().clone(), fsm.get_alphabet().clone(), fsm.get_ends().clone());  
    /// }
    /// ```
    /// 
    /// # Return
    ///
    /// * `FiniteStateMachine` - La machine à état fini correspondante
    /// 
    pub fn new(_states : HashSet<State>, _alphabet: HashSet<Symbol>, _ends: HashSet<State> ) -> Self {
        FiniteStateMachine{
            states : _states,
            alphabet : _alphabet,
            ends: _ends
        }
    }

    /// Creer une machine à etat fini depuis un json
    /// 
    /// # Arguments
    ///
    /// * `content_json` - Le contenu json
    ///
    /// # Examples
    /// 
    /// Le contenu du json
    /// 
    /// ```json
    /// {
    ///     "states" : ["q_0","q_1"],
    ///     "alphabet" : ["b","a"],
    ///     "ends" : ["q_0"],
    /// }
    /// 
    /// ```
    /// 
    /// Le chargement dans le code
    /// 
    /// ```
    /// use automate::*;
    /// use serde_json::{Value, from_str, map::Map};
    /// fn main() {
    ///     let link_file: &str = "src/automates/DFA1.json";
    ///     let content_json: Value = {
    ///         // Charge le contenu du fichier en tant que String
    ///         let content : String = fs::read_to_string(link_file).unwrap();
    ///         // Parse le texte en structure Json
    ///         from_str::<Value>(&content).unwrap()
    ///     };
    ///     //creation depuis du json
    ///     let fsm : FiniteStateMachine = FiniteStateMachine::from_json(content_json.as_object().unwrap()); 
    /// }
    /// ```
    /// 
    /// # Return
    ///
    /// * `FiniteStateMachine` - La machine à état fini correspondante
    /// 
    pub fn from_json(content_json: &Map<String, Value>) -> Self {
        //creation de la machine à l'aide du content_json
        
        let mut alphabet: HashSet<Symbol> = HashSet::new();
        let mut symbol: Symbol;
        for elem in content_json.get("alphabet").unwrap().as_array().unwrap(){
            symbol = Symbol::new(elem.to_string()); //Création du symbol
            alphabet.insert(symbol);//ajout du symbol 
        }
        let mut state: State;
        let mut states: HashSet<State> = HashSet::new();
        for elem in content_json.get("states").unwrap().as_array().unwrap(){
            state = State::new(elem.to_string());
            states.insert(state);
        }
        
        let mut ends: HashSet<State> = HashSet::new();
        for elem in content_json.get("ends").unwrap().as_array().unwrap(){
            state = State::new(elem.to_string());
            ends.insert(state);
        }

        FiniteStateMachine {
            alphabet : alphabet,
            states : states,
            ends: ends,
        }
    }

    /// Créer une machine à état fini depuis un chemin vers un fichier json
    /// 
    /// # Arguments
    ///
    /// * `path` - Le schemin vers le fichier json
    ///
    /// # Examples
    /// 
    /// Le contenu du json
    /// 
    /// ```json
    /// {
    ///     "states" : ["q_0","q_1"],
    ///     "alphabet" : ["b","a"],
    ///     "ends" : ["q_0"],
    /// }
    /// 
    /// ```
    /// 
    /// Le chargement dans le code
    /// 
    /// ```
    /// use automate::*;
    /// use serde_json::{Value, from_str, map::Map};
    /// fn main() {
    ///     let link_file: &str = "src/automates/DFA1.json";
    ///     //creation depuis un lien
    ///     let fsm : FiniteStateMachine = FiniteStateMachine::from_json_file(link_file);   
    /// }
    /// ```
    /// 
    /// # Return
    ///
    /// * `FiniteStateMachine` - La machine à état fini correspondante
    /// 
    pub fn from_json_file(path: &str) -> Self {
        let content_json: Value = {
            // Charge le contenu du fichier en tant que String
            let content : String = fs::read_to_string(path).unwrap();
            // Parse le texte en structure Json
            from_str::<Value>(&content).unwrap()
        };
        //creation de la machine
        FiniteStateMachine::from_json(content_json.as_object().unwrap())
    }

    /// Retourne les états de la machine
    pub fn get_states(&self) -> &HashSet<State> {
        &self.states
    }

    /// Retourne l'alphabet de la machine
    pub fn get_alphabet(&self) -> &HashSet<Symbol> {
        &self.alphabet
    }

    /// Retourne les états finaux de la machine
    pub fn get_ends(&self) -> &HashSet<State> {
        &self.ends
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creation_fsm() {
        let link_file: &str = "src/automates/DFA1.json";
        let content_json: Value = {
            // Charge le contenu du fichier en tant que String
            let content : String = fs::read_to_string(link_file).unwrap();
            // Parse le texte en structure Json
            from_str::<Value>(&content).unwrap()
        };
        //creation depuis un lien
        let fsm : FiniteStateMachine = FiniteStateMachine::from_json_file(link_file);  
        //creation depuis du json
        let fsm2 : FiniteStateMachine = FiniteStateMachine::from_json(content_json.as_object().unwrap());
        //creation depuis new
        let fsm3 : FiniteStateMachine = FiniteStateMachine::new(fsm.get_states().clone(), fsm.get_alphabet().clone(), fsm.get_ends().clone());  

        assert_eq!(fsm.get_states().clone(), fsm2.get_states().clone());
        assert_eq!(fsm.get_ends().clone(), fsm2.get_ends().clone());
        assert_eq!(fsm.get_alphabet().clone(), fsm2.get_alphabet().clone());
        assert_eq!(fsm.get_states().clone(), fsm3.get_states().clone());
        assert_eq!(fsm.get_ends().clone(), fsm3.get_ends().clone());
        assert_eq!(fsm.get_alphabet().clone(), fsm3.get_alphabet().clone());
        
    }
}