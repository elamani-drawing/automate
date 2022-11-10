//string n'implemente pas copy, donc on peut juste utiliser clone
#[derive(Debug, Clone)]
pub struct Symbol {
    symbol: String,
}


impl Symbol {
    
    /// Créer un Symbol
    ///
    /// # Arguments
    ///
    /// * `_symbol` - Le symbole de Self
    ///
    /// # Example
    ///
    /// ```
    /// use automate::Symbol;
    /// fn main() {
    ///     let string_one : String = String::from("symbol");
    ///     let Symbol_one : Symbol = Symbol::new(string_one);
    /// }
    /// ```
    ///
    /// # Return
    ///
    /// * `Symbol` - Le Symbol qui a été créer
    ///
    pub fn new(_symbol : String) -> Self {
        Symbol { symbol: _symbol }
    }


    /// Retourne la valeur du Symbol
    ///
    /// # Example
    ///
    /// ```
    /// use automate::Symbol;
    /// fn main() {
    ///     let string_one : String = String::from("Symbol");
    ///     let symbol_one : Symbol = Symbol::new(string_one.clone());
    ///     println!("{}", symbol_one.get_symbol());
    /// }
    /// ```
    ///
    /// # Return
    ///
    /// * `&String` - Le symbol de selfs
    ///
    pub fn get_symbol(&self) -> &String {
        &self.symbol
    }
}

impl PartialEq<Symbol> for Symbol {
    fn eq(&self, other: &Symbol) -> bool {
        self.get_symbol() == other.get_symbol()
    }

    fn ne(&self, other: &Symbol) -> bool {
        self.get_symbol() != other.get_symbol()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creation_partial_eq_copy() {      
        let string_one : String = String::from("Symbol");
        let string_two : String = String::from("Symbol two");

        let symbol_one : Symbol = Symbol::new(string_one.clone());
        let symbol_one_bis : Symbol = Symbol::new(string_one.clone());
        let symbol_two : Symbol = Symbol::new(string_two);
        
        assert_eq!(symbol_one, symbol_one_bis);
        assert_eq!(symbol_one==symbol_two, false);
        assert_eq!(symbol_one.get_symbol(), &(string_one));
    }
}