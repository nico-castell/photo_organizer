# CLI Photo organizer
[![Commits since last release](https://img.shields.io/github/commits-since/nico-castell/photo_organizer/latest?label=Commits%20since%20last%20release&color=informational&logo=Git&logoColor=white&style=flat-square)](https://github.com/nico-castell/photo_organizer/commits)
[![Crates version](https://img.shields.io/crates/v/iphone_organizer?color=informational&label=Crate%20version&logo=Rust&logoColor=white&style=flat-square)](https://crates.io/crates/iphone_organizer/versions)
[![License](https://img.shields.io/github/license/nico-castell/photo_organizer?label=License&color=informational&logo=Open%20Source%20Initiative&logoColor=white&style=flat-square)](LICENSE)
[![Tests](https://img.shields.io/github/workflow/status/nico-castell/photo_organizer/tests?label=tests&logo=GitHub%20Actions&logoColor=white&style=flat-square)](https://github.com/nico-castell/photo_organizer/actions/workflows/rust-tests.yml)

This program can take all of the files in the DCIM folder of an iPhone and organize them following a
directory structure like what follows:

```
/home/user/PhoneImport
├── 2021
│   ├── 11
│   │   ├── IMG_8000.jpg
│   │   └── IMG_8001.jpg
│   └── 12
│       ├── IMG_8002.jpg
│       └── IMG_8002.aae
└── 2022
    └── 01
        ├── IMG_8003.jpg
        └── IMG_8004.jpg
```

## Installation
To install this application, you will need to have **cargo** from the Rust language. If you don't
have it, you can refer to the installation instructions
[here](https://www.rust-lang.org/learn/get-started).

Then you simply run the following command in your terminal:

```
$ cargo install iphone_organizer
```

## Usage
The recommended first step is to copy the DCIM folder from your phone to your home directory.

You can use this program by opening a terminal, and typing the name of the command, the DCIM folder
in your home directory, and a destination folder, such as `~/PhoneImport`. Like this:

```
$ iphone_organizer ~/DCIM ~/PhoneImport
```

If you have edited any photos in your phone, the app will print the name of the corresponding
[.aae files](https://fileinfo.com/extension/aae).

```
$ iphone_organizer ~/DCIM ~/PhoneImport
/home/user/PhoneImport/2021/12/IMG_8002.aae
```

There are a couple of flags available for the command, you can see them all by running the following
command:

```
$ iphone_organizer --help
Usage:
      iphone_organizer SOURCE DESTINATION [OPTIONS]

Options:
   -s | --skip     ) Skips all files that are already present at DESTINATION.
                     This is the default.

   -o | --override ) Replaces files already present at DESTINATION with the
                     version from SOURCE.

   -l | --lean     ) Remove files present at DESTINATION but not SOURCE.

   -h | --help     ) Prints this help information.

Version: 1.2.2, MIT License
```

## About
This program and this repository are available under an [MIT License](LICENSE).
