# <center>XKV</center>

## BitCask
- #### Bitcask has only one writable file. Writable files are called active files and read-only files are called non-active:
![img.png](img.png)

- #### Bitcask files formatted:
![img_1.png](img_1.png)

- #### The index structure of memory:
![img_2.png](img_2.png)

- #### Query process:
![img_3.png](img_3.png)

### TODO!
1. Supports high-performance range query
2. Index persistence
3. wal (under consideration)
4. Multi thread safe write
5. File partition