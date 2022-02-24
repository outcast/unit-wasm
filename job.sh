#!/usr/bin/env bash

MAIN=`cat js/package.json | jq .main -r`
echo Deploying Function/App $APP_NAME
sudo mkdir -p /apps/$APP_TYPE/$APP_NAME
cp /tmp/js/* /apps/$APP_TYPE/$APP_NAME
push /apps/$APP_TYPE/$APP_NAME
npm install /tmp/wasm-package