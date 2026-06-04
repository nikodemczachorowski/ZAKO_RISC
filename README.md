Kod wykonujemy w folderze /ZAKO_RISC i komendą: cargo run -- jump_test.cod pipe2.dat lub cargo run -- pipe2_2.cod pipe2.dat
Wykonujemy w ten sposób przykladowe dzialanie kodu z wieloma skokami(jump_test) oraz program SOI na lab2 z ZAKO(pipe2_2). Aktualnie bez stalli problematycznym jest dzialanie innych programow(bledy).

Ogólna komenda:

```
cargo run -- PLIK_Z_KOMENDAMI_COD PLIK_Z_PAMIECIA_DAT
```
WAŻNE! W PRZYPADKU WŁASNEGO PROGRAMU, .COD MUSI BYĆ EDYTOWANY NA FORMAT .COD PODOBNY W PRZYKLADOWYCH. TO ZNACZY, ŻE MUSZĄ BYĆ JEDYNIE MNEMONIKI I REJESTRY!!!

Dostępne mnemoniki w files.rs w wczytywaniu z pliku:
```rust
 let opcode: u8 = match parts[0].to_uppercase().as_str() {
            "ADD" => 0x01,
            "ADDI" => 0x11,
            "SUB" => 0x02,
            "SUBI" => 0x12,
            "MUL" => 0x03,
            "MULI" => 0x13,
            "DIV" => 0x04,
            "DIVI" => 0x14,
            "AND" => 0x05,
            "ANDI" => 0x15,
            "OR" => 0x06,
            "ORI" => 0x16,
            "XOR" => 0x07,
            "XORI" => 0x17,
            "LDW" => 0x18,
            "STW" => 0x19,
            "BRZ" => 0x20,
            "BRNZ" => 0x21,
            "BRGT" => 0x22,
            "BRGE" => 0x23,
            "BRLT" => 0x24,
            "BRLE" => 0x25,
            "NOP" => 0x00,
            instr => panic!("Invalid instruction: {}", instr),
        };
```
