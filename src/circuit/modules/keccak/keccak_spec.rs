use std::marker::PhantomData;
use halo2_proofs::halo2curves::bn256::Fr as Fp;
use halo2_proofs::{circuit::*, plonk::*};
use zkevm_circuits::keccak_circuit::KeccakConfig as InnerConfig;

use super::Module;

pub struct KeccakConfig<F> {
    ///
    pub hash_inputs: Vec<Column<Advice>>,
    ///
    pub instance: Option<Column<Instance>>,
    ///
    pub pow5_config: Pow5Config<Fp, WIDTH, RATE>,
}

#[derive(Debug, Clone)]
pub struct KeccakChip<
    S: Spec<Fp, WIDTH, RATE> + Sync,
    const WIDTH: usize,
    const RATE: usize,
    const L: usize,
> {
    config: KeccakConfig<WIDTH, RATE>,
    _marker: PhantomData<S>,
}

type InputAssignments = ();

impl<S: Spec<Fp, WIDTH, RATE> + Sync, const WIDTH: usize, const RATE: usize, const L: usize>
Module<Fp> for KeccakChip<S, WIDTH, RATE, L>
{
    type Config = KeccakConfig<WIDTH, RATE>;

    type InputAssignments = InputAssignments;

    type RunInputs = Vec<Fp>;
    type Params = ();

    fn new(config: Self::Config) -> Self {
        Self {
            config,
            _marker: PhantomData
        }
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        todo!()
    }

    fn name(&self) -> &'static str {
        "Keccak"
    }

    fn run(input: Self::RunInputs) -> Result<Vec<Vec<Fp>>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn layout_inputs(
        &self,
        layouter: &mut impl Layouter<Fp>,
        input: &[crate::tensor::ValTensor<Fp>],
    ) -> Result<Self::InputAssignments, Error> {
        !todo!()
    }

    fn layout(
        &self,
        layouter: &mut impl Layouter<Fp>,
        input: &[crate::tensor::ValTensor<Fp>],
        row_offsets: Vec<usize>,
    ) -> Result<crate::tensor::ValTensor<Fp>, Error> {
        todo!()
    }

    fn instance_increment_input(&self) -> Vec<usize> {
        todo!()
    }

    fn num_rows(input_len: usize) -> usize {
        todo!()
    }
}