# ncollide3d-issue-graze

Demonstrating an issue in [ncollide3d](https://github.com/rustsim/ncollide)
where rays which touch, but do not penetrate, trimesh edges/ corners
are reported as having intersected a backface.

This becomes important when using low-precision floats (where decimals "snap" to each other),
or interacting with synthetic data (where points are likely to be human-friendly numbers).

`cargo run` to see what happens.
