
#[derive(Debug)]
pub enum AllocPolicy {
    /// Will always attempt to re-use the lowest ID out of all returned IDs.
    RecycleLowest,
}

#[derive(Debug)]
pub enum ReleasePolicy {
    /// Free handles are never tracked, and thus never re-used.
    DontTrack,
}

pub struct HandleManager {
    alloc_policy: AllocPolicy,
    release_policy: ReleasePolicy,
    highest: usize,
    highest_ever: Option<usize>,
}

impl Default for HandleManager {
    fn default() -> Self {
        HandleManager{
            alloc_policy: AllocPolicy::RecycleLowest,
            release_policy: ReleasePolicy::DontTrack,
            highest: 0,
            highest_ever: None,
        }
    }
}

impl HandleManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_alloc_policy(mut self, policy: AllocPolicy) -> Self {
        if self.highest_ever.is_some() {
            panic!("Cannot change allocation policy once handles have been dispensed.")
        }

        self.alloc_policy = policy;
        self
    }

    pub fn with_release_policy(mut self, policy: ReleasePolicy) -> Self {
        if self.highest_ever.is_some() {
            panic!("Cannot change release policy once handles have been dispensed.")
        }

        self.release_policy = policy;
        self
    }

    pub fn next(&mut self, ) -> usize {
        match &self.release_policy {
            ReleasePolicy::DontTrack => {
                let result = self.highest;
                self.highest += 1;
                self.highest_ever = Some(result);
                result
            }
        }
    }

    pub fn release(&mut self, _handle: usize) {
        match &self.release_policy {
            ReleasePolicy::DontTrack => {/* nothing to do */},
        }
    }
}

#[cfg(test)]
mod test;
