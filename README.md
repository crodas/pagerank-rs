# Simple page rank

Very simple implementation of the [PageRank](http://ilpubs.stanford.edu:8090/422/1/1999-66.pdf) algorithm.

### Features

- Small footprint
- Zero dependency
- Easy to use API
- Fast
`
### Usage

```rust
let mut pr = Pagerank::<&str>::new();
pr.add_edge("source", "target");
pr.add_edge("source", "another target");
pr.calculate();

// print result (always sorted)

pr.nodes()
	.iter()
	.map(|(node, score)| println!("page {} with score {}", node, score))
	.for_each(drop);
```


### Built-in binary example

The repository has a built-in binary example which works with [WikiLinkGraphs](https://zenodo.org/record/2539424) dataset.


```
gzcat eswiki.wikilink_graph.2018-03-01.csv.gz| cargo run --release wikilink
```
