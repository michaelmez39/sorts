# Sorting Analysis
## Introduction
In this analysis of three different sorting algorithms, were written and benchmarked. The chosen algorithms are merge sort, heap sort, and insertion sort. Each of these algorithms were used to sort an array of size 10000, 20000, 50000, and 100000. Each sort was repeated five times for each input size. The theoretical time complexities for the given sorting algorithms are given in the table below.

|Sort|Worst|Average|Best|
|---|---|---|---|
|Insertion|O(n^2)|O(n^2)|O(n)|
|Heap|O(nlogn)|O(nlogn)|O(nlogn)|
|Merge|O(nlogn)|O(nlogn)|O(nlogn)|

## Results
### Insertion Sort
![insertion_time4.png](:/b40c96b466c44a1990a1eba9a750df91)
![insertion_sorted_time.png](:/68fce13b0ba84b8a9bf070ea8f57d108)

Although the performance of insertion sort is acceptable for the 10,000 and 20,000 element arrays, both the reversed and random performance both show a polynomial increase in time consistent with it's theoretical time complexity. The ordered case however has an O(n) time complexity.

### Merge Sort
![merge_time.png](:/0d76f78089724557808c4d1aa0733e5c)
The results for merge sort appear to be almost linear, but still increasing slightly. This result is consistent with with a time complexity of O(nlogn). Random, ordered, and reversed data all have a similar performance, but 
### Heap Sort
![heap_time.png](:/fc631cd47b9e46099f9096a20054b0d7)

## Comparison of Chosen Sorts
Of the sorts in this comparison, heap sort had the best performance for the random and reversed data, being approximately five times faster than merge sort, even though they had the same time complexity. This difference in performance could be from merge sort using recursion, from having to allocate memory, or some other unknown factor.

For sorted data, insertion sort outperformed both merge and heap sort. This is because insertion sort will only do n comparisons for sorted data. From the insertion sort graph that shows sorted array performance, we can see that it has a linear time increase for an increasing number of elements.

An important difference between heap sort, and the other two sorts is that heap sort is an unstable sorting algorithm, so two elements with the same value may not appear in the same order as in the original array. This is mainly important for applications that involve sorting keys. 

## Honorable Mentions 
These are a few sorting algorithms that were implemented. They were too slow to do five iterations for each size array, so they only had one iteration each size.



### Selection Sort

### Parallel Odd Even Sort
![odd_even_time2.png](:/630efe838908453784e35870c6f48f50)
Odd even sort is a sort that first looks at all the even indexes of an array, and compares it to the odd index next to it, it swaps them if they are out of order. It than looks at all the odd indexes, compares it with the next element, and swaps them if they are out of order. If there are no swaps, the algorithm exits. Odd even sort has a time complexity of O(n^2). This algorithm is easy to make parallel because all of the comparisons and swapping can be done independently of each other. It is similar to bubble sort, because they both compare and swap side by side elements. Although it was parallelized, it still had very poor performance. (I wanted to try parallel merge sort, but that was really hard)

#### Bubble Sort
Frightening
