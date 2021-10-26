// For this exercise, you will implement a verkle tree, as described
// here: https://vitalik.ca/general/2021/06/18/verkle.html
//
// There is a test which should pass, which you can run with
//
//     cargo test
//
// There are TODOs throughout the code, which you should fill in appropriately.
//
// The task is to define the types VerkleTree and VerkleProof so that you can define the 
// functions below.

use ark_ec::models::{short_weierstrass_jacobian::GroupAffine as SWAffine, SWModelParameters};
use ark_ec::PairingEngine;
use ark_ff::{Field, PrimeField};
use ark_poly::polynomial::univariate::DensePolynomial;
use ark_poly_commit::marlin::marlin_pc::MarlinKZG10;
use ark_poly_commit::{Polynomial, PolynomialCommitment};

impl<F: Field, P: Polynomial<F>, PC: PolynomialCommitment<F, P>> VerkleTree<F, P, PC>
where
    PC::Commitment: ToFieldElements<F>,
{
    /// Create a verkle tree with the given depth and branching factor
    pub fn new(comm_key: PC::CommitterKey, depth: usize, branching_factor: usize) -> Self {
        panic!("TODO");
    }

    /// Returns the depth of the tree
    pub fn depth(&self) -> usize {
        panic!("TODO");
    }

    /// Returns the polynomial commitment at the root of the tree
    pub fn root(&self) -> PC::Commitment {
        panic!("TODO");
    }

    /// Add an elemen to the tree at the given position
    pub fn insert(&mut self, position: usize, x: F) {
        panic!("TODO");
    }

    /// Batch-open the verkle tree at the given set of positions
    pub fn open(&self, position: Vec<usize>) -> Option<(Vec<F>, VerkleProof<F, P, PC>)> {
        panic!("TODO");
    }

    /// Check the correctness of an opening
    pub fn check(
        root: PC::Commitment,
        vk: PC::VerifierKey,
        (x, proof): (Vec<F>, VerkleProof<F, P, PC>),
    ) -> bool {
        panic!("TODO");
    }
}

pub trait ToFieldElements<F: Field> {
    // Just stipulates a method for converting a polynomial commitment into an vector of field
    // elements.
    fn to_field_elements(&self) -> Vec<F>;
}

impl<'a, 'b, P, E> ToFieldElements<P::ScalarField>
    for ark_poly_commit::marlin::marlin_pc::Commitment<E>
where
    P: SWModelParameters,
    E: PairingEngine<Fq = P::BaseField, G1Affine = SWAffine<P>>,
    P::ScalarField: PrimeField,
    P::BaseField: PrimeField<BigInt = <P::ScalarField as PrimeField>::BigInt>,
{
    fn to_field_elements(&self) -> Vec<P::ScalarField> {
        // We don't use degree bounds, and so ignore the shifted part of the commitments
        let _ = self.shifted_comm;
        [self.comm.0.x, self.comm.0.y]
            .iter()
            .map(|a| P::ScalarField::from_repr(a.into_repr()).unwrap())
            .collect()
    }
}

// TODO: Define a type VerkleTree<F, P, PC> representing a Verkle tree with leaves
// of type F that uses the polynomial commitment scheme PC.
enum VerkleTree<F: Field, P: Polynomial<F>, PC: PolynomialCommitment<F, P>> {
    TODO(F, P, PC),
}

// TODO: Define a struct VerkleProof, which will be a Verkle opening proof for multiple field
// elements
struct VerkleProof<F: Field, P: Polynomial<F>, PC: PolynomialCommitment<F, P>> {
    // Just here to get rid of the unused variable warning
    todo: (F, P, PC),
}

use ark_bn254::{Bn254, Fr};
type Bn254KZG = MarlinKZG10<Bn254, DensePolynomial<Fr>>;

pub fn test() {
    let depth = 2;
    let branching_factor = 256;

    // TODO
    let max_degree: usize = panic!("TODO");

    let mut rng = rand::thread_rng();
    let setup = Bn254KZG::setup(max_degree, None, &mut rng).unwrap();
    let committer_key: <Bn254KZG as PolynomialCommitment<_, _>>::CommitterKey = panic!("TODO");
    let verifier_key: <Bn254KZG as PolynomialCommitment<_, _>>::VerifierKey = panic!("TODO");

    let mut verkle_tree = VerkleTree::<Fr, DensePolynomial<Fr>, Bn254KZG>::new(
        committer_key,
        depth,
        branching_factor,
    );

    let num_entries = 1024;

    // Insert x^2 at position x for x in 0..1024
    for x in 0..num_entries {
        verkle_tree.insert(x, Fr::from(x as u64).square());
    }

    // Open only at even entries.
    let open_at = (0..(num_entries / 2)).map(|x| 2 * x).collect();

    let (opening_values, opening_proof) = verkle_tree.open(open_at).unwrap();

    assert_eq!(open_at.len(), opening_values.len());
    for (x, y) in open_at.iter().zip(opening_values.iter()) {
        assert_eq!(Fr::from(*x as u64).square(), *y);
    }

    let root = verkle_tree.root();

    assert!(VerkleTree::<Fr, DensePolynomial<Fr>, Bn254KZG>::check(
        root,
        verifier_key,
        (opening_values, opening_proof)
    ));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        crate::test()
    }
}
