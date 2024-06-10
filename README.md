# Hang

The **hang** is designed to make your system hang or pause for a specified duration or until a specific time.

## Usage
- To hang for a duration, execute:
  ```
  $ ./hang <duration>
  ```
  Example: `$ ./hang 5s`

- To hang until a specific time, use:
  ```
  $ ./hang <HH:MM:SS>
  ```
  Example: `$ ./hang 10:20:30`

- If no arguments are provided, the program will hang for 1 second.

## Duration Format
- The duration can be specified in the following formats:
  - `ns` for nanoseconds
  - `ms` for milliseconds
  - `s` for seconds
  - `m` for minutes
  - `h` for hours

