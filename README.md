# CLI Photo organizer
[![Commits since last release](https://img.shields.io/github/commits-since/nico-castell/photo_organizer/latest?label=Commits%20since%20last%20release&color=informational&logo=Git&logoColor=white&style=flat-square)](https://github.com/nico-castell/photo_organizer/commits)
[![Release](https://img.shields.io/github/v/release/nico-castell/photo_organizer?label=Release&color=informational&logo=GitHub&logoColor=white&style=flat-square)](https://github.com/nico-castell/photo_organizer/releases)
[![License](https://img.shields.io/github/license/nico-castell/photo_organizer?label=License&color=informational&logo=Open%20Source%20Initiative&logoColor=white&style=flat-square)](LICENSE)
<!-- [![CodeQL](https://img.shields.io/github/workflow/status/nico-castell/photo_organizer/CodeQL?label=CodeQL&logo=GitHub%20Actions&logoColor=white&style=flat-square)](https://github.com/nico-castell/photo_organizer/actions/workflows/codeql-analysis.yml) -->

This program can take all of the files in the DCIM folder of an iPhone and organize them following a
directory structure like what follows:

```
/home/user/imported
├── 2021
│   ├── 11
│   │   ├── IMG8000.jpg
│   │   └── IMG8001.jpg
│   └── 12
│       ├── IMG8002.jpg
│       └── IMG8002.aae
└── 2022
    └── 01
        ├── IMG8003.jpg
        └── IMG8004.jpg
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
/home/user/PhoneImport/2021/12/IMG8002.aae
```

There are a couple of flags available for the command, you can see them all by running the following
command:

```
$ iphone_organizer --help
iphone_organizer SOURCE DESTINATION [OPTIONS]
Version: 1.0.2

Options:
   -s | --skip     ) Skips all files that are already present at DESTINATION.
                     This is the default.

   -o | --override ) Replaces files already present at DESTINATION with the
                     version from SOURCE.

   -h | --help     ) Prints this help information.
```

## About
This program and this repository are availabe under an [MIT License](LICENSE).
