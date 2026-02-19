use std::ptr::NonNull;

// Using manual casting because what is passed and
// what is expected both controlled by generated code.
struct DynFnDataParam {
    ptr: NonNull<()>,
}
