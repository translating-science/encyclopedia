#!/bin/bash

if [[ -z "${POSTHOG_PROJECT}" ]]
then
    echo "Posthog environment variables not set. Continuing..."
else
    POSTHOG_URL=${POSTHOG_URL:-"https://app.posthog.com"}

    echo "Using POSTHOG_PROJECT=${POSTHOG_PROJECT} and POSTHOG_URL=${POSTHOG_URL}..."

    export POSTHOG_URL
    export POSTHOG_PROJECT
fi

categories=("genes" "genes/NF1")
endpoints=("index.html" "genes/index.html" "genes/NF1/index.html")
server="http://localhost:8081/"

# clean generated static site pages
rm -rf static_site
mkdir static_site
cp -r resources static_site/resources

for category in ${categories[*]}
do
    mkdir "static_site/${category}"
done

# run server in background
# need to build first, otherwise we get a timeout in github actions
cargo build
cargo run &
build_pid=$!

# sleep for 5 sec to let server start
sleep 5

for endpoint in ${endpoints[*]}
do
    curl "${server}${endpoint}" -o "static_site/${endpoint}" -s
done

# terminate server running in background
kill $build_pid
