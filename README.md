# Turing Machine 2.0
An improved Turing Machine made in Rust!

The [old one](https://github.com/margual56/TuringMachine) was not great in many ways, so I decided to completely re-write it from scratch!

## Demo:
![demo](https://user-images.githubusercontent.com/30444886/203064166-a36a2693-7c5f-4fb1-a71f-bbc50fa31e09.gif)

## Installation
<details>
<summary style="font-size: 1.25em">Option 0: Access the WebAssembly version</summary>
I have compiled the program to WASM, and hosted it in <a href="https://turing.coldboard.net">https://turing.coldboard.net</a>.

The web version is less efficient and is more limited than the local version. It also can be subject to breakages from time to time.

I designed the application with an "offline-first" approach, so the recommended way to use it is to download it.

<p style="color: #af0f0f">Please, if you intend to use the Turing Machine for an extended period of time, consider downloading the latest version and using it that way</p>
</details>

<details>
<summary style="font-size: 1.25em">Option 1: Download the binary</summary>
<ul><li> Go to <a href="https://github.com/margual56/turing-machine-2.0/releases/latest">the releases page</a> and download the latest executable file for your operating system.</li>
<li>In Linux you will probably need to make it executable (`chmod +x turing-machine-linux`).</li>
</ul>
</details>

<details>
<summary style="font-size: 1.25em">Option 2: Compile it yourself</summary>
<ul><li>Run <code>cargo install --git "https://github.com/margual56/turing-machine-2.0"</code></li>
<li>You also need to add the cargo folder to your path (for example, in UNIX: <code>export PATH=$PATH:$HOME/.cargo/bin</code>)</li>
</details><br/>


## Usage
Just run the executable to get a GUI out-of-the-box. If you want to use the CLI, run it through the console with the argument `--cli`!

## Programming it
The favored programming IDE is VScode (or code-OSS), for which I have created a syntax highlighter for the Turing Machine code (`.tm`).

The extension is also Open Source and can be found here: [https://github.com/margual56/vscode-turing-machine](https://github.com/margual56/vscode-turing-machine)

You can install it from your editor's tab or downloading it: 
- [VSCode extension](https://marketplace.visualstudio.com/items?itemName=MarcosGutirrezAlonso.turing-machine)
- [Open-vsx (code-OSS) extension](https://open-vsx.org/extension/MarcosGutirrezAlonso/turing-machine)