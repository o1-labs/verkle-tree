# O1 Labs Technical Challenge

**WARNING: Do not fork this repository or make a public repository containing your solution. Either copy it to a private repository or submit your solution via other means.**

Links to solutions may be sent to crypto-interview@o1labs.org

Welcome, cryptonaut! Here, at O1 Labs, we believe that even extremely talented people don't always feel comfortable with long technical interview panels. With the goals of being more inclusive and bringing more diversity to our teams, we created this repository so that anyone can express their creativity and technical capabilities without going through such exhausting hurdles.

Anyone is free to take this challenge. The file `src/lib.rs` contains the skeleton that will guide you through the tasks you need to fulfill in order to beat the challenge. The goal is to implement a Verkle tree, as described [here](https://vitalik.ca/general/2021/06/18/verkle.html).

This challenge provides you an opportunity to demonstrate the following:

- Your ability to write a data structure algorithm (in this case a Verkle tree).
- Your ability to understand some cryptographic concepts.
- Your ability to write clean, idiomatic Rust.

Please include with your submission, a section at the end of this document or within code, detailing any theoretical optimizations that could be made to the algorithm to improve its performance.

## Specification

The goal is to define the types `VerkleTree` and `VerkleProof` so that you can define the functions in the `src/lib.rs` file.

```Rust
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
```

There are many TODOs throughout the code, which you should fill in appropriately.

```Rust
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

    /// Add an element to the tree at the given position
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
```

## Building and Running

There is a test which should pass, which you can run with:

```bash
cargo test
```

## Time Considerations

We don't expect the algorithm to have been optimized for large amounts of data inputs, but include a description any optimizations you can think of, that could enable the tree to handle many thousands of entries. Take as long as you want, within reason. We expect a fully working Verkle tree that you would be happy to deploy to production.

## Advanced Mode

If you really want to go the extra mile, you are free to implement any optimizations you see fit, just don't forget to explain them in detail in your solution.
