set multiplot
set datafile separator ','
set grid
# Company values
set title 'Company values'
set xrange [0:]
set yrange [0:]
set size 1,0.33
set origin 0,0
plot for [i=2:*:3] 'data/training_performance.csv' using 0:i with lines title 'Company '.(i/3)
# Company processors
set title 'Company processors'
set xrange [0:]
set yrange [0:]
set size 1,0.33
set origin 0,0.33
plot for [i=3:*:3] 'data/training_performance.csv' using 0:i with lines title 'Company '.(i/3-1)
# Company processor ticks
set grid
set title 'Company processor productive ticks'
set xrange [0:]
set yrange [0:]
set size 1,0.33
set origin 0,0.66
plot for [i=4:*:3] 'data/training_performance.csv' using 0:i with lines title 'Company '.(i/3-1)
pause 1
reread
