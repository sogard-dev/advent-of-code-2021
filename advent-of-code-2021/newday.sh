#!/bin/bash

for i in $(seq 1 25);
do
    day=day$i

    if [ ! -d $day ]; then
        echo "Creating $day"
        cp -r "day_template" $day
        var1=day_template
        sed -i -e "s/$var1/$day/g" "$day/task_test.go"
        sed -i -e "s/$var1/$day/g" "$day/task.go"
    fi
done

