
# Handle Manager
Handle Manager is a utility for tracking available handles in a system. These are useful ways to communicate across FFI boundaries or recycle memory from pre-allocated spaces. For instance you might preallocate a certain number of particle emitters, then use the handle manager to track which particular ones are available or not. Alternatively you may just use it as a means to hand opaque handles off to scripting, plugin or FFI layers.

# Policies
Handle management is augmented by setting policy enums. These are set when the manager is created and cannot be changed after IDs have been handed out.

Policy changes are particularly useful for allowing special debug modes, for situations like "track freed IDs, but don't hand out recycled IDs." Such a configuration is useful for detecting double-frees or aiding in finding stale references (ex. a script has ID#3 for a sound, which gets freed, then tries to play it again later; would result in playing the wrong sound in a release build but could be set in certain debug modes to throw errors instead.)

# Notes
Handles are always a `usize`, so there might be circumstances that this wastes memory. Changing `HangleManager` to a generic could allow for smaller handle types but this comes at the cost of contributing to compile-time slowdowns.

Policy setting might be settable as a generic parameter as well. Theoretically that should allow specializing handle allocation and releasing at compile-time; that would reduce runtime overhead marginally, but also contribute to compile-time slowdowns.

Currently there are no benchmarks that prove these optimizations are necessary, but feel free to open a ticket if you have found a problem in your use case.

Freed IDs are stored in a straightforward free vector. This always works, but is not as efficient as it could be. Let n be the maximum number of handles ever valid at once, and up to O(n) memory could be used to store free handles in the worst case. This could be fixed by storing spans that are twiddled around and coalesced, bringing memory usage down to O(n/2) in the worst case.

The worst case for the current implementation is ex. 1000 handles taken out, then returned in any order. There will then be 1000 handles on the free list. The worst case for the above fix is 1000 handles taken out, then every second handle is returned. Staggered IDs could not be compressed to a simple "start/stop" span.
