# Applied Physics II - NITIC

Practices of Applied Physics II and its development environment.

## Setup

### environment

You can use [`Dev Containers`](https://github.com/microsoft/vscode-dev-containers) to setup the development environment.

Before start, you need to clone this repository and pre-setup by running the following command:

```bash
git clone "https://github.com/dino3616/applied-physics-ii.git"
cd "applied-physics-ii"
cp ".env.example" ".env"
```

You need to press `Shift` + `Cmd` (`Ctrl`) + `P`, and type `Dev Containers: Open Folder in Container...`. Then, select the root directory of this repository.
Now all you have to do is sit back and wait!

## Compile and Run

Successed to setup, you can compile and run the program by running the following command:

```bash
cargo run --bin part[n]-prac[m]
```
