use zkwasm::proof_context::proof_context::ProofContext;
use zkwasm::wasm_context::wasm_context::WasmContext;

fn main() {
    /*
    (param i64 i64) (result i64)
        local.get 0
        local.get 1
        i64.add))

    [0x20, 0x00, 0x20, 0x1, 0x7c, 0xb]
    */

    let mut proof_context = ProofContext::new();

    let mut a = WasmContext::new(
        &[0x20u8, 0x00, 0x20, 0x1, 0x7c, 0xb],
        &[
            0x0u8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2,
        ],
    );

    a.next(&mut proof_context);
    a.next(&mut proof_context);
    a.next(&mut proof_context);
    a.next(&mut proof_context);

    proof_context.verify_trace_in_plain();
}
