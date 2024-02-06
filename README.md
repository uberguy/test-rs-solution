# Zelis Code Test (Rust)
---
You are given a large JSON file, ````data/test.json````. The file has been delivered by the client corrupt and is unable to be processed normally. After some inspection, you realize that at least one ````:```` has been mistyped as ````;```` within the file. The file must be processed by the end of the day and it is not feasible for the client to correct. Write a program that solves this problem.

- The program should take input from STDIN and write output to STDOUT.
- Optionally, the program can take ````--input```` and ````--output```` parameters, specifying file locations.
- Correct output will be valid JSON.
- Given a known schema, you can be confident that ````;```` does not exist within any valid input. As such, it's safe to simply replace them without concern for having accidentally replaced a valid character.
- This is a common problem that we've seen before and we will need to be able to use the program again on files much larger than the example.
- The primary goal is performance. The process will have to be run frequently for years to come, so it is important that it be as efficient as possible. It also needs to finish in time on short notice.

Create a new repository implementing your solution. When complete, share with ````@jgmartin````, ````@MrLeebo```` and ````@ealders```` for review. Thanks and good luck!

---

## Implementation Notes

I think I've coded a solution that meets the criteria outlined above, as I understand them.
- The only output is corrected JSON.
- Given there's has only 1 circumstance in which `;` in JSON can be replaced by `:` to make it valid, I felt that the schema of the data was more-or-less irrelevant.
  - The only circumstance in which a `:` is used in JSON is to delimit a key from a value (otherwise a `:` can only appear as just another character in a string).
- Not much thought was given to finding ways to make the code more efficient.
  - The assumption is that the solution will be completely IO-bound and anything like partitioning tasks for concurrent work would not make much of a difference.
- In the spirit of efficiency and minimal resource consumption, I chose to use a streaming approach that would process the data in chunks of N bytes. 

### Approaches Taken
1. My first thought was to dig in to the bowels of a JSON parsing library to get some help, but my experience with such libraries in Java and Go led me to believe that the ROI on that effort wouldn't be worth it, given the axiom that simply replacing `;` with `:` under the right circumstances would be enough.
2. My second thought was to use regular expressions to capture a JSON key followed by a `;`, but I ultimately rejected that approach because it would involve additional work (probably regular expressions) around identifying matches that spanned the buffer boundary being used to consume the data. For example, if a JSON key started near the end of the buffer but was completed near the beginning of the next buffer of content, I would need to keep track of this between buffer loads.
3. My final thought was to simply iterate over every character and identify the `;` that immediately followed a JSON key. This wasn't difficult and didn't really need much work in keeping track of what had been seen so far.

### Problems

While cloning this repo I got a 404 error related to the link to the large data set. Consequently I haven't tested this with a very large file. However, I included some test files that can be run with `cargo test`, and even thought they're small I think they exercise all reasonable instances of how this data can be corrected.

```
Downloading data/test.json (1.0 GB)
Error downloading object: data/test.json (0024082): Smudge error: Error downloading data/test.json (002408257cb09e8dc48c01becb4eda36376a0152057d21ea87b922965710cb24): [002408257cb09e8dc48c01becb4eda36376a0152057d21ea87b922965710cb24] Object does not exist on the server: [404] Object does not exist on the server

Errors logged to '~/rust/test-rs/.git/lfs/logs/20240204T170808.00933.log'.
Use `git lfs logs last` to view the log.
error: external filter 'git-lfs filter-process' failed
fatal: data/test.json: smudge filter lfs failed
```
