Kod wykonujemy w folderze /ZAKO_RISC i komendą: cargo run -- jump_test.cod pipe2.dat lub cargo run -- pipe2_2.cod pipe2.dat
Wykonujemy w ten sposób przykladowe dzialanie kodu z wieloma skokami(jump_test) oraz program SOI na lab2 z ZAKO(pipe2_2). Aktualnie bez stalli problematycznym jest dzialanie innych programow(bledy).

Ogólna komenda:

```
cargo run -- PLIK_Z_KOMENDAMI_COD PLIK_Z_PAMIECIA_DAT
```
WAŻNE! W PRZYPADKU WŁASNEGO PROGRAMU, .COD MUSI BYĆ EDYTOWANY NA FORMAT .COD PODOBNY W PRZYKLADOWYCH. TO ZNACZY, ŻE MUSZĄ BYĆ JEDYNIE MNEMONIKI I REJESTRY!!!

Dostępne mnemoniki w instruction.rs w enum ALU:
```rust
enum ALU {
    NOP,
    ADD(i32, i32),
    SUB(i32, i32),
    MUL(i32, i32),
    DIV(i32, i32),
    AND(i32, i32),
    OR(i32, i32),
    XOR(i32, i32),
    LOAD(u32),
    STORE(i32, u32),
    BRZ(i32, i32),
    BRNZ(i32, i32),
    BRGT(i32, i32),
    BRGE(i32, i32),
    BRLT(i32, i32),
    BRLE(i32, i32),
}
```
