#!/bin/bash
echo "Fixing feed locations"
for i in $(find public -name 'rss.xml'); do
    mv $i ${i%rss.xml}feed.xml
done
