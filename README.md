janalyze
========

A command-line tool for analyzing and cleaning project directories.

Overview
--------

janalyze is a fast and efficient Rust-based utility that scans project
directories, identifies build artifacts and dependency caches, and
reclaims disk space by removing unnecessary files.

Features
--------

- Detect project types based on configuration files
- Identify cleanup targets specific to each project type
- Calculate the size of directories to be removed
- Single directory or recursive scanning/cleaning
- Support for 13+ project types
- Colored terminal output for better readability
- Detailed size reporting

Supported Project Types
-----------------------

- Rust (target/)
- Node.js (node_modules/, dist/, .next/)
- Python (__pycache__/, dist/, build/)
- Django (__pycache__/, staticfiles/, media/)
- Java (target/)
- Go (bin/, pkg/)
- C# (bin/, obj/)
- Flutter (build/, .dart_tool/)
- React (node_modules/, build/)
- Vue.js (node_modules/, dist/)
- Angular (node_modules/, dist/, angular.json)
- Ruby on Rails (log/, tmp/, Gemfile)
- Laravel (vendor/, storage/framework/cache/)
- Gradle (build/)

Installation
------------

To build from source, ensure you have Rust installed. Then run:

  cargo build --release

The compiled binary will be available in target/release/janalyze

Usage
-----

Display help:

  janalyze help

Scan a single directory:

  janalyze scan <path>

Clean a single directory:

  janalyze clean <path>

Scan recursively (all subdirectories):

  janalyze scan /path/to/projects --recursive

Clean recursively (all subdirectories):

  janalyze clean /path/to/projects --recursive

Examples
--------

Scan current directory for cleanup targets:

  janalyze scan .

Clean all projects in a directory tree:

  janalyze clean ~/projects --recursive

Check how much space can be freed in a Node.js project:

  janalyze scan ~/my-app

How It Works
------------

1. Scans the target directory for project configuration files
   (Cargo.toml, package.json, pom.xml, etc.)

2. Matches the configuration file to a cleanup rule

3. Identifies cleanup targets based on the detected project type

4. Calculates the total size of files to be removed

5. For clean mode, permanently removes the identified directories

License
-------

This project is licensed under the MIT License.
See LICENSE file for details.

Contributing
------------

Contributions are welcome! Feel free to submit pull requests or
open issues for bugs and feature requests.

Author
------

balint0513
