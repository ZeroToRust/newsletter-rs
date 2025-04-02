# Link Time Optimizations  

This project is optimized for faster build and link times by using high-performance linkers. Faster linking significantly reduces compilation time, improving the development workflow, especially for large projects.  

## **Supported Linkers by Platform**  

- **Linux (`x86_64-unknown-linux-gnu`)**: Uses `mold`, a high-speed linker optimized for parallel processing.  
- **macOS (`x86_64-apple-darwin`)**: Uses `lld`, the LLVM linker known for its efficiency and performance.  

## **Installation Instructions**  

### **Linux**  
To install the `mold` linker, you can use `apt` package manager ship with Debian and Debian-based distributions.
Additionally, clang is required as the linker driver to use `mold`

```sh
sudo apt update
sudo apt install mold clang  
```
For other Linux distributions or package managers, refer to the [official mold installation guide](https://github.com/rui314/mold/blob/master/README.md).

### **macOS**  
Ensure Xcode Command Line Tools are installed, then install `lld` via Homebrew:

```sh
xcode-select --install  # Install Xcode Command Line Tools  
brew install lld        # Install lld  
```

## **Verification**  

Ensure the linkers are correctly installed:  

```sh
mold --version  # Linux  
lld --version   # macOS  
```

## **Testing the Setup**  

To confirm `mold` is being used, inspect the `.comment` section of a compiled executable:  

```sh
readelf -p .comment <executable-file>
```

If `mold` is used, you should see an entry similar to:  

```sh
String dump of section '.comment':
  [    2b]  mold 9a1679b47d9b22012ec7dfbda97c8983956716f7
```
