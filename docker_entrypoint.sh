#!/bin/bash

set -ea

_term() { 
  echo "Caught SIGTERM signal!" 
  kill -TERM "$mvctest_process" 2>/dev/null
}

# This just sets up the environment from EmbassyOS config, I believe.
#configurator > .env 
#source .env

dotnet ./SampleMvcApp.dll &
mvctest_process=$!

trap _term SIGTERM

wait -n $mvctest_process