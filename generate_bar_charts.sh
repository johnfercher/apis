#!/bin/bash

version=$1
gnuplot -e "set terminal png; set yrange [0:*]; set boxwidth 0.5; set style fill solid; set output 'docs/${version}/images/files_quantity.png'; plot 'docs/${version}/data/files_quantity.dat' using 2:xtic(1) with boxes notitle;"
gnuplot -e "set terminal png; set yrange [0:*]; set boxwidth 0.5; set style fill solid; set output 'docs/${version}/images/lines_quantity.png'; plot 'docs/${version}/data/lines_quantity.dat' using 2:xtic(1) with boxes notitle;"