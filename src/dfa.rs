use crate::Symbol;

use super::{Transition, State, FiniteStateMachine};
use std::collections::{HashMap, HashSet};

/// Automate fini déterministe
#[derive(Debug, Clone)]
pub struct DeterministicFiniteAutomaton {
    start: State,
    delta: HashMap<Transition<State>, State>,
    fsm: FiniteStateMachine, 
}


impl DeterministicFiniteAutomaton {
    
    /// Créer un automate fini déterministe
    ///
    /// # Arguments
    ///
    /// * `_start` - L'état de départ
    /// * `_delta` - Une HashMap décrivant les transitions possible entre les differents états
    /// * `_fsm` - Une machine a état fini
    /// 
    /// # Example
    ///
    /// ```
    /// use automate::DeterministicFiniteAutomaton;
    /// fn main() {
    /// 
    /// }
    /// ```
    ///
    /// # Return
    ///
    /// * `DeterministicFiniteAutomaton` - Le State qui a été créer
    ///
    pub fn new(_start : State, _delta : HashMap<Transition<State>, State>, _fsm : FiniteStateMachine) -> Self {
        DeterministicFiniteAutomaton{
            start : _start,
            delta : _delta,
            fsm: _fsm
        }
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
        
    }
}