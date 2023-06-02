# The Linux Process Manager (or lpm)
is a rust-based command-line interface task manager implemented for linux based systems. It displays the process table with attributes, overview of the system, sorting and filtering, updating, as well as options to kill, continue, terminate, and sleep processes. Implemented with the ```cursive``` crates for a *"frontend"*, and the ```sysinfo``` and ```procfs``` crates for a *"backend"*, this program offers a gui flavor to a command line task manager which offers all high-end features in a simplistic, easy to navigate manner. Click [here](https://github.com/saraa-mohamedd/lpm-GUI) for the GUI portion of this project.

![](https://github.com/saraa-mohamedd/The-Linux-Process-Manager/blob/main/lpm-screenrec.gif)

## To Run

```cd``` into the project directory\
run ```cargo install``` to download dependencies\
run ```cargo build``` to build the program\
run ```cargo install --path``` to download the binary executable file

#### From any terminal on your Linux based system

run ```lpm -h``` to view all running commands and options


