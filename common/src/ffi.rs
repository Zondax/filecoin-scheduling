/// Resource requirements struct comming from go
pub struct FfiResourceReq {
    /// The resource being requested: GPU, CPU, GPUmem
    resource: *const libc::c_char,
    /// The amount of resources requested
    quantity: libc::c_uint,
    /// It is preemtible
    preemptible: libc::c_char,
}

pub struct FfiResourceAlloc {
    requirements: FfiResourceReq,
    resource_id: libc::c_uint,
}
