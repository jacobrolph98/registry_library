This crate is intended to be used as a shared storage for generic stats, reducing data redundancy. This lets developers store a u32 Id component per entity, which can be used as a key to reference a struct containing the desired stats rather than have them be duplicated.

This library adds the following;
- Id<T> struct
    - Stores a u32, acting as the unique ID to index for a struct of type T containing the stats
- Interner
    - Maps a provided string ID to an Id<T> value, to be used during run time
- Registry
    - Maps Id<T> to stat struct upon registry