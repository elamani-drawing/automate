use super::{Transition, State,Symbol, FiniteStateMachine};
use std::collections::{HashMap, HashSet};
use std::fs;
use serde_json::{Value, from_str, map::Map};

/// Automate a état fini déterministe
#[derive(Debug, Clone)]
pub struct DeterministicFiniteAutomaton {
    start: State,
    delta: HashMap<Transition<State>, State>,
    fsm: FiniteStateMachine, 
}


impl DeterministicFiniteAutomaton {    
    /// Créer un automate a état fini déterministe
    /// 
    /// # Arguments
    ///
    /// * `_start` - L'état initial de l'automate
    /// * `_delta` - Une HashMap decrivant les differentes transition de l'automate
    /// * `_fsm` - Une machine à état fini dérivant l'automate
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
	///     "start" : "q_0", 
	///     "delta" : [
	///     	{
	///     		"state" : "q_0",         
	///     		"symbol" : "a",         
	///     		"image" : "q_1"         
	///     	},         
	///     	{         
	///     		"state" : "q_1",         
	///     		"symbol" : "b",         
	///     		"image" : "q_0"        
	///     	}
	///     ] 
    /// }
    /// 
    /// ```
    /// 
    /// Le chargement dans le code
    /// 
    /// ```
    /// use automate::*;
    /// use std::fs;
    /// use serde_json::{Value, from_str, map::Map};
    /// fn main() {
    /// 
    ///     let link_file: &str = "src/automates/DFA1.json";
    ///     let content_json: Value = {
    ///         // Charge le contenu du fichier en tant que String
    ///         let content : String = fs::read_to_string(link_file).unwrap();
    ///         // Parse le texte en structure Json
    ///         from_str::<Value>(&content).unwrap()
    ///     };
    ///     //creation depuis un lien
    ///     let dfa : DeterministicFiniteAutomaton = DeterministicFiniteAutomaton::from_json_file(link_file);  
    ///     //creation depuis du json
    ///     let dfa2 : DeterministicFiniteAutomaton = DeterministicFiniteAutomaton::from_json(content_json.as_object().unwrap());
    ///     let fsm: FiniteStateMachine = FiniteStateMachine::from_json(content_json.as_object().unwrap());
    ///     //creation depuis new
    ///     let dfa3 : DeterministicFiniteAutomaton = DeterministicFiniteAutomaton::new(dfa.get_start().clone(), dfa.get_delta().clone(), fsm.clone());  
    /// 
    /// }
    /// ```
    /// 
    /// # Return
    ///
    /// * `DeterministicFiniteAutomaton` - L'automate déterministe à état fini correspondante
    /// 
    pub fn new(_start : State, _delta : HashMap<Transition<State>, State>, _fsm : FiniteStateMachine) -> Self {
        DeterministicFiniteAutomaton{
            start : _start,
            delta : _delta,
            fsm: _fsm
        }
    }
    /// Créer un automate à état fini détérministe depuis un chemin du json
    /// 
    /// # Arguments
    ///
    /// * `_start` - L'état initial de l'automate
    /// * `_delta` - Une HashMap decrivant les differentes transition de l'automate
    /// * `_fsm` - Une machine à état fini dérivant l'automate
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
	///     "start" : "q_0", 
	///     "delta" : [
	///     	{
	///     		"state" : "q_0",         
	///     		"symbol" : "a",         
	///     		"image" : "q_1"         
	///     	},         
	///     	{         
	///     		"state" : "q_1",         
	///     		"symbol" : "b",         
	///     		"image" : "q_0"        
	///     	}
	///     ] 
    /// }
    /// 
    /// ```
    /// 
    /// Le chargement dans le code
    /// 
    /// ```
    /// use automate::*;
    /// use std::fs;
    /// use serde_json::{Value, from_str, map::Map};
    /// fn main() {
    /// 
    ///     let link_file: &str = "src/automates/DFA1.json";
    ///     let content_json: Value = {
    ///         // Charge le contenu du fichier en tant que String
    ///         let content : String = fs::read_to_string(link_file).unwrap();
    ///         // Parse le texte en structure Json
    ///         from_str::<Value>(&content).unwrap()
    ///     };
    ///     //creation depuis du json
    ///     let dfa : DeterministicFiniteAutomaton = DeterministicFiniteAutomaton::from_json(content_json.as_object().unwrap());
    /// 
    /// }
    /// ```
    /// 
    /// # Return
    ///
    /// * `DeterministicFiniteAutomaton` - L'automate déterministe à état fini correspondante
    /// 
    pub fn from_json(content_json: &Map<String, Value>) -> Self {
        //creation de la machine à l'aide du content_json
        let state_init :State = State::new(content_json.get("start").unwrap().to_string());
        let mut symbol: Symbol;
        let mut state: State;
        let mut image: State;
        let mut transition: Transition<State>;
        
        let mut alphabet: HashSet<Symbol> = HashSet::new();
        let mut states: HashSet<State> = HashSet::new();
        
        let mut delta: HashMap<Transition<State>, State> = HashMap::new();
        let mut transition_json: &Map<String, Value>;
        for element_delta in content_json.get("delta").unwrap().as_array().unwrap(){
            transition_json = element_delta.as_object().unwrap();
            symbol = Symbol::new(transition_json.get("symbol").unwrap().to_string());
            state = State::new(transition_json.get("state").unwrap().to_string());
            image = State::new(transition_json.get("image").unwrap().to_string());
            transition = Transition::new(symbol.clone(), state.clone()); //création de la transition: sur l'etat state, la lecture de state par symbol mene à image
            delta.insert(transition, image.clone());
            //a chaque ajoute le symbole dans l'alphabet et les etatsalphabet.insert(symbol);
            states.insert(state);
            states.insert(image);
            alphabet.insert(symbol);
        }
        states.insert(state_init.clone());
        
        let mut ends: HashSet<State> = HashSet::new();
        for elem in content_json.get("ends").unwrap().as_array().unwrap(){
            state = State::new(elem.to_string());
            ends.insert(state);
        }
        
        //on aurait pus directement utiliser l'interfasse de FiniteStateMachine pour enumerer les etat, l'alphabet etc. mais par precaution on le fait mannuellement par apport au contenu des transitions
        //let fsm = FiniteStateMachine::from_json(content_json);
        let fsm : FiniteStateMachine = FiniteStateMachine::new(states, alphabet, ends);
        DeterministicFiniteAutomaton { 
            start: state_init, 
            delta: delta, 
            fsm: fsm
        }
    }

    /// Créer un automate à état fini détérministe depuis un chemin vers un fichier json
    /// 
    /// # Arguments
    ///
    /// * `_start` - L'état initial de l'automate
    /// * `_delta` - Une HashMap decrivant les differentes transition de l'automate
    /// * `_fsm` - Une machine à état fini dérivant l'automate
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
	///     "start" : "q_0", 
	///     "delta" : [
	///     	{
	///     		"state" : "q_0",         
	///     		"symbol" : "a",         
	///     		"image" : "q_1"         
	///     	},         
	///     	{         
	///     		"state" : "q_1",         
	///     		"symbol" : "b",         
	///     		"image" : "q_0"        
	///     	}
	///     ] 
    /// }
    /// 
    /// ```
    /// 
    /// Le chargement dans le code
    /// 
    /// ```
    /// use automate::*;
    /// use std::fs;
    /// use serde_json::{Value, from_str, map::Map};
    /// fn main() {
    /// 
    ///     let link_file: &str = "src/automates/DFA1.json";
    ///     let content_json: Value = {
    ///         // Charge le contenu du fichier en tant que String
    ///         let content : String = fs::read_to_string(link_file).unwrap();
    ///         // Parse le texte en structure Json
    ///         from_str::<Value>(&content).unwrap()
    ///     };
    ///     //creation depuis un lien
    ///     let dfa : DeterministicFiniteAutomaton = DeterministicFiniteAutomaton::from_json_file(link_file);  
    /// 
    /// }
    /// ```
    /// 
    /// # Return
    ///
    /// * `DeterministicFiniteAutomaton` - L'automate déterministe à état fini correspondante
    /// 
    pub fn from_json_file(path: &str) -> Self {
        let content_json: Value = {
            // Charge le contenu du fichier en tant que String
            let content : String = fs::read_to_string(path).unwrap();
            // Parse le texte en structure Json
            from_str::<Value>(&content).unwrap()
        };
        //creation de la machine
        DeterministicFiniteAutomaton::from_json(content_json.as_object().unwrap())
    }

    /// Retourne l'état de départ de l'automate
    pub fn get_start(&self) -> &State {
        &self.start
    }
    
    /// Retourne les transitions de l'automate
    pub fn get_delta(&self) -> &HashMap<Transition<State>, State> {
        &self.delta
    }

    /// Retournes les differents états de l'automate
    pub fn get_states(&self) -> &HashSet<State> {
        &self.fsm.get_states()
    }

    /// Retourne l'alphabet de l'automate
    pub fn get_alphabet(&self) -> &HashSet<Symbol> {
        &self.fsm.get_alphabet()
    }

    /// Retourne les états finaux de l'automate
    pub fn get_ends(&self) -> &HashSet<State> {
        &self.fsm.get_ends()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creation_partial_eq_copy() {      
        let link_file: &str = "src/automates/DFA1.json";
        let content_json: Value = {
            // Charge le contenu du fichier en tant que String
            let content : String = fs::read_to_string(link_file).unwrap();
            // Parse le texte en structure Json
            from_str::<Value>(&content).unwrap()
        };
        //creation depuis un lien
        let dfa : DeterministicFiniteAutomaton = DeterministicFiniteAutomaton::from_json_file(link_file);  
        //creation depuis du json
        let dfa2 : DeterministicFiniteAutomaton = DeterministicFiniteAutomaton::from_json(content_json.as_object().unwrap());
        let fsm: FiniteStateMachine = FiniteStateMachine::from_json(content_json.as_object().unwrap());
        //creation depuis new
        let dfa3 : DeterministicFiniteAutomaton = DeterministicFiniteAutomaton::new(dfa.get_start().clone(), dfa.get_delta().clone(), fsm.clone());  

        assert_eq!(dfa.get_start().clone(), dfa2.get_start().clone());
        assert_eq!(dfa.get_ends().clone(), dfa2.get_ends().clone());
        assert_eq!(dfa.get_delta().clone(), dfa2.get_delta().clone());
        assert_eq!(dfa3.get_states().clone(), dfa.get_states().clone());
        assert_eq!(dfa3.get_ends().clone(), dfa.get_ends().clone());
        assert_eq!(dfa3.get_alphabet().clone(), dfa.get_alphabet().clone());
        //dbg!(dfa);
    }
}