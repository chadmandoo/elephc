<?php
echo "iterator iterator:\n";
$wrapped = new IteratorIterator(new ArrayObject(["one" => 1, "two" => 2]), "ArrayObject");
foreach ($wrapped as $key => $value) {
    echo $key;
    echo "=";
    echo $value;
    echo "\n";
}

echo "limit over infinite:\n";
$cycle = new LimitIterator(new InfiniteIterator(new ArrayIterator(["A", "B"])), 0, 5);
foreach ($cycle as $value) {
    echo $value;
}
echo "\n";

echo "no rewind:\n";
$source = new ArrayIterator([10, 20, 30]);
$source->next();
$noRewind = new NoRewindIterator($source);
foreach ($noRewind as $value) {
    echo $value;
    echo " ";
}
echo "\n";

echo "seek:\n";
$limited = new LimitIterator(new ArrayIterator(["zero", "one", "two", "three"]), 1, 2);
$limited->seek(2);
echo $limited->key();
echo "=";
echo $limited->current();
echo "\n";

echo "append:\n";
$append = new AppendIterator();
$append->append(new ArrayIterator(["left" => "L"]));
$append->append(new ArrayIterator(["right" => "R"]));
foreach ($append as $key => $value) {
    echo $append->getIteratorIndex();
    echo ":";
    echo $key;
    echo "=";
    echo $value;
    echo "\n";
}

echo "multiple:\n";
$multi = new MultipleIterator(MultipleIterator::MIT_NEED_ANY | MultipleIterator::MIT_KEYS_ASSOC);
$multi->attachIterator(new ArrayIterator(["a" => 1, "b" => 2]), "left");
$multi->attachIterator(new ArrayIterator(["x" => 10]), "right");
foreach ($multi as $keys => $values) {
    echo $keys["left"];
    echo "/";
    echo is_null($keys["right"]) ? "null" : $keys["right"];
    echo "=";
    echo $values["left"];
    echo "/";
    echo is_null($values["right"]) ? "null" : $values["right"];
    echo "\n";
}
