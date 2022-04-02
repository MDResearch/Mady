mod chain;
mod fold_chain;

pub trait ChainIter {
    type Input;
    type Err;

    fn before(
        &mut self,
    ) -> Box<dyn Iterator<Item = Box<dyn chain::Chain<Input = Self::Input, Err = Self::Err>>>>;

    fn after(
        &mut self,
    ) -> Box<dyn Iterator<Item = Box<dyn chain::Chain<Input = Self::Input, Err = Self::Err>>>>;
}

pub use chain::Chain;
pub use fold_chain::FoldChain;