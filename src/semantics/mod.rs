use error::SemaResult;

mod error;
mod ident;

pub use ident::*;

pub struct SemanticEngine<T: Analysis> {
    analyses: Vec<T>,
}

impl<T: Analysis> SemanticEngine<T> {
    pub fn new() -> Self {
        SemanticEngine { analyses: vec![] }
    }

    pub fn with_analysis(mut self, analysis: T) -> Self {
        self.analyses.push(analysis);
        self
    }

    pub fn run(&mut self) -> SemaResult<()> {
        let mut errors = Vec::new();
        for analyis in &mut self.analyses {
            analyis.analyze().map_err(|err| errors.extend(err));
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(())
    }
}

pub trait Analysis: Sized {
    fn analyze(&mut self) -> SemaResult<()>;
}
