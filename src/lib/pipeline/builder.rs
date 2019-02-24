use std::sync::Arc;

use super::{PipelineFunction, Pipeline, PipelineElement, PipelineTail, PipelineStage};

pub struct PipelineBuilder<T, E> {
    elements: Vec<Arc<PipelineFunction<T, E>>>
}

impl <T, E> PipelineBuilder<T, E> where T: 'static, E: 'static {
    pub fn new() -> Self {
        PipelineBuilder{
            elements: vec![]
        }
    }

    pub fn next_func(&mut self, func: &'static PipelineFunction<T, E>) -> &mut Self {
        self.elements.push(Arc::new(func));
        self
    }

    pub fn next_stage<S : PipelineStage<T, E> + 'static>(&mut self, stage: S) -> &mut Self {
        let func = move |context, next| stage.process(context, next);

        self.elements.push(Arc::new(func));
        self
    }

    pub fn build(&self) -> Arc<Pipeline<T, E>> {
        return self.elements.iter().rev().fold(
            Arc::new(PipelineTail{}),
            |pipeline,func| {
                return Arc::new(PipelineElement::new(func.clone(), pipeline))
            }
        )
    }
}