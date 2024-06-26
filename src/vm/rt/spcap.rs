
#[derive(Debug, Clone)]
pub struct SpaceCap {
    pub call_depth: usize, // 16
    pub load_contract: usize, // 20

    pub total_stack: usize, // 16*16 = 256
    pub total_local: usize, // 16*16 = 256

    pub max_heap_seg: usize, // 256 * 64 = 16kb

    pub max_global: usize, // 32
    pub max_memory: usize, // 12

}

impl SpaceCap {

    pub fn new() -> SpaceCap {

        SpaceCap {
            call_depth:     16,
            load_contract:  20,
            total_stack:    256,
            total_local:    256,
            max_heap_seg:   64,
            max_global:     32,
            max_memory:     12,
        }
    }

}



