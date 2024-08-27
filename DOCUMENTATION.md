NAME
    fsrenamer - file system renamer


DESCRIPTION
    Simple tool for refactoring filenames in an automated way.
    Removes all nonstandart, illegal characters and whitespaces from names of directories and files.

    -f 
        affects files only (non stackable with -d)
    -d 
        affects directory only (non stackable with -f)
    -r={u8}
        number of recursions (default 0 stays only in passed PATH, 255 means unlimited)
    -c 
        enables copying instead of renaming
    -D 
        disables replacing common diacritics with their ASCII version
    -s 
        supress error messages
    -l 
        enables log
    -I 
        do not remove illegal characters
    -A
        do not remove nonascii
    -a
        affect files and dirs starting with .
        (start directory/file is refactored even if it starts with . and this is not used)
    -R={char}
        set char with which should be invalid chars replaced (default None)

    --help 
        display this help and exit
    --version
        output version information and exit


EXAMPLES
    fsrenamer "/to_refactor" -d -r=2 -R=x


WARNING
    Work in progress, bugs expected,
    DO NOT USE ON IMPORTANT FILES WITHOUT BACKUP!


AUTHOR
    Written by Michal Friml


REPORTING BUGS
    Please report any problems to my github repository at https://github.com/MichalFriml/FileSystemNameRefactor