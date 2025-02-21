//! Input for the [`Nautilus`](https://github.com/RUB-SysSec/nautilus) grammar fuzzer methods
use alloc::{rc::Rc, vec::Vec};
use core::cell::RefCell;
use std::hash::{Hash, Hasher};

use libafl_bolts::{ownedref::OwnedSlice, HasLen};
use serde::{Deserialize, Serialize};

use super::TargetBytesConverter;
use crate::{
    common::nautilus::grammartec::{
        newtypes::NodeId,
        rule::RuleIdOrCustom,
        tree::{Tree, TreeLike},
    },
    generators::nautilus::NautilusContext,
    inputs::{BytesInput, Input, InputConverter},
    Error,
};

/// An [`Input`] implementation for `Nautilus` grammar.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NautilusInput {
    /// The input representation as Tree
    pub tree: Tree,
}

impl Input for NautilusInput {}

/// Rc Ref-cell from Input
impl From<NautilusInput> for Rc<RefCell<NautilusInput>> {
    fn from(input: NautilusInput) -> Self {
        Rc::new(RefCell::new(input))
    }
}

impl HasLen for NautilusInput {
    #[inline]
    fn len(&self) -> usize {
        self.tree.size()
    }
}

impl NautilusInput {
    /// Creates a new codes input using the given terminals
    #[must_use]
    pub fn new(tree: Tree) -> Self {
        Self { tree }
    }

    /// Create an empty [`Input`]
    #[must_use]
    pub fn empty() -> Self {
        Self {
            tree: Tree {
                rules: vec![],
                sizes: vec![],
                paren: vec![],
            },
        }
    }

    /// Generate a `Nautilus` input from the given bytes
    pub fn unparse(&self, context: &NautilusContext, bytes: &mut Vec<u8>) {
        bytes.clear();
        self.tree.unparse(NodeId::from(0), &context.ctx, bytes);
    }

    /// Get the tree representation of this input
    #[must_use]
    pub fn tree(&self) -> &Tree {
        &self.tree
    }

    /// Get the tree representation of this input, as a mutable reference
    #[must_use]
    pub fn tree_mut(&mut self) -> &mut Tree {
        &mut self.tree
    }
}

impl Hash for NautilusInput {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tree().paren.hash(state);
        for r in &self.tree().rules {
            match r {
                RuleIdOrCustom::Custom(a, b) => {
                    a.hash(state);
                    b.hash(state);
                }
                RuleIdOrCustom::Rule(a) => a.hash(state),
            }
        }
        self.tree().sizes.hash(state);
    }
}

/// `InputConverter` to convert from `NautilusInput` to `BytesInput`
#[derive(Debug)]
pub struct NautilusToBytesInputConverter<'a> {
    ctx: &'a NautilusContext,
}

impl<'a> NautilusToBytesInputConverter<'a> {
    #[must_use]
    /// Create a new `NautilusToBytesInputConverter` from a context
    pub fn new(ctx: &'a NautilusContext) -> Self {
        Self { ctx }
    }
}

impl InputConverter for NautilusToBytesInputConverter<'_> {
    type From = NautilusInput;
    type To = BytesInput;

    fn convert(&mut self, input: Self::From) -> Result<Self::To, Error> {
        let mut bytes = vec![];
        input.unparse(self.ctx, &mut bytes);
        Ok(BytesInput::new(bytes))
    }
}

/// A converter to convert a nautilus context to target bytes
#[derive(Debug)]
pub struct NautilusTargetBytesConverter<'a> {
    /// The Nautilus Context
    ctx: &'a NautilusContext,
}

impl<'a> NautilusTargetBytesConverter<'a> {
    /// Create a new [`NautilusTargetBytesConverter`]
    #[must_use]
    pub fn new(ctx: &'a NautilusContext) -> NautilusTargetBytesConverter<'a> {
        NautilusTargetBytesConverter { ctx }
    }
}

impl TargetBytesConverter<NautilusInput> for NautilusTargetBytesConverter<'_> {
    fn to_target_bytes<'a>(&mut self, input: &'a NautilusInput) -> OwnedSlice<'a, u8> {
        let mut bytes = Vec::new();
        input.unparse(self.ctx, &mut bytes);
        OwnedSlice::from(bytes)
    }
}
