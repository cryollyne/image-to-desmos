use std::marker::PhantomData;

use crate::Args;

pub trait Pipeline<I, O>: Sized + Transform<I, O> {
    fn exec_pipeline(self, i: I, args: &Args) -> O {
        let total_steps = self.total_steps();
        self.map(i, 1, total_steps, args)
    }

    fn compose<N, R>(self, other: R) -> CompositeTransform<I, N, Self, R, O>
    where
        Self: Pipeline<I, O>,
        R: Pipeline<O, N>,
    {
        CompositeTransform::new(self, other)
    }
}

pub trait Transform<I, O>: Sized {
    fn total_steps(&self) -> u32;
    fn map(self, i: I, step: u32, total_steps: u32, args: &Args) -> O;
    fn get_name(&self) -> &str;
}

impl<T, I, O> Pipeline<I, O> for T where T: Transform<I, O> {}

#[derive(Debug)]
pub struct FunctionTransform<'a, I, O, F>
where
    F: FnOnce(I) -> O
{
    f: F,
    name: &'a str,
    _i: PhantomData<I>,
    _o: PhantomData<O>,
}

impl<'a, I, O, F> FunctionTransform<'a, I, O, F>
where
    F: FnOnce(I) -> O
    {
    pub fn new(f: F, name: &str) -> FunctionTransform<I, O, F> {
        FunctionTransform {
            f,
            name,
            _i: Default::default(),
            _o: Default::default(),
        }
    }
}

impl<'a, I, O, F> Transform<I, O> for FunctionTransform<'a, I, O, F>
where
    F: FnOnce(I) -> O
{
    fn total_steps(&self) -> u32 {
        1
    }
    fn get_name(&self) -> &str {
        self.name
    }

    fn map(self, i: I, step: u32, total_steps: u32, args: &Args) -> O {
        if args.verbose {
            eprintln!("[{step}/{total_steps}]\t{}", self.get_name());
        }
        (self.f)(i)
    }
}

pub struct CompositeTransform<I, O, L, R, M>
where
    L: Pipeline<I, M>,
    R: Pipeline<M, O>,
{
    left: Box<L>,
    right: Box<R>,
    _i: PhantomData<I>,
    _o: PhantomData<O>,
    _m: PhantomData<M>,
}

impl<I, O, L, R, M> CompositeTransform<I, O, L, R, M>
where
    L: Pipeline<I, M>,
    R: Pipeline<M, O>,
{
    pub fn new(left: L, right: R) -> CompositeTransform<I, O, L, R, M> {
        CompositeTransform {
            left: Box::new(left),
            right: Box::new(right),
            _i: Default::default(),
            _o: Default::default(),
            _m: Default::default(),
        }
    }
}

impl<I, O, L, R, M> Transform<I, O> for CompositeTransform<I, O, L, R, M>
where
    L: Pipeline<I, M>,
    R: Pipeline<M, O>,
{
    fn total_steps(&self) -> u32 {
        self.left.total_steps() + self.right.total_steps()
    }

    fn get_name(&self) -> &str {
        "Composite Transform"
    }

    fn map(self, i: I, step: u32, total_steps: u32, args: &Args) -> O {
        let left_steps = self.left.total_steps();
        self.right.map(
            self.left.map(i, step, total_steps, args),
            step + left_steps,
            total_steps,
            args,
        )
    }
}
