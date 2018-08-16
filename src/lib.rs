
#[derive(Debug)]
pub enum AllocPolicy {
    /// Will not recycle released handles, regardless of whether they are tracked.
    NeverRecycle,
    /// Will always attempt to re-use the lowest ID out of all returned IDs.
    RecycleLowest,
}

#[derive(Debug)]
pub enum ReleasePolicy {
    /// Free handles are never tracked, and thus never re-used.
    DontTrack,
    /// Tracks a free list of handles.
    Tracked,
}

pub struct HandleManager {
    alloc_policy: AllocPolicy,
    release_policy: ReleasePolicy,
    highest: usize,
    highest_ever: Option<usize>,
    freed: Vec<usize>,
}

impl Default for HandleManager {
    fn default() -> Self {
        HandleManager{
            alloc_policy: AllocPolicy::RecycleLowest,
            release_policy: ReleasePolicy::DontTrack,
            highest: 0,
            highest_ever: None,
            freed: Vec::with_capacity(0),
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

    fn simple_allocate(&mut self) -> usize {
        let result = self.highest;
        self.highest += 1;
        self.highest_ever = Some(result);
        result
    }

    pub fn next(&mut self, ) -> usize {
        match &self.release_policy {
            ReleasePolicy::DontTrack => self.simple_allocate(),
            ReleasePolicy::Tracked => {
                match self.alloc_policy {
                    AllocPolicy::NeverRecycle => self.simple_allocate(),
                    AllocPolicy::RecycleLowest => {
                        if self.freed.len() > 0 {
                            self.freed.remove(0)
                        } else {
                            self.simple_allocate()
                        }
                    }
                }
            }
        }
    }

    pub fn release(&mut self, handle: usize) {
        // TODO think about how double-frees should be handled
        match &self.release_policy {
            ReleasePolicy::DontTrack => {/* nothing to do */},
            ReleasePolicy::Tracked => {
                // XXX either an insertion sort or a heap would probably be better to be honest
                self.freed.push(handle);
                let mut jambojuice = &mut self.freed[..];
                jambojuice.sort();
            },
        }
    }
}

#[cfg(test)]
mod test;
