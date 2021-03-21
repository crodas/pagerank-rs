# Simple page rank

Very simple implementation of the [PageRank](http://ilpubs.stanford.edu:8090/422/1/1999-66.pdf) algorithm.

### Features

- Small footprint
- Easy to use API
- Fast
`
### Usage

```rust
let mut pr = Pagerank::<&str>::new();
pr.add_link("source", "target");
pr.add_link("source", "another target");
pr.calculate();

// print result (always sorted)

pr.nodes()
	.iter()
	.map(|node| println!("page {} with score {}", node.id(), node.score()))
	.for_each(drop);
```


### Built-in binary example

The repository has a built-in binary example which works with [WikiLinkGraphs](https://zenodo.org/record/2539424) dataset.


```
gzcat eswiki.wikilink_graph.2018-03-01.csv.gz| cargo run --release wikilink
```
