use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SymbolTable<T: Clone> {
    previous: Option<Box<SymbolTable<T>>>,
    symbols: HashMap<String, T>,
}

impl<T: Clone> SymbolTable<T> {
    pub fn new() -> Self {
        SymbolTable {
            previous: None,
            symbols: HashMap::new(),
        }
    }

    pub fn with_previous(mut self, previous: SymbolTable<T>) -> Self {
        self.previous = Some(Box::new(previous));
        self
    }

    pub fn find(&self, symbol: &str) -> Option<T> {
        self.symbols
            .get(symbol)
            .cloned()
            .or_else(|| self.previous.as_ref().and_then(|prev| prev.find(symbol)))
    }

    pub fn insert(&mut self, symbol: String, value: T) {
        self.symbols.insert(symbol, value);
    }

    pub fn clear(&mut self) {
        self.symbols.clear();
    }
}
