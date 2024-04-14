use super::{CompileContext, CompileError, Instruction, AtomBox};

#[derive(Debug)]
pub struct Label {
    name: String,
}

impl Label {
    pub fn new(name: String) -> Self {
        Label { name }
    }
}

impl Instruction for Label {
    fn compile(&self, ctx: &mut super::CompileContext) -> Result<(), super::CompileError> {
        ctx.emit_label(&self.name)
    }
}

#[derive(Debug)]
pub struct LabelScope {
  id: usize,
  atoms: Vec<AtomBox>
}

impl LabelScope {
  pub fn new(id: usize, atoms: Vec<AtomBox>) -> Self {
    LabelScope {
      id, atoms
    }
  }
}

impl Instruction for LabelScope {
    fn compile(&self, ctx: &mut CompileContext) -> Result<(), CompileError> {
        ctx.enter_local_scope(self.id);

        for atom in self.atoms.iter() {
          atom.compile(ctx)?;
        }

        ctx.leave_local_scope();

        Ok(())
    }
}