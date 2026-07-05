<?php
require_once __DIR__ . '/_bundle.php';
use AIC\Components\Domain\HeadAsset;
use AIC\Components\Domain\HeadAssetType;
use AIC\Components\Domain\HeadAssetMode;
$assets = [
    new HeadAsset(HeadAssetType::Css, '/build/app.css'),
    new HeadAsset(HeadAssetType::Js, '/build/app.js', null, null, HeadAssetMode::Module),
    new HeadAsset(HeadAssetType::Js, '/build/defer.js', null, null, HeadAssetMode::Defer),
];
foreach ($assets as $a) { echo $a->dedupKey(), "\n"; }
