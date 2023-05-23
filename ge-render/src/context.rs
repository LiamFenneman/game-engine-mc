use ge_util::EngineConfig;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Debug)]
pub(crate) struct Context(Arc<Mutex<InnerContext>>);

#[derive(Debug)]
pub(crate) struct InnerContext {
    pub config: EngineConfig,
    pub uniform_bind_group: wgpu::BindGroup,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
}

impl Context {
    pub(crate) fn new(
        config: EngineConfig,
        uniform_bind_group: wgpu::BindGroup,
        uniform_bind_group_layout: wgpu::BindGroupLayout,
    ) -> Self {
        return Self(Arc::new(Mutex::new(InnerContext {
            config,
            uniform_bind_group,
            uniform_bind_group_layout,
        })));
    }

    pub(crate) fn lock(&'_ self) -> MutexGuard<'_, InnerContext> {
        return self.0.lock().expect("another user of the mutex panicked");
    }
}

impl Clone for Context {
    fn clone(&self) -> Self {
        return Self(Arc::clone(&self.0));
    }
}
