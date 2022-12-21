# Problem 06: Broken Radio

Communication device is broken.  Need to get it working to communicate with the
elfs.

## Part One

Find the start of the data stream.  The start packet is the first four unique
ascii characters.  Need to find the index where the data actually starts, so
start packet index + 4.

### Samples

```BASH
mjqjpqmgbljsphdztnvjfqwrcgsmlb
```

In this sample the start packet is `jpqm` which means the datastream starts at
index *7*.

```BASH
bvwbjplbgvbhsrlpgdmjqwftvncz
nppdvjthqldpwncqszvftbrmjlhg
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw
```

1. Data starts at *5*
2. Data starts at *6*
3. Data starts at *10*
4. Data starts at *11*


#### Sample File Lookup

|File Name|Index|
|---------|-----|
|sample1.txt|7|
|sample2.txt|5|
|sample3.txt|6|
|sample4.txt|10|
|sample5.txt|11|