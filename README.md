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
- The key must be kept secret and be used only once

## Usage
- Build the project with `cargo build --release`
- Run the program (with `cargo run --release`)

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
Or
```shell
./one_time_pad --encrypt <file>
```

#### Optionnal arguments:
- `--dir`: Recursively encrypt all files in the directory
- `--delete`: Delete the original file after encryption
##### Example:
```shell
./one_time_pad --encrypt --dir --delete <folder>
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
Or
```shell
./one_time_pad --decrypt <file>
```

#### Optionnal arguments:
- `--dir`: Recursively decrypt all files in the directory
- `--secure`: Fills the pad with zeros before deleting it
##### Example:
```shell
./one_time_pad --decrypt --dir --secure <folder>
```

## Disclaimer
This implementation is for educational purposes only. **Do not use it to encrypt sensitive data** as I am not a cryptographer and I cannot guarantee the security of this implementation.
I am not responsible for any misuse of this software.