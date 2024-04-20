use std::{cell::RefCell, mem};

use super::{Atom, AtomBox, CompileContext, CompileError};

use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug, Clone)]
pub struct LabelResolver {
    label_map: HashMap<String, usize>,
    resolving_labels: bool,
    parent: Option<Box<LabelResolver>>,
}

impl LabelResolver {
    pub fn new() -> Self {
        LabelResolver {
            label_map: HashMap::new(),
            resolving_labels: true,
            parent: None,
        }
    }

    pub fn attach_parent(&mut self, parent: Box<LabelResolver>) {
        self.resolving_labels = parent.resolving_labels;
        self.parent = Some(parent);
    }

    pub fn detach_parent(&mut self) -> Box<LabelResolver> {
        self.parent.take().unwrap()
    }

    pub fn finish(&mut self) {
        self.resolving_labels = false;
    }

    pub fn register_label(&mut self, label: &String, id: usize) -> Result<(), CompileError> {
        match self.label_map.entry(label.to_owned()) {
            Entry::Occupied(_) => Err(CompileError::LabelRedefined(label.to_owned())),
            Entry::Vacant(v) => {
                v.insert(id);
                Ok(())
            }
        }
    }

    pub fn resolve_label_id(&mut self, label: &String) -> Result<usize, CompileError> {
        if self.resolving_labels {
            return Ok(usize::MAX);
        }
        
        if let Some(id) = self.label_map.get(label) {
            Ok(*id)
        } else if let Some(parent) = &mut self.parent {
            parent.resolve_label_id(label)
        } else {
            Err(CompileError::UnknownLabel(label.to_owned()))
        }
    }

    pub fn ready(&mut self) -> bool {
        !self.resolving_labels
    }
}


#[derive(Debug)]
pub struct Label {
    name: String,
    id: RefCell<Option<usize>>,
}

impl Label {
    pub fn new(name: String) -> Self {
        Label {
            name,
            id: RefCell::new(None),
        }
    }
}

impl Atom for Label {
    fn compile(&self, ctx: &mut super::CompileContext) -> Result<(), super::CompileError> {
        let mut id = self.id.borrow_mut();
        match *id {
            Some(label_id) => ctx.emit_label(label_id),
            None => {
                let new_id = ctx.emit_new_label();
                ctx.named_resolver.register_label(&self.name, new_id)?;
                *id = Some(new_id);
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub struct LabelScope {
    atoms: Vec<AtomBox>,
    scope: RefCell<Option<Box<LabelResolver>>>,
}

impl LabelScope {
    pub fn new(atoms: Vec<AtomBox>) -> Self {
        LabelScope {
            atoms,
            scope: RefCell::new(None),
        }
    }
}

impl Atom for LabelScope {
    fn compile(&self, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let mut parent = self
            .scope
            .borrow_mut()
            .take()
            .unwrap_or_else(|| Box::new(LabelResolver::new()));

        mem::swap(&mut ctx.named_resolver, &mut parent);
        ctx.named_resolver.attach_parent(parent);

        for atom in self.atoms.iter() {
            atom.compile(ctx)?;
        }

        let mut parent = ctx.named_resolver.detach_parent();
        mem::swap(&mut ctx.named_resolver, &mut parent);

        *self.scope.borrow_mut() = Some(parent);
        Ok(())
    }
}
