use std::{marker::PhantomData, sync::Arc};

use error::SemaResult;

mod error;
mod function;
mod ident;
mod origin;

pub use function::*;
pub use ident::*;
pub use origin::*;

// pub struct SemanticEngine {
//     analyses: Vec<Arc<dyn Analysis>>,
// }

// impl SemanticEngine {
//     pub fn new() -> Self {
//         SemanticEngine { analyses: vec![] }
//     }

//     pub fn with_analysis(mut self, analysis: Arc<dyn Analysis>) -> Self {
//         self.analyses.push(analysis);
//         self
//     }

//     pub fn run(&mut self) -> SemaResult<()> {
//         let mut errors = Vec::new();
//         for analyis in &mut self.analyses {
//             analyis.analyze().map_err(|err| errors.extend(err));
//         }

//         if errors.len() > 0 {
//             return Err(errors);
//         }

//         Ok(())
//     }
// }

pub trait Analysis {
    fn analyze(&mut self) -> SemaResult<()>;
}
