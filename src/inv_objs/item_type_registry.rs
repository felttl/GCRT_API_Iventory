use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use crate::inv_objs::item_type_id::ItemTypeId;

#[derive(Debug, PartialEq, Eq)]
pub enum RegistryError {
    /// a type with this name already exists
    DuplicateName(String),
    /// a parent id does not come from this builder
    UnknownParent(ItemTypeId),
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegistryError::DuplicateName(n) => write!(f, "type \"{n}\" already registered"),
            RegistryError::UnknownParent(id) => write!(f, "unknown parent type {id:?}"),
        }
    }
}

impl std::error::Error for RegistryError {}

/// Phase 1: declare the types (mutable). Phase 2: `build()` freezes them.
#[derive(Default)]
pub struct ItemTypeRegistryBuilder {
    names: Vec<String>,
    /// lineage[i] = type i itself + all its ancestors (computed here, never by the user)
    lineage: Vec<Vec<ItemTypeId>>,
    by_name: HashMap<String, ItemTypeId>,
}

impl ItemTypeRegistryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// ## Add a type with no parents
    /// - Parents must already be registered, so cycles are impossible.
    pub fn add(&mut self, name: &str) -> Result<ItemTypeId, RegistryError> {
        self.addwp(name, &[])
    }

    /// ## Add a type with parents
    /// - Register a type with zero, one or several parents.
    /// - Parents must already be registered, so cycles are impossible (forbiden A->B->A).
    pub fn addwp(&mut self, name: &str, parents: &[ItemTypeId]) -> Result<ItemTypeId, RegistryError> {
        if self.by_name.contains_key(name) {
            return Err(RegistryError::DuplicateName(name.to_string()));
        }
        for p: &ItemTypeId in parents {
            if p.index() >= self.names.len() {
                return Err(RegistryError::UnknownParent(*p));
            }
        }
        let id: ItemTypeId = ItemTypeId::new(self.names.len() as u32);
        let mut lineage: Vec<ItemTypeId> = vec![id];
        for p: &ItemTypeId in parents {
            for a in &self.lineage[p.index()] {
                if !lineage.contains(a) {
                    lineage.push(*a); // diamond (several paths to one ancestor): kept once
                }
            }
        }
        self.names.push(name.to_string());
        self.lineage.push(lineage);
        self.by_name.insert(name.to_string(), id);
        Ok(id)
    }

    /// ## Complete the build
    /// - Freeze the types (calling this methode will not allow you to add more types later)
    /// - the result can be shared between inventories.
    /// #### version 0.1.0
    /// - NO hot reload yet, must be recompiled if you want to add more types later
    pub fn build(self) -> Arc<ItemTypeRegistry> {
        Arc::new(ItemTypeRegistry {
            names: self.names,
            lineage: self.lineage,
            by_name: self.by_name,
        })
    }
}

/// Read-only registry of item types.
pub struct ItemTypeRegistry {
    names: Vec<String>,
    lineage: Vec<Vec<ItemTypeId>>,
    by_name: HashMap<String, ItemTypeId>,
}

impl ItemTypeRegistry {
    /// number of registered types
    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    pub fn name(&self, id: ItemTypeId) -> Option<&str> {
        self.names.get(id.index()).map(|s| s.as_str())
    }

    /// ids are not stable between runs: use names for saves and for the UI
    pub fn find(&self, name: &str) -> Option<ItemTypeId> {
        self.by_name.get(name).copied()
    }

    /// is `id` the type `of`, or one of its sub-types?
    pub fn is_a(&self, id: ItemTypeId, of: ItemTypeId) -> bool {
        self.lineage(id).contains(&of)
    }

    /// the type itself + all its ancestors (used by the inventory to feed the filter)
    pub(crate) fn lineage(&self, id: ItemTypeId) -> &[ItemTypeId] {
        self.lineage.get(id.index()).map_or(&[], |v| v.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lineage_and_errors() {
        let mut b = ItemTypeRegistryBuilder::new();
        let drink = b.add("drink", &[]).unwrap();
        let hot = b.add("hot drink", &[drink]).unwrap();
        let milk = b.add("with milk", &[]).unwrap();
        let latte = b.add("latte", &[hot, milk]).unwrap();

        assert_eq!(b.add("latte", &[]), Err(RegistryError::DuplicateName("latte".into())));
        let reg = b.build();
        assert_eq!(reg.len(), 4);
        assert_eq!(reg.name(latte), Some("latte"));
        assert_eq!(reg.find("latte"), Some(latte));
        assert!(reg.is_a(latte, drink) && reg.is_a(latte, hot) && reg.is_a(latte, milk));
        assert!(!reg.is_a(hot, latte) && !reg.is_a(drink, milk));
        assert_eq!(reg.lineage(latte).len(), 4); // itself + hot + drink + milk, drink counted once
    }
}