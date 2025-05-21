# Moved to https://codeberg.org/demostf/plugin

# backup-rs

Backup program for demos.tf demos.

A simple program that incrementally backs up every demo file from demos.tf to a local directory.

## Usage

The following environment variables are required for the program

    STORAGE_ROOT: The directory to store the demos in
    STATE_FILE: The textfile to store the backup progress in between runs

The program will look in a .env file if the variables aren't set in the environment
