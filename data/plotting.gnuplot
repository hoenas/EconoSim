set datafile separator ','
set grid
set xrange [0:]
set yrange [0:]
plot 'data/training_performance.csv' using 1:2 with lines
pause 0.1
reread