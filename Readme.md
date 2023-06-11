# blazingly fast java version manager

This project aim to be a fast, efficient and easy to use java version manager

> **Warning**
> This project is in very early stage of development and thus is not recommended for professional use.

## Motivations

I made this project for 4 main reasons :
- I am a windows user that use cmd a lot and sdkman doesn't support it
- I do Java development in my professional life
- I wanted to learn rust
- I love cli tools

## Installation

> **Warning**
> This project is in very early stage and his usage can lead to weird behavior for your prompt environment. You should back up your HKCU environment reg key (windows user environment) and your .bashrc if you are a git bash user before use.

> **Warning**
> This project is only supported on windows at this stage.

### Install from binaries

> **Warning**
> Not supported yet.

### Install from source

#### Install cargo and cargo make 

- cargo : [Install cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- cargo make : [Install cargo make](https://github.com/sagiegurari/cargo-make)
- zip utility in path 

#### Build

```sh
cargo make release
```
#### Unzip archive 

- Choose a directory you have access to and unzip the archive
- Add the directory to PATH env variable

#### Run it 

##### Cmd 

```cmd
bf-j-vm.bat -h
```

##### Git bash

> **Note**
> On git bash you need to run init first

```bash
bf-j-vm.sh init git-bash
```

```bash
bf-j-vm.sh -h 
```

##### Powershell

> **Warning**
> Not supported yet

### Configuration

> **Warning**  
> At this stage of development you need to edit the json configuration file yourself. You can get the config directory path by running bf-j-vm with "get config-path" options.

The config path is a simple json file where you can reference your java installations and the correponding key to call it using bf-j-vm.

Example on windows :
```json
{
  "java_versions": {
    "17": {
      "home_folder": "C:/dev/interpreteur_compilateur/Java/jdk-17.0.1",
      "installed": true
    },
    "11": {
      "home_folder": "C:/dev/interpreteur_compilateur/Java/jdk-11.0.12+7",
      "installed": true
    },
    "graal17": {
      "home_folder": "C:/dev/interpreteur_compilateur/Java/graalvm-ce-java17-22.3.2",
      "installed": true
    }
  }
}
```

With this configuration i can call :
```cmd
bf-j-vm.bat java switch 11
```
To switch to java 11 globally on my system and refresh current java version for current prompt 
