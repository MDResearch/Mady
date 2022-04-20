mod chain;
mod fold_chain;

pub trait ChainIter {
    type Input;
    type Err;

    fn before(
        &mut self,
    ) -> Box<
        dyn Iterator<Item = &mut Box<dyn chain::Chain<Input = Self::Input, Err = Self::Err>>> + '_,
    >;

    fn after(
        &mut self,
    ) -> Box<
        dyn Iterator<Item = &mut Box<dyn chain::Chain<Input = Self::Input, Err = Self::Err>>> + '_,
    >;
}

pub use chain::Chain;
pub use fold_chain::FoldChain;
