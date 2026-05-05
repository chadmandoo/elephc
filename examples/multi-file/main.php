<?php
// Multi-file example: include functions from other files, including through a
// loader function.

function load_libraries() {
    require_once 'math.php';
    require_once 'greet.php';
}

load_libraries();

for ($i = 0; $i < 2; $i = $i + 1) {
    require_once 'bootstrap.php';
}

hello("World");

echo "3 + 4 = " . add(3, 4) . "\n";
echo "5 * 6 = " . multiply(5, 6) . "\n";
echo "10! = " . factorial(10) . "\n";
