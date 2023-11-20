set multiplot
set datafile separator ','
set grid
set key autotitle columnhead # use the first line as title

# Marketplace statistics
set title 'Marketplace statistics'
set xrange [0:]
set yrange [0:]
set size 1,0.25
set origin 0,0
plot for [i=2:7:1] 'data/training_performance.csv' using 0:i with lines


# Company values
set title 'Company values'
set xrange [0:]
set yrange [0:]
set size 1,0.25
set origin 0,0.25
plot for [i=8:*:3] 'data/training_performance.csv' using 0:i with lines

# Company processors
set title 'Company processors'
set xrange [0:]
set yrange [0:]
set size 1,0.25
set origin 0,0.5
plot for [i=9:*:3] 'data/training_performance.csv' using 0:i with lines

# Company processor ticks
set grid
set title 'Company processor productive ticks'
set xrange [0:]
set yrange [0:]
set size 1,0.25
set origin 0,0.75
plot for [i=10:*:3] 'data/training_performance.csv' using 0:i with lines
pause 1
reread
