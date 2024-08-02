# chrono db
A Time series Database built on Rust to handle Petabyte Scale Data

**Introducing SALT**

- Scalability
- Adaptability
- Low Latency
- Throughput

By Integrating this principle, We aim to create a faster and cheaper database with latest technology that can meet the demand of the future applications.

# Goal

- Build the next gen fastest timeseries database
- Push the limits to 100+ Million Events / second on avg spec machine
- High Scalability & Throughput with minimum cost & latency
- Design to store huge data with millions of events concurrently
- True Low code development for more optimization & control
- Develop a simple protocol & storage engine

# Docs

### Usage
- ```cargo run init <db name> <keeper>``` for creating chrono db
- ```cargo run start <db name>``` for starting chrono db

- copy the creds printed & connect with the [golang driver](https://github.com/citra-org/chrono-db-go-driver) for using chrono db

If you get any issues, try with sudo/admin access, for further issues, reachout on the community server.

We will be soon releasing download support for linux systems.

### Info

- KEEPER is admin/user/database manager
- EVENTS are the time based records stored with header & body
- STREAM is collection of events, which can in traditional databased called table
- CHRONO is the database itself which is collection of streams


- Event Format ```TIME HEADER BODY```
- Data is being stored in ```/var/lib/citra/chrono```
- Credentials are being stored in ```~/.citra/chrono```

### ToDo

- Encryption & Decryption of file
- Read filters & queries
- Better Impl for data storage & streaming data
- Parser & Validation for write
- Concurrency & I/O Optimizations
- Impl of B'Trees & hash for files
- Impl of time based binary search for searching
- Indexer, Sharding, testing and much more

### Architecture

To be announced soon....

### Benchmark

To be announced soon....

### Package

To be published soon....

---

**THIS DATABASE IS STILL UNDER DEV, HAS LOT OF BUGS & IMPROVEMENTS TO BE DONE**

**FEEL FREE TO OPEN A ISSUES & GET YOUR PR MERGED**

---

### Contributors

**Creator**: Sugam Kuber ([Github](https://github.com/sugamkuber)) ([LinkedIn](https://linkedin.com/in/sugamkuber))
