#!/bin/sh

if [ "$#" -gt 1 ]
    then

        if [ -d "$2" ]
            then
            [ -n "$3" ] && ignore="! -path */$3/*"
            ext=" -name *.$1"
            path="$2"

            files=$(/bin/find "$path" -type f $ext  $ignore -print)

            printf --  "Files:\t%s\n" "$(/bin/ls $files | /usr/bin/wc -l)"
            printf --  "Lines:\t%s\n\n" "$(/usr/bin/cat $files | /usr/bin/wc -l )"
            exit 0
        else
            printf -- "\nDirectory %s not found.\n\n" "$2"
        fi

else
    printf -- '\nUSAGE: %s <EXTENSION> <PATH_TO_DIR>\n\n' "$0"
    exit 1
fi