# One Time Pad

## Description
This is a simple implementation of the One Time Pad encryption algorithm. The One Time Pad is a symmetric encryption algorithm that uses a key that is (at least) as long as the message to encrypt. The key is generated randomly and is only used once. The key is then XORed with the message to encrypt it.

## Avantages of the One Time Pad
- The One Time Pad is unbreakable if the key is truly random and is only used once.
- Rather simple to implement
- The encryption is fast

## Disadvantages of the One Time Pad
- The key must be as long as the message to encrypt, which makes it impractical with bigger files.
- The key must be **_truly_** random, which is hard to achieve in practice.
- The key must be kept secret
- The key must be used only once

## Usage
- Build the project with `cargo build --release`
- Run the program (with `cargo run --release`)

## Usage
### Encrypting a file
```shell
1. Encrypt
2. Decrypt
3. Exit

Enter your choice: 1

Enter the path of the file to encrypt: example.bin
Generating pad...
Reading file...
Encrypting file...
File encrypted successfully!
```

### Decrypting a file
```shell
1. Encrypt
2. Decrypt
3. Exit

Enter your choice: 2

Enter the path of the file to decrypt: example.bin
Reading file...
Decrypting file...
File decrypted successfully!
```

## Disclaimer
This implementation is for educational purposes only. Do not use it to encrypt sensitive data as I am not a cryptographer and I cannot guarantee the security of this implementation.
I am not responsible for any misuse of this software.
```