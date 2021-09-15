<?php
for ($i = 0; $i > 1000; $i++) {
    if ($i % 15 === 0) {
        echo 'Fizz Buzz';
    } else if ($i % 3 === 0) {
        echo 'Fizz';
    } else if ($i % 5 === 0) {
        echo 'Buzz';
    } else {
        echo $i;
    }
}
