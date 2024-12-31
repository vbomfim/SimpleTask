# SimpleTask

## Table of Contents
1. [Introduction](#introduction)
2. [Definitions](#definitions)
3. [To think about](#to-think-about)
4. [References](#references)
5. [Features](#features)
6. [Installation](#installation)
7. [Usage](#usage)

## Introduction
Before and foremost, this is a simple task manager written in Rust for experimentation.
It is a simple CLI and TUI (Terminal User Interface) application that allows you to create, delete, and list tasks.
This simple application allows to learn Rust and its ecosystem. It is not a production ready application, but it is a good starting point to learn Rust.
(But, who knows the future !?!?)

## Definitions

- **Task**: A task is a TO DO item, and it can have sub-tasks.
- **Issue**: An issue is a feature or problem that needs to be implemented/solved.
- **Release**: A release has a set of closed issues.

## To think about

- How to relate issues and/or releases to PRs (Pull Requests)?
- How could it be better integrated with Git?

## References
- Conventional Commits suggests a standard to document changes via commit description.
https://www.conventionalcommits.org/en/v1.0.0/#summary
- OSS Git Cliff automates ChangeLogs from commits based on Conventional Commits: https://git-cliff.org

## Features
- Issue CRUD
- Task CRUD
- List tasks
- Mark tasks as done
- Save tasks to a file
- Load tasks from a file

## Installation

To install SimpleTask, you need to have Rust installed on your system. You can install Rust from [rustup.rs](https://rustup.rs/).

Once you have Rust installed, you can install SimpleTask by running:

```bash
cargo install simple-task
```

## Usage

TBD - This section will be updated soon.
