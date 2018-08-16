
#[derive(Debug)]
pub enum AllocPolicy {
    /// Will always attempt to re-use the lowest ID out of all returned IDs.
    RecycleLowest,
}

pub struct HandleManager {
    alloc_policy: AllocPolicy
}

impl Default for HandleManager {
    fn default() -> Self {
        HandleManager{
            alloc_policy: AllocPolicy::RecycleLowest,
        }
    }
}

impl HandleManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_alloc_policy(mut self, policy: AllocPolicy) -> Self {
        self.alloc_policy = policy;
        self
    }

    pub fn next(&mut self, ) -> usize {
        unimplemented!()
    }

    pub fn release(&mut self, _handle: usize) {
        unimplemented!()
    }
}

#[cfg(test)]
mod test;
