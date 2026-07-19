# MUCLI
**M**ulti **U**tility **C**ommand **L**ine **I**nterface

---

## Table of Contents
* [Overview](#overview) - What is MUCLI
  * [MUCLI Commands](#mucli-commands) - What can you do with MUCLI
* [Crates used](#crates-used) - What was used to create MUCLI
* [Contributing](#contributing) - How can you help out MUCLI's development

---

## Overview
> **ⓘ NOTE**  
> 1. This project was created to learn the fundamentals of *Rust* and should not be taken as a serious project.  
> 2. A lot of the functionalities might be really `basic` or `not complete`.
> 3. If you want to help improve **MUCLI**, visits the [*"Contributing"*](#contributing) section below.
> 4. All the functionalities of **MUCLI** were tested on a `Windows 11` machine.  

**MUCLI** is an *"all-in-one"* command-line application written in Rust. Rather than requiring multiple small utilities for everyday tasks, MUCLI brings a wide range of features together into a single program.

Whether you need to handle files, inspect your system, or perform other common operations, MUCLI aims to a simple and convenient way to do so directly from the terminal.  

In the following table of contents, you'll find an overview of every feature currently available in MUCLI, along with a brief description of what each command does.

## MUCLI Commands
- [File](#file)
    - Create
    - Delete
    - Write
    - Copy
    - Read
- [Dir](#dir)
    - Create
    - Delete
- [Images](#images)
- [System](#system)
  - Configs
- [Connection](#connection)
    - Ping
- [Misc](#misc)
  - Weather
  - Password-Generator
  - Credits

### File
The followings commands are used to handle file specific tasks.  

> **ⓘ NOTE**  
> The commands won't work if you don't have the authorization.

#### 1. Create
*Command*:  
`file create <file_name> <dir_path>`

*Example*:  
```bash
file create file_name.txt C:\Users\user\Desktop
```

This allows you to create a file in the specified directory (the extension `MUST` be included).

#### 2. Delete
*Command*:  
`file delete <file_path>`

*Example*:
```bash
file delete C:\Users\user\Desktop\file_name.txt
```

This allows you to delete a specified file (the extension `MUST` be included).

#### 3. Write
*Command*:  
`file write "<message>" <file_path> [FLAGS]`

*Example*:
```bash
file write "this is a message" C:\Users\user\Desktop\file_name.txt
```

*Flags*:  
This command contains flags that can be included.  
`-o` or `--overwrite` => overwrites the contents of the file.

This allows you to write contents in a specified file (the extension `MUST` be included).

#### 4. Copy
*Command*:  
`file copy <file_copy_path> <file_paste_path> [FLAGS]`

*Example*:
```bash
file copy C:\Users\user\file_name.txt C:\Users\user\NEW_DIR\file_name.txt
```

*Flags*:  
This command contains flags that can be included.  
`-o` or `--overwrite` => overwrites the contents of the file.

This allows you to copy the contents of a file inside another (the extension `MUST` be included).

#### 5. Read
*Command*:  
`file read <file_path>`

*Example*:
```bash
file read C:\Users\user\Desktop\file_name.txt
```

This allows you to read the contents of a file (the extension `MUST` be included).

### Dir

The followings commands are used to handle directory specific tasks.

> **ⓘ NOTE**  
> The commands won't work if you don't have the authorization.

#### 1. Create
*Command*:  
`dir create <dir_name> <dir_parent_directory>`

*Example*:
```bash
dir create new_dir C:\Users\user\Desktop
```

This allows you to create a new subdirectory, give a parent directory.

#### 2. Delete
*Command*:  
`dir delete <dir_path> [FLAGS]`

*Example*:
```bash
dir delete C:\Users\user\Desktop\new_dir
```

*Flags*:  
This command contains flags that can be included.  
`-s` or `--save-elements` => saves the files inside the specified directory to its parent directory (on the example, the files are going to be saved in "C:\Users\user\Desktop").

This allows you to delete a directory and its contents (with the possibility of saving them).

### Images
***WORK IN PROGRESS***

### System

The followings commands are used to show system settings.

#### 1. Config
*Command*:  
`system config [FLAGS]`

*Example*:
```bash
system config
```

*Flags*:  
This command contains flags that can be included.  
**If no flags are given, the command will show all the following settings.**  
`-s` or `--sys` => Shows system related information.  
`-m` or `--mem` => Shows information about your RAM.  
`-p` or `--proc` => Shows processes related information (process ID, name and disk usage).  
`-n` or `--network` => Shows network information (network interfaces name, total data received and data transmitted).

This allows you to show system settings.

### Connection
***WORK IN PROGRESS***

### Misc

The followings commands are miscellaneous settings.

#### 1. Weather
*Command*:  
`misc weather <latitude> <longitude> [FLAGS]`

*Example*:
```bash
misc weather 41.902782 12.496366
```

*Flags*:  
This command contains flags that can be included.  
**If no flags are given, the command will show all the following settings.**  
`--lat` => Shows the latitude.  
`--long` => Shows the longitude.  
`-e` or `--elevation` => Shows the elevation.  
`-t` or `--temperature` => Shows the hourly estimate of the temperature (**might not be correct**).
`-p` or `--precipitation` => Shows the hourly estimate of the precipitation (**might not be correct**).

This allows you to show weather information.

#### 2. Password-Generator
***WORK IN PROGRESS***

---

## Crates used
| Crate                                             | Version | Features |
|---------------------------------------------------|---------|----------|
| [anyhow](https://crates.io/crates/anyhow)         | 1.0.103 |          |
| [clap](https://crates.io/crates/clap)             | 4.6.1   | derive   |
| [reqwest](https://crates.io/crates/reqwest)       | 0.13.4  | blocking |
| [serde](https://crates.io/crates/serde)           | 1.0.228 | derive   |
| [serde_json](https://crates.io/crates/serde_json) | 1.0.150 |          |
| [sysinfo](https://crates.io/crates/sysinfo)       | 0.38.4  |          |
| [webbrowser](https://crates.io/crates/webbrowser) | 1.2.1   |          |

---

## Contributing
If you'd like to support the development of MUCLI, feel free to:
1. Clone the repository
    ```bash
    git clone https://github.com/usersolvesgits/MUCLI.git
    ```
    modify the code however you like and create a pull request.

2. Open an issue ([click here](https://github.com/usersolvesgits/MUCLI/issues)) explaining what you would like to be added in the future.

***Every contribution, even small ones, are welcome.***
