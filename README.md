# wave_challenge

Assumptions I had for this challenge:

**1.** For a Quadrilateral surface to be considered 'wet', it would have at least one coordinate where the z-index was below 0. <br>

   To clarify: Even if 3 coordinate points were 'dry' and one is 'wet', that Quadrilateral surface falls into the 'wet' data set.
   
**2.** Quadrilateral surfaces may be touching and therefore may share points.

## Usage
Run 
```
cargo run
```
in the terminal to run the program.

## Documentation

For the documentation, please run 

```
cargo doc --open
```

in the terminal to open the documentation.

## Output

The outputs created from the project are 

* [wet_data.json](https://github.com/shuhsuan/wave_challenge/blob/master/wet_data.json)
  This contains a data set of quadrilaterals that map to coordinates with a positive z-index.
  
* [dry_data.json](https://github.com/shuhsuan/wave_challenge/blob/master/dry_data.json)
  This contains a data set of quadrilaterals that map to coordinates with a negative z-index.


