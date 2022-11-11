use std::collections::{ HashSet};
mod state;
pub use state::State;
mod symbol;
pub use symbol::Symbol;
mod transition;
pub use transition::Transition;
mod fsm;
pub use fsm::FiniteStateMachine;
mod dfa;
pub use dfa::DeterministicFiniteAutomaton;
mod nfa;
pub use nfa::NonDeterministicFiniteAutomaton;


/// insert tout les elements de apres dans avant
fn insert_all(mut old: HashSet<State>, new: HashSet<State>)-> HashSet<State>{
    for state in new{
        old.insert(state);
    }
    return old;
}
