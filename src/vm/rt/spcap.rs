
#[derive(Debug, Clone)]
pub struct SpaceCap {
    pub call_depth: usize, // 16
    pub load_contract: usize, // 20
    pub inherit_contract: usize, // 4

    pub total_stack: usize, // 16*16 = 256
    pub total_local: usize, // 16*16 = 256

    pub max_heap_seg: usize, // 256 * 64 = 16kb

    pub max_global: usize, // 32
    pub max_memory: usize, // 12

    pub max_ctl_func: usize, // 200 cache
    pub max_ctl_libx: usize, // 100 cache
    pub max_ctl_body: usize, // 50 cache

}

impl SpaceCap {

    pub fn new() -> SpaceCap {

        SpaceCap {
            call_depth:        16,
            load_contract:     20,
            inherit_contract:  4,
            total_stack:       256,
            total_local:       256,
            max_heap_seg:      64,
            max_global:        32,
            max_memory:        12,
            max_ctl_func:      200,
            max_ctl_libx:      100,
            max_ctl_body:      50,
        }
    }

}



