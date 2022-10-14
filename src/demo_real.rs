use num_bigint::BigUint;
use std::{borrow::Borrow, collections::HashMap, marker::PhantomData, rc::Rc};

use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{AssignedCell, Cell, Layouter, Region, SimpleFloorPlanner, Value},
    dev::MockProver,
    halo2curves::bn256::Fr,
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Fixed},
};

use halo2_base::{
    gates::{flex_gate::FlexGateConfig, GateInstructions,
	    range::{RangeConfig, RangeStrategy::Vertical},
    },
    utils::{bigint_to_fe, biguint_to_fe, decompose_bigint, fe_to_bigint, fe_to_biguint,
	    value_to_option},
    AssignedValue, Context, ContextParams,
    QuantumCell::{Constant, Existing, Witness},
};


const NUM_ADVICE: usize = 1;
const NUM_ADVICE_LOOKUP: usize = 0;
const NUM_FIXED: usize = 1;

#[derive(Default)]
pub struct MyCircuit<F> {
    pub A: Vec<Vec<F>>,
    pub b: Vec<Option<F>>,
    pub _marker: PhantomData<F>,
}

impl<F: FieldExt> Circuit<F> for MyCircuit<F> {
    type Config = RangeConfig<F>;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
	Self { A: vec![], b: vec![], _marker: PhantomData }
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
	RangeConfig::configure(
	    meta,
	    Vertical,
	    &[NUM_ADVICE],
	    &[NUM_ADVICE_LOOKUP],
	    NUM_FIXED,
	    2,
	    "default".to_string()
	)
    }

    fn synthesize(
	&self,
	config: Self::Config,
	mut layouter: impl Layouter<F>
    ) -> Result<(), Error> {
	config.load_lookup_table(&mut layouter)?;

	let mut first_pass = true;
	layouter.assign_region(
	    || "demo",
	    |region| {
		if first_pass {
		    first_pass = false;
		    return Ok(());
		}

		let mut aux = Context::new(
		    region,
		    ContextParams { num_advice: vec![("default".to_string(), NUM_ADVICE)] },
		);
		let ctx = &mut aux;
		// Start typing in demo

//		println!("intermed {:?}", res_vec.iter().map(|x| fe_to_biguint(value_to_option(x.value()).unwrap())).collect::<Vec<BigUint>>());


//		println!("final {:?}", final_vec.iter().map(|x| fe_to_biguint(value_to_option(x.value()).unwrap())).collect::<Vec<BigUint>>());

		// End typing in demo
		println!("Using {} advice columns and {} fixed columns", NUM_ADVICE, NUM_FIXED);
                println!("total cells: {}", ctx.advice_rows["default"].iter().sum::<usize>());
                println!(
                    "maximum rows used by an advice column: {}",
                    ctx.advice_rows["default"].iter().max().unwrap()
                );
                let (const_rows, _, _) = config.finalize(ctx)?;
                println!("maximum rows used by a fixed column: {}", const_rows);
                Ok(())
	    }
	)?;
	Ok(())
    }
}


#[cfg(test)]
#[test]
fn test_mock_real() {
    let k = 10;
    let A = vec![vec![Fr::from(1), Fr::from(1), Fr::from(1), Fr::from(1)],
		 vec![Fr::from(0), Fr::from(1), Fr::from(1), Fr::from(1)],
		 vec![Fr::from(0), Fr::from(0), Fr::from(1), Fr::from(1)],
		 vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(1)]];

    let b_pre = vec![Fr::from(1), Fr::from(2), Fr::from(3), Fr::from(4)];
    let b = b_pre.iter().map(|x| Some(*x)).collect();
    
    let circuit = MyCircuit::<Fr> { A, b, _marker: PhantomData };
    let prover = MockProver::run(k, &circuit, vec![]).unwrap();
    prover.assert_satisfied();
    assert_eq!(prover.verify(), Ok(()));
}
