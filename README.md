# fizzy

`fizzy` is a tool for reading and modifying disk images.

## Commands
The following commands are supported. Arguments in [] are optional. Arguments in {} are required.

## info
Displays info about a disk or partition

`fiz info {image} [partition]`
 
## ls
Lists files in a directory on a disk or partition

`fiz ls {image} [partition]/{path}`

## tree
Displays a directory tree on a disk or partition

`fiz tree {image} [partition]/{path}`

## cat
Displays the contents of a file on a disk or partition

`fiz cat {image} [partition]/{path}`

## cp
Copies a file from one location to another on a disk(s) or partition(s)

`fiz cp {image} [partition]/{path} {image} [partition]/{path}`

## rm
Removes a file from a disk or partition

`fiz rm {image} [partition]/{path}`

## mv
Moves a file from one location to another on a disk(s) or partition(s)

`fiz mv {image} [partition]/{path} {image} [partition]/{path}`

## format
Formats a disk or partition

`fiz format {fs} {image} [partition]`

## boot
Installs a bootloader to a disk or partition

`fiz boot boot.img {image} [partition]`

## create
Creates a disk image

`fiz create {image}`

## partition
Partitions a disk image

`fiz partition {image}`
 
## get
Copies a file from a disk image to the local filesystem

`fiz get {image} [partition]/{path} {path}`

## put
Copies a file from the local filesystem to a disk image

`fiz put {path} {image} [partition]/{path}`

## sys
Copies DOS system files to a disk or partition

`fiz sys {file 1},{file 2},{file n} {image} [partition]`
