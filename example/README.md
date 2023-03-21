We add some modifies to extract the execution trace from `wasmi`.

```
[
    Trace {
        iaddr: InstructionPtr {
            ptr: 0x0000000000000000,
        },
        program_counter: 0,
        opcode: Unreachable,
        stack: [],
        stack_depth: 0,
        local: [
            UntypedValue {
                bits: 4,
            },
            UntypedValue {
                bits: 5,
            },
        ],
        local_depth: 2,
        calling_frame: [],
        calling_frame_depth: 0,
        action: None,
        memory_address: 0,
        memory: [],
    },
    Trace {
        iaddr: InstructionPtr {
            ptr: 0x00006000024f4340,
        },
        program_counter: 1,
        opcode: LocalGet {
            local_depth: LocalDepth(
                2,
            ),
        },
        stack: [
            UntypedValue {
                bits: 4,
            },
        ],
        stack_depth: 1,
        local: [
            UntypedValue {
                bits: 4,
            },
            UntypedValue {
                bits: 5,
            },
        ],
        local_depth: 2,
        calling_frame: [],
        calling_frame_depth: 0,
        action: None,
        memory_address: 0,
        memory: [],
    },
    Trace {
        iaddr: InstructionPtr {
            ptr: 0x00006000024f4350,
        },
        program_counter: 2,
        opcode: LocalGet {
            local_depth: LocalDepth(
                2,
            ),
        },
        stack: [
            UntypedValue {
                bits: 4,
            },
            UntypedValue {
                bits: 5,
            },
        ],
        stack_depth: 2,
        local: [
            UntypedValue {
                bits: 4,
            },
            UntypedValue {
                bits: 5,
            },
        ],
        local_depth: 2,
        calling_frame: [],
        calling_frame_depth: 0,
        action: None,
        memory_address: 0,
        memory: [],
    },
    Trace {
        iaddr: InstructionPtr {
            ptr: 0x00006000024f4360,
        },
        program_counter: 3,
        opcode: I64Add,
        stack: [
            UntypedValue {
                bits: 9,
            },
        ],
        stack_depth: 1,
        local: [
            UntypedValue {
                bits: 4,
            },
            UntypedValue {
                bits: 5,
            },
        ],
        local_depth: 2,
        calling_frame: [],
        calling_frame_depth: 0,
        action: None,
        memory_address: 0,
        memory: [],
    },
    Trace {
        iaddr: InstructionPtr {
            ptr: 0x00006000024f4370,
        },
        program_counter: 4,
        opcode: Return(
            DropKeep {
                drop: 2,
                keep: 1,
            },
        ),
        stack: [
            UntypedValue {
                bits: 9,
            },
        ],
        stack_depth: 1,
        local: [
            UntypedValue {
                bits: 4,
            },
            UntypedValue {
                bits: 5,
            },
        ],
        local_depth: 2,
        calling_frame: [],
        calling_frame_depth: 0,
        action: None,
        memory_address: 0,
        memory: [],
    },
]
```
