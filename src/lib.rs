
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
    /// Creates a new handle manager with some default settings. You very likely want to change
    /// these to suit your use case.
    pub fn new() -> Self {
        Default::default()
    }

    /// Changes the allocation policy of handles. Panics if handles have already been issued.
    pub fn with_alloc_policy(mut self, policy: AllocPolicy) -> Self {
        if self.highest_ever.is_some() {
            panic!("Cannot change allocation policy once handles have been dispensed.")
        }

        self.alloc_policy = policy;
        self
    }

    /// Changes the release policy of this handle manager. Panics if handles have already been issued.
    pub fn with_release_policy(mut self, policy: ReleasePolicy) -> Self {
        if self.highest_ever.is_some() {
            panic!("Cannot change release policy once handles have been dispensed.")
        }

        self.release_policy = policy;
        self
    }

    fn simple_allocate(&mut self) -> Option<usize> {
        let result = self.highest;
        // XXX what's the generic-safe way of doing this?
        if result == std::usize::MAX {
            None // We're out of IDs!
        } else {
            self.highest += 1;
            self.highest_ever = Some(result);
            Some(result)
        }
    }

    /// Retrieve a handle from the manager. Either generates a new ID if one cannot be recycled,
    /// or recycles one which was previously valid.
    pub fn next(&mut self) -> Option<usize> {
        match &self.release_policy {
            ReleasePolicy::DontTrack => self.simple_allocate(),
            ReleasePolicy::Tracked => {
                match self.alloc_policy {
                    AllocPolicy::NeverRecycle => self.simple_allocate(),
                    AllocPolicy::RecycleLowest => {
                        if self.freed.len() > 0 {
                            Some(self.freed.remove(0))
                        } else {
                            self.simple_allocate()
                        }
                    }
                }
            }
        }
    }

    /// Checks if a given ID is currently known to the handle manager. Note that if you are using
    /// a policy which does not track freed values, this can only check if a handle has never been
    /// valid to this point.
    pub fn is_used(&self, id: usize) -> bool {
        match self.highest_ever {
            None => false,
            Some(x) => {
                match &self.release_policy {
                    // when we don't track frees, we have to assume almost everything is still valid
                    ReleasePolicy::DontTrack => {id <= x},
                    // a little more cimplex
                    ReleasePolicy::Tracked => {
                        // if above highest ID ever assigned, can't be valid
                        if id > x {
                            return false
                        }
                        match self.freed.iter().find(|x| **x == id) {
                            Some(_) => false,
                            None => true,
                        }
                    },
                }
            },
        }
    }

    /// Returns a handle to the handle manager. This can fail if the handle was not valid or
    /// is currently in a free list.
    pub fn release(&mut self, handle: usize) -> Result<(),()> {
        // TODO think about how double-frees should be handled
        match &self.release_policy {
            ReleasePolicy::DontTrack => /* nothing to do */ Ok(()),
            ReleasePolicy::Tracked => {
                if self.is_used(handle) {
                    // XXX either an insertion sort or a heap would probably be better to be honest
                    self.freed.push(handle);
                    let mut jambojuice = &mut self.freed[..];
                    jambojuice.sort_unstable();
                    Ok(())
                } else {
                    // Uh oh. Either double free or invalid free attempt.
                    Err(())
                }
            },
        }
    }
}

#[cfg(test)]
mod test;
