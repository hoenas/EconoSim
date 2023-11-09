set datafile separator ','
plot 'data/training_performance.csv' using 1:2 with lines
pause 1
reread