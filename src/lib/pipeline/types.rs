use std::sync::Arc;

pub type PipelineFunction<T, E> = Fn(T, Arc<Pipeline<T, E>>) -> Result<T, E>;

pub trait Pipeline<T, E> {
    fn call(&self, context: T) -> Result<T, E>;
}

pub struct PipelineElement<T, E> {
    func: Arc<PipelineFunction<T, E>>,
    next: Arc<Pipeline<T, E>>
}

impl <T, E> PipelineElement<T, E> {
    pub fn new(func: Arc<PipelineFunction<T, E>>, next: Arc<Pipeline<T, E>>) -> Self {
        return PipelineElement{func, next};
    }
}

impl <T, E> Pipeline<T, E> for PipelineElement<T, E> {
    fn call(&self, context: T) -> Result<T, E> {
        (self.func)(context, self.next.clone())
    }
}

pub struct PipelineTail { }

impl <T, E> Pipeline<T, E> for PipelineTail {
    fn call(&self, context: T) -> Result<T, E> {
        return Result::Ok(context)
    }
}