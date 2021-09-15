#!/bin/bash

version=$1

cs="$(cloc netcoreapi --exclude-dir=bin,obj,Migrations --include-ext=cs --csv | tail -n 1 | cut -d',' -f5)"
csm="$(cloc Netcoreapi-Minimal --exclude-dir=bin,obj,Migrations --include-ext=cs --csv | tail -n 1 | cut -d',' -f5)"
rs="$(cloc rustapi --exclude-dir=target --include-ext=rs --csv | tail -n 1 | cut -d',' -f5)"
py="$(cloc pythondjango --exclude-dir migrations --include-ext=py --csv | tail -n 1 | cut -d',' -f5)"

printf "C# $cs\nC#(Minimal) $csm\nRust $rs\nPython $py" > "docs/$version/data/lines_quantity.dat"