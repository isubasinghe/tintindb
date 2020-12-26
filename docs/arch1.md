---
id: arch1
title: High Level Architecture
sidebar_label: Overview
---

## Overview

The aim of this project is to be extremely fast at queries at the cost of reindexing speed. 
Ideally we would like to still be performant with reindexes, but this is not the priority.


### Features
* Faceting
* Boosting of query (for Titles etc)
* Stemming
* Regex support
* Boolean queries

### Optimising write speeds
Most important thing to do is to cache the data structure for the FST. We really dont want 
to sort everything to rebuild the FST. 

#### Things to Cache

These are coming directly from the equation for BM25 scoring. 

![BM25](/img/bm25.png)

Where the inverse document frequency is given by 

![IDF](/img/idf.png)

From these two equations, what we need to cache/materialize should be fairly obvious.

* Average document length
* The documents in which a certain word appears in