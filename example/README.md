We add some modifies to extract the execution trace from `wasmi`.

-   **iaddr:** Address of instruction in the memory
-   **program_counter:** Number of instructions have been executed
-   **opcode:** Opcode has been executed
-   **stack:** The stack of Wasm run-time
-   **stack_depth:** The depth of the above stack
-   **local:** Local variables which can be accessed by using `local.get <index>`
-   **local_depth:** The depth of local
-   **calling_frame:** Calling frame of Wasm run-time
-   **calling_frame_depth:** The depth of calling frame
-   **action:** Memory access action
-   **memory_address:** Memory address
-   **memory:** Memory chunks

```
Trace {
    iaddr: InstructionPtr {
        ptr: 0x0000000131607800,
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
}
```
