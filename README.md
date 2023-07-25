# tee2

**Under development!!! and in WIP stage. please do not use yet**


An alternative to the populate linux [tee](https://www.gnu.org/software/coreutils/manual/html_node/tee-invocation.html) program.

The biggest difference is log rotation. The gnu tee command will happily fill your disks, and this is normally not wanted. 


## Description

This application reads from standard input and writes to a file, the same data is written to standard output.
When the file reaches a certain limit it is truncated or rotated.


File rotation will happen by default as:

e.g `command | tee2 myfile.log`

 1. myfile.log is renamed to myfile_<iso_timestamp>.log
 2. a new file is created as myfile.log
 3. if the log-file-count is 3 when we have `myfile_{i}.log, myfile_{i=1}.log, myfile_{i+2}.log`, `myfile_{i}.log` is deleted





## Objectives

* Log rotation
* Log truncation
* Reasonably performant.


## Non Goals
 * Fully compatible with tee. This application does not strive to be fully compatible with tee. It would be nice, but is not a goal.