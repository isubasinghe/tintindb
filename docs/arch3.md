---
id: arch3
title: High Level Architecture
sidebar_label: Storage
---

## Storage

### Metadata storage
We use RocksDB for metadata storage. 
High level constructs are achieved through key decomposition [0]



#### Resources 
* [0] https://blog.yugabyte.com/how-we-built-a-high-performance-document-store-on-rocksdb/