use super::{Transition, State,Symbol, FiniteStateMachine, insert_all};
use std::collections::{HashMap, HashSet};
use std::{fs};
use serde_json::{Value, from_str, map::Map};

/// Automate a état fini déterministe
#[derive(Debug, Clone)]
pub struct NonDeterministicFiniteAutomaton {
    starts: HashSet<State>,
    delta: HashMap<Transition<State>, HashSet<State>>,
    fsm: FiniteStateMachine, 
}

impl NonDeterministicFiniteAutomaton {    
    /// Créer un automate a état fini non déterministe
    /// 
    /// # Arguments
    ///
    /// * `_starts` - Les états initiaux de l'automate
    /// * `_delta` - Une HashMap decrivant les differentes transitions de l'automate
    /// * `_fsm` - Une machine à état plusieurs état fini décrivant l'automate
    ///
    /// # Examples
    /// 
    /// Le contenu du json
    /// 
    /// ```json
    /// {
    ///     "states" : ["s","t", "u", "v"],
    ///     "alphabet" :  ["b","a"],
    ///     "ends" : ["q_0"],
	///     "starts" : ["s"], 
	///     "delta" : [
	///             {
    ///              "state" : "s",         
    ///              "symbol" : "a",         
    ///              "images" : ["s", "t", "u", "v"]        
    ///             },         
    ///             {
    ///              "state" : "t",         
    ///              "symbol" : "b",         
    ///              "images" : ["s"]        
    ///             }, 
    ///             {
    ///              "state" : "u",         
    ///              "symbol" : "b",         
    ///              "images" : ["t"]        
    ///             }, 
    ///             {
    ///              "state" : "v",         
    ///              "symbol" : "b",         
    ///              "images" : ["u"]        
    ///             }
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
    ///     let link_file: &str = "src/automates/NFA1.json";
    ///     let content_json: Value = {
    ///         // Charge le contenu du fichier en tant que String
    ///         let content : String = fs::read_to_string(link_file).unwrap();
    ///         // Parse le texte en structure Json
    ///         from_str::<Value>(&content).unwrap()
    ///     };
    ///     //creation depuis un lien
    ///     let nfa : NonDeterministicFiniteAutomaton = NonDeterministicFiniteAutomaton::from_json_file(link_file);  
    ///     //creation depuis du json
    ///     let nfa2 : NonDeterministicFiniteAutomaton = NonDeterministicFiniteAutomaton::from_json(content_json.as_object().unwrap());
    ///     let fsm: FiniteStateMachine = FiniteStateMachine::from_json(content_json.as_object().unwrap());
    ///     //creation depuis new
    ///     let nfa3 : NonDeterministicFiniteAutomaton = NonDeterministicFiniteAutomaton::new(nfa.get_starts().clone(), nfa.get_delta().clone(), fsm.clone());  
    /// 
    /// }
    /// ```
    /// 
    /// # Return
    ///
    /// * `NonDeterministicFiniteAutomaton` - L'automate déterministe à état fini correspondante
    /// 
    pub fn new(_starts : HashSet<State>, _delta : HashMap<Transition<State>, HashSet<State>>, _fsm : FiniteStateMachine) -> Self {
        NonDeterministicFiniteAutomaton{
            starts : _starts,
            delta : _delta,
            fsm: _fsm
        }
    }
    /// Créer un automate à état fini non détérministe depuis un chemin du json
    ///   
    /// # Arguments
    ///
    /// * `_starts` - Les états initiaux de l'automate
    /// * `_delta` - Une HashMap decrivant les differentes transitions de l'automate
    /// * `_fsm` - Une machine à état plusieur état fini décrivant l'automate
    ///
    /// # Examples
    /// 
    /// Le contenu du json
    /// 
    /// ```json
    /// {
    ///     "states" : ["s","t", "u", "v"],
    ///     "alphabet" :  ["b","a"],
    ///     "ends" : ["q_0"],
	///     "starts" : ["s"], 
	///     "delta" : [
	///             {
    ///              "state" : "s",         
    ///              "symbol" : "a",         
    ///              "images" : ["s", "t", "u", "v"]        
    ///             },         
    ///             {
    ///              "state" : "t",         
    ///              "symbol" : "b",         
    ///              "images" : ["s"]        
    ///             }, 
    ///             {
    ///              "state" : "u",         
    ///              "symbol" : "b",         
    ///              "images" : ["t"]        
    ///             }, 
    ///             {
    ///              "state" : "v",         
    ///              "symbol" : "b",         
    ///              "images" : ["u"]        
    ///             }
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
    ///     let link_file: &str = "src/automates/NFA1.json";
    ///     let content_json: Value = {
    ///         // Charge le contenu du fichier en tant que String
    ///         let content : String = fs::read_to_string(link_file).unwrap();
    ///         // Parse le texte en structure Json
    ///         from_str::<Value>(&content).unwrap()
    ///     };
    ///     //creation depuis du json
    ///     let nfa : NonDeterministicFiniteAutomaton = NonDeterministicFiniteAutomaton::from_json(content_json.as_object().unwrap());
    /// 
    /// }
    /// ```
    /// 
    /// # Return
    ///
    /// * `NonDeterministicFiniteAutomaton` - L'automate déterministe à état fini correspondante
    /// 
    pub fn from_json(content_json: &Map<String, Value>) -> Self {
        //creation du NFA à l'aide du content_json
        let mut symbol: Symbol;
        let mut state: State;
        let mut image: State;
        let mut transition: Transition<State>;
        
        let mut alphabet: HashSet<Symbol> = HashSet::new();
        let mut states: HashSet<State> = HashSet::new();

        let mut starts : HashSet<State> = HashSet::new();
        for start in content_json.get("starts").unwrap().as_array().unwrap(){
            state = State::new(start.to_string().replace("\"", ""));
            starts.insert(state.clone());
            states.insert(state);
        }
        
        let mut delta: HashMap<Transition<State>, HashSet<State>> = HashMap::new();
        let mut transition_json: &Map<String, Value>;
        let mut images : HashSet<State> ;
        for element_delta in content_json.get("delta").unwrap().as_array().unwrap(){
            transition_json = element_delta.as_object().unwrap();
            symbol = Symbol::new(transition_json.get("symbol").unwrap().to_string().replace("\"", ""));
            state = State::new(transition_json.get("state").unwrap().to_string().replace("\"", ""));
            //generation des images du state
            images = HashSet::new();
            for img in transition_json.get("images").unwrap().as_array().unwrap(){
                image = State::new(img.to_string().replace("\"", ""));
                states.insert(image.clone());
                images.insert(image);
            }

            transition = Transition::new(symbol.clone(), state.clone()); //création de la transition: sur l'etat state, la lecture de state par symbol mene à un set d'images
            delta.insert(transition, images.clone());
            
            states.insert(state);
            alphabet.insert(symbol);
        }
        
        let mut ends: HashSet<State> = HashSet::new();
        for elem in content_json.get("ends").unwrap().as_array().unwrap(){
            state = State::new(elem.to_string().replace("\"", ""));
            ends.insert(state.clone());
            states.insert(state);
        }
        
        //on aurait pus directement utiliser l'interfasse de FiniteStateMachine pour enumerer les etat, l'alphabet etc. mais par precaution on le fait mannuellement par apport au contenu des transitions
        //let fsm = FiniteStateMachine::from_json(content_json);
        let fsm : FiniteStateMachine = FiniteStateMachine::new(states, alphabet, ends);
        NonDeterministicFiniteAutomaton { 
            starts: starts, 
            delta: delta, 
            fsm: fsm
        }
    }

    /// Créer un automate à état fini détérministe depuis un chemin vers un fichier json
    ///  
    /// # Arguments
    ///
    /// * `_starts` - Les états initiaux de l'automate
    /// * `_delta` - Une HashMap decrivant les differentes transitions de l'automate
    /// * `_fsm` - Une machine à état plusieur état fini décrivant l'automate
    ///
    /// # Examples
    /// 
    /// Le contenu du json
    /// 
    /// ```json
    /// {
    ///     "states" : ["s","t", "u", "v"],
    ///     "alphabet" :  ["b","a"],
    ///     "ends" : ["q_0"],
	///     "starts" : ["s"], 
	///     "delta" : [
	///             {
    ///              "state" : "s",         
    ///              "symbol" : "a",         
    ///              "images" : ["s", "t", "u", "v"]        
    ///             },         
    ///             {
    ///              "state" : "t",         
    ///              "symbol" : "b",         
    ///              "images" : ["s"]        
    ///             }, 
    ///             {
    ///              "state" : "u",         
    ///              "symbol" : "b",         
    ///              "images" : ["t"]        
    ///             }, 
    ///             {
    ///              "state" : "v",         
    ///              "symbol" : "b",         
    ///              "images" : ["u"]        
    ///             }
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
    ///     let link_file: &str = "src/automates/NFA1.json";
    ///     let content_json: Value = {
    ///         // Charge le contenu du fichier en tant que String
    ///         let content : String = fs::read_to_string(link_file).unwrap();
    ///         // Parse le texte en structure Json
    ///         from_str::<Value>(&content).unwrap()
    ///     };
    ///     //creation depuis un lien
    ///     let nfa : NonDeterministicFiniteAutomaton = NonDeterministicFiniteAutomaton::from_json_file(link_file);  
    /// 
    /// }
    /// ```
    /// 
    /// # Return
    ///
    /// * `NonDeterministicFiniteAutomaton` - L'automate déterministe à état fini correspondante
    /// 
    pub fn from_json_file(path: &str) -> Self {
        let content_json: Value = {
            // Charge le contenu du fichier en tant que String
            let content : String = fs::read_to_string(path).unwrap();
            // Parse le texte en structure Json
            from_str::<Value>(&content).unwrap()
        };
        //creation de la machine
        NonDeterministicFiniteAutomaton::from_json(content_json.as_object().unwrap())
    }

    /// Retourne les états de départ de l'automate
    pub fn get_starts(&self) -> &HashSet<State> {
        &self.starts
    }
    
    /// Retourne les transitions de l'automate
    pub fn get_delta(&self) -> &HashMap<Transition<State>, HashSet<State>> {
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
    /// Applique une transition et renvoie un set d'etat (representant l'image de la transition)
    pub fn apply_delta(&self, transition : Transition<State>)-> Option<HashSet<State>>{
        if let Some(n) = self.get_delta().get(&transition) {
            return Some(n.clone());
        }
        return None;
    }

    /// Applique les transition et renvoie un set d'etat (representant l'image de la transition)
    pub fn apply_deltas(&self,set_transition : Transition<HashSet<State>>) -> Option<HashSet<State>>{
        let mut images : HashSet<State> = HashSet::new();
        let mut current;
        let mut transition : Transition<State>;
        let symbol :Symbol = set_transition.get_symbol().clone();
        for state in set_transition.get_content().clone(){
            transition = Transition::new(symbol.clone(), state);
            current = self.apply_delta(transition);
            if current != None {
                images = insert_all(images, current.unwrap());
            }else{
            }
        }
        if images.is_empty() {
            return None;
        }
        return Some(images);
    }

    /// indique si un mot est accepté dans la langue de l'automate
    pub fn accept(&self, _word : &str) -> bool {
        let mut symbol : Symbol;
        let mut currents : HashSet<State> = self.get_starts().clone();//etats de depart
        let mut transition : Transition<HashSet<State>>;
        let mut temp : Option<HashSet<State>> ;
        for lettre in _word.chars() {
            symbol = Symbol::new(String::from(lettre));
            transition = Transition::new(symbol, currents.clone());
            //execution de delta pour reccuperer l'image
            temp =self.apply_deltas(transition);
            if temp==None {
                //si aucune image n'a ete trouver, ca ne sert à rien de poursuitre
                return false;
            }
            currents =temp.unwrap();
        }
        dbg!(currents.clone());
        for state in currents{
            //si on trouve un etat qui fait parti des etats finaux de l'automate, on valide le mot
            if self.get_ends().contains(&state){
                return true;
            }
        }
        //aucun des etats de currents ne fait parti des etats finaux
        return false;
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creation_partial_eq_copy() {      
        let link_file: &str = "src/automates/NFA1.json";
        let content_json: Value = {
            // Charge le contenu du fichier en tant que String
            let content : String = fs::read_to_string(link_file).unwrap();
            // Parse le texte en structure Json
            from_str::<Value>(&content).unwrap()
        };
        //creation depuis un lien
        let nfa : NonDeterministicFiniteAutomaton = NonDeterministicFiniteAutomaton::from_json_file(link_file);  
        //creation depuis du json
        let nfa2 : NonDeterministicFiniteAutomaton = NonDeterministicFiniteAutomaton::from_json(content_json.as_object().unwrap());
        let fsm: FiniteStateMachine = FiniteStateMachine::from_json(content_json.as_object().unwrap());
        //creation depuis new
        let nfa3 : NonDeterministicFiniteAutomaton = NonDeterministicFiniteAutomaton::new(nfa.get_starts().clone(), nfa.get_delta().clone(), fsm.clone());  

        assert_eq!(nfa.get_starts().clone(), nfa2.get_starts().clone());
        assert_eq!(nfa.get_ends().clone(), nfa2.get_ends().clone());
        assert_eq!(nfa.get_delta().clone(), nfa2.get_delta().clone());
        assert_eq!(nfa3.get_states().clone(), nfa.get_states().clone());
        assert_eq!(nfa3.get_ends().clone(), nfa.get_ends().clone());
        assert_eq!(nfa3.get_alphabet().clone(), nfa.get_alphabet().clone());

        assert_eq!(nfa.accept("abbbb"), false);
        assert_eq!(nfa.accept("b"), false);
        assert_eq!(nfa.accept("aabb"), true);
    }
}