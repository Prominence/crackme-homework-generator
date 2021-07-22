# How to use
```shell
./crackme-homework-generator --help
./crackme-homework-generator -n 10
./crackme-homework-generator -n 10 >> keys.txt
```

# How to build

## To build and execute on the same system
```shell
cargo build --release
cd target/release
./crackme-homework-generator --help
```

## To build on linux and execute on Windows system
### Prerequisites
Arch Linux:
```shell
yay -S mingw-w64
rustup target add x86_64-pc-windows-gnu 
rustup toolchain install stable-x86_64-pc-windows-gnu
````

```shell
cargo build --target x86_64-pc-windows-gnu --release
cd target/x86_64-pc-windows-gnu/release
...
````
Copy to Windows system **crackme-homework-generator.exe** file
```shell
crackme-homework-generator.exe --help
```

## Example output
```shell
❯ ./crackme-homework-generator -n 10
0563-3236-4190-1904
1477-7219-3871-4654
8270-6092-1817-0971
3928-9274-3649-9832
8040-0732-4017-1191
5234-4415-6152-5711
9177-8781-6675-8088
9060-4731-0582-3318
0958-4729-6970-1867
1084-3613-1435-6304

```