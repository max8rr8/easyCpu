use crate::{
    asm::{alu::AluOperation, jump::JumpOperation},
    stack::{
        instr::{
            func::FunctionOperation, local::LocalOperation, manip::{ManipStackOp, ManipStackOperation}, AluStackOp, ConstStackOp, FunctionStackOp, JumpStackOp, LabelStackOp, LocalStackOp
        }, optatom::LabelMarkStackOp, StackOperation
    },
};

struct OptimizationCtx {
    compiled: Vec<Box<dyn StackOperation>>,
    queue: Vec<Box<dyn StackOperation>>,
}

impl OptimizationCtx {
    fn look_at(&self, i: usize) -> Option<&dyn StackOperation> {
        if i > self.compiled.len() {
            return None;
        }

        self.compiled
            .get(self.compiled.len() - i)
            .map(|x| x.as_ref())
    }

    fn look_as<T: 'static>(&self, i: usize) -> Option<&T> {
        if i > self.compiled.len() {
            return None;
        }

        self.compiled
            .get(self.compiled.len() - i)
            .and_then(|x| x.as_any().downcast_ref::<T>())
    }

    fn take(&mut self) -> Option<Box<dyn StackOperation>> {
        self.compiled.pop()
    }

    fn queue<T: StackOperation + 'static>(&mut self, op: T) {
        self.queue.push(Box::new(op));
    }
}

fn optimize_svar_lvar(ctx: &mut OptimizationCtx) -> bool {
    let Some(load_op) = ctx.look_as::<LocalStackOp>(1) else {
        return false;
    };

    let load_mode = match load_op.op {
        LocalOperation::LOAD(mode) => mode,
        _ => {
            return false;
        } // Not load op
    };

    let load_idx = load_op.idx;

    let Some(store_op) = ctx.look_as::<LocalStackOp>(2) else {
        return false;
    };

    let store_mode = match store_op.op {
        LocalOperation::STORE(mode) => mode,
        _ => {
            return false;
        } // Not load op
    };

    if load_mode != store_mode {
        return false;
    }

    if load_idx != store_op.idx {
        return false;
    }

    ctx.take();
    ctx.take();

    // Reverse order
    ctx.queue(LocalStackOp::new(LocalOperation::STORE(load_mode), load_idx).unwrap());
    ctx.queue(ManipStackOp::dup());

    true
}

fn optimize_pconst_add(ctx: &mut OptimizationCtx) -> bool {
    let Some(alu_op) = ctx.look_as::<AluStackOp>(1) else {
        return false;
    };

    let is_sub = match alu_op.op {
        AluOperation::ADD => false,
        AluOperation::SUB => true,
        _ => return false,
    };

    let Some(const_op) = ctx.look_as::<ConstStackOp>(2) else {
        return false;
    };

    if const_op.do_add {
        return false;
    }

    let val = if is_sub {
        0u16.wrapping_sub(const_op.val)
    } else {
        const_op.val
    };

    ctx.take();
    ctx.take();

    ctx.queue(ConstStackOp::new(val, true));

    true
}

fn optimize_drop_pure(ctx: &mut OptimizationCtx) -> bool {
    let Some(manip_op) = ctx.look_as::<ManipStackOp>(1) else {
        return false;
    };

    let drop_cnt: usize = match manip_op.op {
        ManipStackOperation::Drop(cnt) => cnt as usize,
        _ => return false,
    };

    let Some(dropped_op) = ctx.look_at(2) else {
        return false;
    };

    let signature = dropped_op.signature();
    if signature.flags != 0 || signature.pushes > drop_cnt {
        return false;
    }

    let new_dropped = drop_cnt - signature.pushes + signature.takes;

    ctx.take();
    ctx.take();

    ctx.queue(ManipStackOp::drop(new_dropped as u8));

    true
}


fn optimize_jmp_ret_clean(ctx: &mut OptimizationCtx) -> bool {
  if let Some(ret_op) = ctx.look_as::<FunctionStackOp>(2) {
      if ret_op.op != FunctionOperation::RETURN {
          return false;
      }
  } else if let Some(jmp_op) = ctx.look_as::<JumpStackOp>(2) {
      if jmp_op.op != JumpOperation::JMP {
          return false;
      }
  } else {
      return false;
  }

  if ctx.look_as::<LabelMarkStackOp>(1).is_some() {
      return false;
  }

  ctx.take();

  true
}


fn optimize_label_aconst(ctx: &mut OptimizationCtx) -> bool {
  let Some(cons_op) = ctx.look_as::<ConstStackOp>(1) else {
    return false;
  };

  if !cons_op.do_add {
    return false;
  }

  let shift = cons_op.val;

  let Some(label_op) = ctx.look_as::<LabelStackOp>(2) else {
      return false;
  };

  let mut newl = *label_op;

  ctx.take();
  ctx.take();

  newl.shift = newl.shift.wrapping_add(shift);
  ctx.queue(newl);

  true
}


pub fn optimize(ops: Vec<Box<dyn StackOperation>>) -> Vec<Box<dyn StackOperation>> {
    let mut ctx = OptimizationCtx {
        compiled: Vec::new(),
        queue: ops,
    };
    ctx.queue.reverse();

    loop {
        if optimize_svar_lvar(&mut ctx)
            || optimize_pconst_add(&mut ctx)
            || optimize_drop_pure(&mut ctx)
            || optimize_jmp_ret_clean(&mut ctx)
            || optimize_label_aconst(&mut ctx)
        {
            continue;
        } else if let Some(op) = ctx.queue.pop() {
            ctx.compiled.push(op);
        } else {
            break;
        }
    }
    ctx.compiled
}
